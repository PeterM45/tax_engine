use rust_decimal_macros::dec;
use tax_engine::{
    DeductionType, IncomeTaxCalculator, TaxBracket, TaxEntity, TaxEntityType, TaxSchedule,
};

fn main() {
    // Simple example that doesn't require online functionality
    let mut entity = TaxEntity::new(TaxEntityType::Individual, dec!(50000), 2024);

    entity.add_deduction(dec!(5000), DeductionType::Personal);

    let schedule = TaxSchedule::new(
        2024,
        vec![TaxBracket {
            lower_bound: dec!(0),
            upper_bound: Some(dec!(50000)),
            rate: dec!(0.15),
        }],
    );

    match IncomeTaxCalculator::calculate_tax(&entity, &schedule) {
        Ok(tax) => println!("Tax calculated: ${:.2}", tax),
        Err(e) => eprintln!("Error calculating tax: {}", e),
    }
}
