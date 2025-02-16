//! US Federal tax rate scraping implementation.
//!
//! Provides functionality to fetch and parse US federal tax rates from the IRS website.
//! Handles various IRS website formats and patterns for tax bracket information.

use super::TaxRateScraper;
use crate::errors::TaxError;
use crate::models::{Country, Jurisdiction, TaxBracket, TaxEntityType, TaxSchedule};
use async_trait::async_trait;
use regex::Regex;
use rust_decimal::prelude::*;
use scraper::{Html, Selector};

/// Scraper implementation for US federal tax rates.
pub struct USFederalScraper {
    client: reqwest::Client,
}

impl USFederalScraper {
    /// Creates a new USFederalScraper instance with a configured HTTP client.
    ///
    /// The client is configured with:
    /// - A realistic browser user agent
    /// - 10-second timeout
    /// - Fallback to default client if custom configuration fails
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new())
        }
    }

    /// Attempts to fetch tax rate information from various IRS website URLs.
    ///
    /// Tries multiple URL patterns in sequence, as the IRS website structure
    /// can vary by tax year. Debug information is printed to help diagnose
    /// fetching issues.
    ///
    /// # Arguments
    ///
    /// * `year` - The tax year to fetch rates for
    ///
    /// # Returns
    ///
    /// The HTML content of the first successfully fetched page, or an error
    /// if all URLs fail.
    async fn fetch_rates_from_irs(&self, year: u16) -> Result<String, TaxError> {
        let urls = vec![
            format!("https://www.irs.gov/newsroom/irs-provides-tax-inflation-adjustments-for-tax-year-{}", year),
            format!("https://www.irs.gov/pub/irs-drop/rp-{}-23.pdf", year - 1),
            format!("https://www.irs.gov/newsroom/tax-year-{}-inflation-adjustments", year),
        ];

        let mut last_error = String::new();
        for url in &urls {
            println!("Trying URL: {}", url);
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let text = response
                            .text()
                            .await
                            .map_err(|e| TaxError::FetchError(e.to_string()))?;
                        println!("Successfully fetched content from: {}", url);
                        println!(
                            "First 500 chars of content: {}",
                            &text[..500.min(text.len())]
                        );
                        return Ok(text);
                    }
                    println!("Status not success: {}", response.status());
                }
                Err(e) => {
                    last_error = e.to_string();
                    println!("Error fetching {}: {}", url, last_error);
                }
            }
        }

        Err(TaxError::FetchError(format!(
            "Failed to fetch IRS data: {}",
            last_error
        )))
    }

    /// Parses tax brackets from IRS website content.
    ///
    /// Searches for specific text patterns that indicate tax bracket information
    /// and constructs TaxBracket instances from the parsed data.
    ///
    /// # Arguments
    ///
    /// * `content` - The HTML content from the IRS website
    /// * `year` - The tax year (used for validation)
    ///
    /// # Returns
    ///
    /// A vector of parsed tax brackets, sorted by lower bound,
    /// or an error if no valid brackets are found.
    fn parse_tax_brackets(&self, content: &str, _year: u16) -> Result<Vec<TaxBracket>, TaxError> {
        let document = Html::parse_document(content);
        let mut brackets = Vec::new();

        for element in document.select(&Selector::parse("p,div").unwrap()) {
            let text = element.text().collect::<String>().to_lowercase();

            if text.contains("% for incomes over") {
                if let Some(bracket) = self.parse_rate_text(&text) {
                    brackets.push(bracket);
                }
            }

            if text.contains("lowest rate is") && text.contains("or less") {
                if let Some(bracket) = self.parse_lowest_rate_text(&text) {
                    brackets.push(bracket);
                }
            }
        }

        if !brackets.is_empty() {
            brackets.sort_by(|a, b| a.lower_bound.cmp(&b.lower_bound));
            return Ok(brackets);
        }

        Err(TaxError::ParseError(
            "Could not find tax bracket information".to_string(),
        ))
    }

    /// Parses a text fragment containing a standard tax bracket definition.
    ///
    /// Handles patterns like "35% for incomes over $243,725"
    ///
    /// # Arguments
    ///
    /// * `text` - The text fragment to parse
    ///
    /// # Returns
    ///
    /// An Option containing a TaxBracket if the text matches the expected pattern
    fn parse_rate_text(&self, text: &str) -> Option<TaxBracket> {
        let re = Regex::new(r"(\d+)%\s+for\s+incomes\s+over\s+\$([0-9,]+)").ok()?;
        if let Some(caps) = re.captures(text) {
            let rate = caps.get(1)?.as_str().parse::<u32>().ok()? as f64 / 100.0;
            let lower_bound = self.extract_number(caps.get(2)?.as_str())?;

            return Some(TaxBracket {
                rate: Decimal::from_f64(rate)?,
                lower_bound,
                upper_bound: None,
            });
        }
        None
    }

    /// Parses a text fragment containing the lowest tax bracket definition.
    ///
    /// Handles patterns like "lowest rate is 10% for incomes of single individuals
    /// with incomes of $11,600 or less"
    ///
    /// # Arguments
    ///
    /// * `text` - The text fragment to parse
    ///
    /// # Returns
    ///
    /// An Option containing a TaxBracket if the text matches the expected pattern
    fn parse_lowest_rate_text(&self, text: &str) -> Option<TaxBracket> {
        let re = Regex::new(r"(\d+)%.*\$([0-9,]+)\s+or\s+less").ok()?;
        if let Some(caps) = re.captures(text) {
            let rate = caps.get(1)?.as_str().parse::<u32>().ok()? as f64 / 100.0;
            let upper_bound = self.extract_number(caps.get(2)?.as_str())?;

            return Some(TaxBracket {
                rate: Decimal::from_f64(rate)?,
                lower_bound: Decimal::zero(),
                upper_bound: Some(upper_bound),
            });
        }
        None
    }

    /// Extracts a decimal number from a string containing currency formatting.
    ///
    /// Removes currency symbols, commas, and spaces, then parses the result
    /// as a decimal number.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse
    ///
    /// # Returns
    ///
    /// An Option containing the parsed Decimal if successful
    fn extract_number(&self, s: &str) -> Option<Decimal> {
        let cleaned = s.trim().replace('$', "").replace(',', "").replace(" ", "");

        if cleaned.chars().any(|c| c.is_numeric()) {
            Decimal::from_str_exact(&cleaned).ok()
        } else {
            None
        }
    }
}

#[async_trait]
impl TaxRateScraper for USFederalScraper {
    /// Fetches and parses US federal tax rates for a given year.
    ///
    /// # Arguments
    ///
    /// * `jurisdiction` - Must be Federal(USA)
    /// * `entity_type` - Must be Individual
    /// * `tax_year` - The tax year to fetch rates for
    ///
    /// # Returns
    ///
    /// A TaxSchedule containing the parsed brackets, or an error if:
    /// - The jurisdiction/entity type combination is not supported
    /// - The IRS website cannot be accessed
    /// - The tax bracket information cannot be parsed
    /// - No brackets are found for the specified year
    async fn fetch_rates(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
    ) -> Result<TaxSchedule, TaxError> {
        match (jurisdiction, entity_type) {
            (Jurisdiction::Federal(Country::USA), TaxEntityType::Individual) => {
                let content = self.fetch_rates_from_irs(tax_year).await?;
                let brackets = self.parse_tax_brackets(&content, tax_year)?;

                if brackets.is_empty() {
                    return Err(TaxError::RateNotAvailable(tax_year));
                }

                Ok(TaxSchedule::new(tax_year, brackets))
            }
            _ => Err(TaxError::UnsupportedJurisdiction),
        }
    }

    /// Checks if this scraper supports the given jurisdiction.
    ///
    /// Currently only supports US Federal jurisdiction.
    fn supports_jurisdiction(&self, jurisdiction: &Jurisdiction) -> bool {
        matches!(jurisdiction, Jurisdiction::Federal(Country::USA))
    }
}
