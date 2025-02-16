use rust_decimal::Decimal;
use std::str::FromStr;

pub fn parse_currency_string(input: &str) -> Option<Decimal> {
    let cleaned = input.trim().replace('$', "").replace(',', "");
    Decimal::from_str(&cleaned).ok()
}

pub fn format_currency(amount: Decimal) -> String {
    format!("${:.2}", amount)
}
