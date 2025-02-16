use rust_decimal::Decimal;
use std::hash::Hash;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TaxEntityType {
    Individual,
    Corporation,
    Partnership,
}

#[derive(Debug, Clone)]
pub struct TaxEntity {
    pub entity_type: TaxEntityType,
    pub income: Decimal,
    pub deductions: Vec<Deduction>,
    pub tax_year: u16,
}

#[derive(Debug, Clone)]
pub struct Deduction {
    pub amount: Decimal,
    pub category: DeductionType,
}

#[derive(Debug, Clone)]
pub enum DeductionType {
    Business,
    Personal,
    Charitable,
    // Add more as needed
}

impl TaxEntity {
    pub fn new(entity_type: TaxEntityType, income: Decimal, tax_year: u16) -> Self {
        Self {
            entity_type,
            income,
            deductions: Vec::new(),
            tax_year,
        }
    }

    pub fn add_deduction(&mut self, amount: Decimal, category: DeductionType) {
        self.deductions.push(Deduction { amount, category });
    }

    pub fn total_deductions(&self) -> Decimal {
        self.deductions
            .iter()
            .fold(Decimal::ZERO, |acc, d| acc + d.amount)
    }

    pub fn taxable_income(&self) -> Decimal {
        self.income - self.total_deductions()
    }
}
