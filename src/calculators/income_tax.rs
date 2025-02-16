use crate::errors::TaxError;
use crate::models::{TaxEntity, TaxSchedule};
use rust_decimal::Decimal;

pub struct IncomeTaxCalculator;

impl IncomeTaxCalculator {
    pub fn calculate_tax(entity: &TaxEntity, schedule: &TaxSchedule) -> Result<Decimal, TaxError> {
        if entity.tax_year != schedule.tax_year {
            return Err(TaxError::YearMismatch);
        }

        let taxable_income = entity.taxable_income();
        let mut total_tax = Decimal::ZERO;
        let mut remaining_income = taxable_income;

        for bracket in &schedule.brackets {
            let bracket_income = match bracket.upper_bound {
                Some(upper) => {
                    if remaining_income <= Decimal::ZERO {
                        break;
                    }
                    remaining_income.min(upper - bracket.lower_bound)
                }
                None => remaining_income,
            };

            if bracket_income > Decimal::ZERO {
                total_tax += bracket_income * bracket.rate;
                remaining_income -= bracket_income;
            }
        }

        Ok(total_tax)
    }
}
