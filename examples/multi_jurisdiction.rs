use rust_decimal_macros::dec;
use tax_engine::{
    format_currency, Country, DeductionType, IncomeTaxCalculator, Jurisdiction, MemoryCache,
    TaxEntity, TaxEntityType, TaxRateScraper, USFederalScraper,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a cache instance
    let cache = MemoryCache::new(std::time::Duration::from_secs(3600));

    // Create a tax entity with multiple jurisdiction income
    let mut entity = TaxEntity::new(
        TaxEntityType::Individual,
        dec!(100000), // Total income
        2024,
    );

    entity.add_deduction(dec!(12950), DeductionType::Personal);

    // Create scrapers
    let us_federal_scraper = USFederalScraper::new();

    // Fetch US Federal tax schedule
    let us_schedule = us_federal_scraper
        .fetch_rates(
            &Jurisdiction::Federal(Country::USA),
            &entity.entity_type,
            entity.tax_year,
        )
        .await?;

    // Calculate federal tax
    let federal_tax = IncomeTaxCalculator::calculate_tax(&entity, &us_schedule)?;

    println!("Federal tax: {}", format_currency(federal_tax));

    Ok(())
}
