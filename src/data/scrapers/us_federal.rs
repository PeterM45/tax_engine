use super::TaxRateScraper;
use crate::errors::TaxError;
use crate::models::{Country, Jurisdiction, TaxBracket, TaxEntityType, TaxSchedule};
use async_trait::async_trait;
use regex::Regex;
use rust_decimal::prelude::*;
use scraper::{Html, Selector};

pub struct USFederalScraper {
    client: reqwest::Client,
}

impl USFederalScraper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new())
        }
    }

    async fn fetch_rates_from_irs(&self, year: u16) -> Result<String, TaxError> {
        // Updated URLs based on IRS website structure
        let urls = vec![
            // Main newsroom announcement
            format!("https://www.irs.gov/newsroom/irs-provides-tax-inflation-adjustments-for-tax-year-{}", year),
            // Backup URL pattern
            format!("https://www.irs.gov/pub/irs-drop/rp-{}-23.pdf", year - 1),
            // Alternative format sometimes used
            format!("https://www.irs.gov/newsroom/tax-year-{}-inflation-adjustments", year),
        ];

        let mut last_error = String::new();
        for url in &urls {
            println!("Trying URL: {}", url); // Debug print
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let text = response
                            .text()
                            .await
                            .map_err(|e| TaxError::FetchError(e.to_string()))?;
                        println!("Successfully fetched content from: {}", url); // Debug print
                        println!(
                            "First 500 chars of content: {}",
                            &text[..500.min(text.len())]
                        ); // Debug print
                        return Ok(text);
                    }
                    println!("Status not success: {}", response.status()); // Debug print
                }
                Err(e) => {
                    last_error = e.to_string();
                    println!("Error fetching {}: {}", url, last_error); // Debug print
                }
            }
        }

        Err(TaxError::FetchError(format!(
            "Failed to fetch IRS data: {}",
            last_error
        )))
    }

    fn parse_tax_brackets(&self, content: &str, year: u16) -> Result<Vec<TaxBracket>, TaxError> {
        let document = Html::parse_document(content);
        let mut brackets = Vec::new();

        // First try to find the direct text containing rates
        for element in document.select(&Selector::parse("p,div").unwrap()) {
            let text = element.text().collect::<String>().to_lowercase();

            // Look for the specific pattern we found in the IRS website
            if text.contains("% for incomes over") {
                if let Some(bracket) = self.parse_rate_text(&text) {
                    brackets.push(bracket);
                }
            }

            // Also catch the lowest rate which is formatted differently
            if text.contains("lowest rate is") && text.contains("or less") {
                if let Some(bracket) = self.parse_lowest_rate_text(&text) {
                    brackets.push(bracket);
                }
            }
        }

        if !brackets.is_empty() {
            // Sort brackets by lower bound
            brackets.sort_by(|a, b| a.lower_bound.cmp(&b.lower_bound));
            return Ok(brackets);
        }

        Err(TaxError::ParseError(
            "Could not find tax bracket information".to_string(),
        ))
    }

    fn parse_rate_text(&self, text: &str) -> Option<TaxBracket> {
        // Pattern: "35% for incomes over $243,725"
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

    fn parse_lowest_rate_text(&self, text: &str) -> Option<TaxBracket> {
        // Pattern: "lowest rate is 10% for incomes of single individuals with incomes of $11,600 or less"
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

    fn supports_jurisdiction(&self, jurisdiction: &Jurisdiction) -> bool {
        matches!(jurisdiction, Jurisdiction::Federal(Country::USA))
    }
}
