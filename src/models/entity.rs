//! Defines tax entities and their properties.
//!
//! This module provides types for representing different kinds of taxable entities
//! and managing their income and deductions.

use rust_decimal::Decimal;
use std::hash::Hash;

/// The type of entity being taxed.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TaxEntityType {
    /// Individual taxpayer
    Individual,
    /// Corporate entity
    Corporation,
    /// Business partnership
    Partnership,
}

/// Represents a taxable entity with income and deductions.
#[derive(Debug, Clone)]
pub struct TaxEntity {
    /// The type of this tax entity
    pub entity_type: TaxEntityType,
    /// Gross income before deductions
    pub income: Decimal,
    /// List of applicable deductions
    pub deductions: Vec<Deduction>,
    /// Tax year for this entity's calculations
    pub tax_year: u16,
}

/// Represents a single tax deduction.
#[derive(Debug, Clone)]
pub struct Deduction {
    /// The amount to be deducted
    pub amount: Decimal,
    /// The category of this deduction
    pub category: DeductionType,
}

/// Categories of tax deductions.
#[derive(Debug, Clone)]
pub enum DeductionType {
    /// Business-related deductions
    Business,
    /// Personal deductions
    Personal,
    /// Charitable contribution deductions
    Charitable,
}

impl TaxEntity {
    /// Creates a new tax entity without any deductions.
    pub fn new(entity_type: TaxEntityType, income: Decimal, tax_year: u16) -> Self {
        Self {
            entity_type,
            income,
            deductions: Vec::new(),
            tax_year,
        }
    }

    /// Adds a new deduction to this entity.
    pub fn add_deduction(&mut self, amount: Decimal, category: DeductionType) {
        self.deductions.push(Deduction { amount, category });
    }

    /// Calculates the total of all deductions.
    pub fn total_deductions(&self) -> Decimal {
        self.deductions
            .iter()
            .fold(Decimal::ZERO, |acc, d| acc + d.amount)
    }

    /// Calculates taxable income after applying all deductions.
    pub fn taxable_income(&self) -> Decimal {
        self.income - self.total_deductions()
    }
}
