use rust_decimal_macros::dec;
use tax_engine::{
    DeductionType, IncomeTaxCalculator, TaxBracket, TaxEntity, TaxEntityType, TaxSchedule,
};

#[test]
fn test_basic_tax_calculation() {
    let mut entity = TaxEntity::new(TaxEntityType::Individual, dec!(100000), 2024);

    entity.add_deduction(dec!(10000), DeductionType::Personal);

    let schedule = TaxSchedule::new(
        2024,
        vec![
            TaxBracket {
                lower_bound: dec!(0),
                upper_bound: Some(dec!(50000)),
                rate: dec!(0.15),
            },
            TaxBracket {
                lower_bound: dec!(50000),
                upper_bound: None,
                rate: dec!(0.25),
            },
        ],
    );

    let tax = IncomeTaxCalculator::calculate_tax(&entity, &schedule).unwrap();
    assert_eq!(tax, dec!(17500));
}
