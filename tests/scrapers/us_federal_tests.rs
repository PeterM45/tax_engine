use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use tax_engine::{Country, Jurisdiction, TaxEntityType, TaxRateScraper, USFederalScraper};
use tokio;

#[tokio::test]
async fn test_live_fetch() {
    let scraper = USFederalScraper::new();
    let result = scraper
        .fetch_rates(
            &Jurisdiction::Federal(Country::USA),
            &TaxEntityType::Individual,
            2024,
        )
        .await;

    match result {
        Ok(schedule) => {
            println!("\nFetched Tax Brackets for 2024:");
            println!("======================");
            // Sort brackets by rate for clearer output
            let mut brackets = schedule.brackets;
            brackets.sort_by(|a, b| a.rate.cmp(&b.rate));

            for bracket in &brackets {
                println!(
                    "Rate: {:>5}%, Range: ${:>10} to {}",
                    (bracket.rate * Decimal::from(100)).round(),
                    bracket.lower_bound,
                    bracket
                        .upper_bound
                        .map_or("No limit".to_string(), |u| format!("${}", u))
                );
            }
            assert!(!brackets.is_empty(), "Should have found tax brackets");
            assert_eq!(brackets[0].rate, dec!(0.10), "First bracket should be 10%");
        }
        Err(e) => {
            println!("\nError fetching tax rates: {:?}", e);
            println!(
                "This might be normal if IRS website is unavailable or has changed structure."
            );
        }
    }
}
