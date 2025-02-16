use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal_macros::dec;
use tax_engine::*;

fn tax_calculation_benchmark(c: &mut Criterion) {
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

    c.bench_function("calculate tax for 100k income", |b| {
        b.iter(|| IncomeTaxCalculator::calculate_tax(black_box(&entity), black_box(&schedule)))
    });
}

criterion_group!(benches, tax_calculation_benchmark);
criterion_main!(benches);
