//! Implements core income tax calculation logic.
//!
//! This module provides the main calculator for determining income tax
//! based on tax brackets and entity information.

use crate::errors::TaxError;
use crate::models::{TaxEntity, TaxSchedule};
use rust_decimal::Decimal;

/// Calculator for determining income tax based on progressive tax brackets.
pub struct IncomeTaxCalculator;

impl IncomeTaxCalculator {
    /// Calculates the total tax for an entity based on a tax schedule.
    ///
    /// # Arguments
    ///
    /// * `entity` - The tax entity whose tax should be calculated
    /// * `schedule` - The tax schedule containing applicable tax brackets
    ///
    /// # Returns
    ///
    /// The calculated tax amount or an error if calculation fails.
    ///
    /// # Errors
    ///
    /// Returns `TaxError::YearMismatch` if the entity's tax year doesn't match
    /// the schedule's tax year.
    ///
    /// # Examples
    ///
    /// ```
    /// use tax_engine::{IncomeTaxCalculator, TaxEntity, TaxSchedule};
    /// # use rust_decimal_macros::dec;
    /// # use tax_engine::TaxEntityType;
    ///
    /// # let entity = TaxEntity::new(TaxEntityType::Individual, dec!(100000), 2024);
    /// # let schedule = TaxSchedule::new(2024, vec![]);
    /// let tax = IncomeTaxCalculator::calculate_tax(&entity, &schedule);
    /// ```
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
