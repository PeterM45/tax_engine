//! Defines tax bracket structures and schedules.
//!
//! This module provides the core structures for representing tax brackets
//! and organizing them into yearly schedules.

use rust_decimal::Decimal;

/// Represents a single tax bracket with a rate and income bounds.
#[derive(Debug, Clone)]
pub struct TaxBracket {
    /// The lower income bound for this bracket
    pub lower_bound: Decimal,
    /// The optional upper income bound (None represents no upper limit)
    pub upper_bound: Option<Decimal>,
    /// The tax rate for this bracket as a decimal (e.g., 0.25 for 25%)
    pub rate: Decimal,
}

/// A complete set of tax brackets for a specific tax year.
#[derive(Debug, Clone)]
pub struct TaxSchedule {
    /// The tax year these brackets apply to
    pub tax_year: u16,
    /// The ordered list of tax brackets
    pub brackets: Vec<TaxBracket>,
}

impl TaxSchedule {
    /// Creates a new tax schedule with sorted brackets.
    ///
    /// # Arguments
    ///
    /// * `tax_year` - The year this schedule applies to
    /// * `brackets` - Vector of tax brackets that will be sorted by lower bound
    ///
    /// # Examples
    ///
    /// ```
    /// use tax_engine::models::{TaxSchedule, TaxBracket};
    /// use rust_decimal_macros::dec;
    ///
    /// let brackets = vec![
    ///     TaxBracket {
    ///         lower_bound: dec!(0),
    ///         upper_bound: Some(dec!(50000)),
    ///         rate: dec!(0.10),
    ///     }
    /// ];
    /// let schedule = TaxSchedule::new(2024, brackets);
    /// ```
    pub fn new(tax_year: u16, brackets: Vec<TaxBracket>) -> Self {
        let mut brackets = brackets;
        brackets.sort_by(|a, b| a.lower_bound.cmp(&b.lower_bound));
        Self { tax_year, brackets }
    }
}
