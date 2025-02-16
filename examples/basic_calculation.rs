use rust_decimal_macros::dec;
use tax_engine::{
    format_currency, Country, DeductionType, IncomeTaxCalculator, Jurisdiction, TaxEntity,
    TaxEntityType, TaxRateScraper, USFederalScraper,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scraper = USFederalScraper::new();

    let mut entity = TaxEntity::new(TaxEntityType::Individual, dec!(75000), 2024);

    entity.add_deduction(dec!(12950), DeductionType::Personal);

    let schedule = scraper
        .fetch_rates(
            &Jurisdiction::Federal(Country::USA),
            &entity.entity_type,
            entity.tax_year,
        )
        .await?;

    let tax = IncomeTaxCalculator::calculate_tax(&entity, &schedule)?;
    println!("Calculated tax: {}", format_currency(tax));

    Ok(())
}
