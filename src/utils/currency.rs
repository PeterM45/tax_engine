//! Currency handling utilities for parsing and formatting monetary values.
//!
//! This module provides helper functions for working with currency strings
//! and decimal values in a consistent format.

use rust_decimal::Decimal;
use std::str::FromStr;

/// Attempts to parse a currency string into a Decimal value.
///
/// This function handles common currency string formats by:
/// - Removing dollar signs ($)
/// - Removing commas (,)
/// - Trimming whitespace
///
/// # Arguments
///
/// * `input` - A string slice that represents a currency amount
///
/// # Returns
///
/// * `Some(Decimal)` if parsing was successful
/// * `None` if the string couldn't be parsed as a valid decimal
///
/// # Examples
///
/// ```
/// use tax_engine::utils::parse_currency_string;
///
/// let amount = parse_currency_string("$1,234.56");
/// assert_eq!(amount.map(|d| d.to_string()), Some("1234.56".to_string()));
///
/// let invalid = parse_currency_string("not a number");
/// assert_eq!(invalid, None);
/// ```
pub fn parse_currency_string(input: &str) -> Option<Decimal> {
    let cleaned = input.trim().replace('$', "").replace(',', "");
    Decimal::from_str(&cleaned).ok()
}

/// Formats a Decimal value as a currency string.
///
/// The output format includes:
/// - A leading dollar sign ($)
/// - Exactly two decimal places
///
/// # Arguments
///
/// * `amount` - The Decimal value to format
///
/// # Returns
///
/// A String representing the formatted currency amount
///
/// # Examples
///
/// ```
/// use tax_engine::utils::format_currency;
/// use rust_decimal_macros::dec;
///
/// let formatted = format_currency(dec!(1234.5));
/// assert_eq!(formatted, "$1234.50");
/// ```
pub fn format_currency(amount: Decimal) -> String {
    format!("${:.2}", amount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_parse_currency_string() {
        assert_eq!(parse_currency_string("$1,234.56"), Some(dec!(1234.56)));
        assert_eq!(parse_currency_string("1234.56"), Some(dec!(1234.56)));
        assert_eq!(parse_currency_string("$1,234"), Some(dec!(1234)));
        assert_eq!(parse_currency_string("invalid"), None);
    }

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(dec!(1234.56)), "$1234.56");
        assert_eq!(format_currency(dec!(1234.5)), "$1234.50");
        assert_eq!(format_currency(dec!(1234)), "$1234.00");
    }
}
