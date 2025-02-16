use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct TaxBracket {
    pub lower_bound: Decimal,
    pub upper_bound: Option<Decimal>, // None represents no upper limit
    pub rate: Decimal,
}

#[derive(Debug, Clone)]
pub struct TaxSchedule {
    pub tax_year: u16,
    pub brackets: Vec<TaxBracket>,
}

impl TaxSchedule {
    pub fn new(tax_year: u16, brackets: Vec<TaxBracket>) -> Self {
        // Sort brackets by lower bound to ensure correct calculation order
        let mut brackets = brackets;
        brackets.sort_by(|a, b| a.lower_bound.cmp(&b.lower_bound));
        Self { tax_year, brackets }
    }
}
