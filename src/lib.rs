//! A comprehensive tax calculation engine supporting multiple jurisdictions.
//!
//! The tax_engine library provides tools for calculating taxes across different
//! jurisdictions, with support for automated rate fetching and caching.
//!
//! # Features
//!
//! - Automated tax bracket fetching from official sources
//! - Support for multiple jurisdictions (US Federal, Canadian Federal)
//! - Type-safe decimal calculations using rust_decimal
//! - Caching with configurable TTL
//! - Robust error handling
//!
//! # Example
//!
//! ```rust
//! use tax_engine::{
//!     Country, IncomeTaxCalculator, Jurisdiction, TaxEntity,
//!     TaxEntityType, USFederalScraper
//! };
//! use rust_decimal_macros::dec;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a tax entity with $75,000 income
//!     let mut entity = TaxEntity::new(
//!         TaxEntityType::Individual,
//!         dec!(75000),
//!         2024
//!     );
//!
//!     // Add a $5,000 personal deduction
//!     entity.add_deduction(dec!(5000), DeductionType::Personal);
//!
//!     // Fetch current tax rates
//!     let scraper = USFederalScraper::new();
//!     let schedule = scraper
//!         .fetch_rates(
//!             &Jurisdiction::Federal(Country::USA),
//!             &TaxEntityType::Individual,
//!             2024
//!         )
//!         .await?;
//!
//!     // Calculate tax
//!     let tax = IncomeTaxCalculator::calculate_tax(&entity, &schedule)?;
//!     println!("Tax owed: ${:.2}", tax);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Module Organization
//!
//! - `calculators`: Tax calculation implementations
//! - `data`: Data fetching and caching
//! - `errors`: Error types and handling
//! - `models`: Core domain models
//! - `utils`: Utility functions

pub mod calculators;
pub mod data;
pub mod errors;
pub mod models;
pub mod utils;

// Re-export commonly used items
pub use calculators::IncomeTaxCalculator;
pub use data::cache::memory::MemoryCache;
pub use data::scrapers::{us_federal::USFederalScraper, TaxRateScraper};
pub use errors::TaxError;
pub use models::{
    Country, DeductionType, Jurisdiction, TaxBracket, TaxEntity, TaxEntityType, TaxSchedule,
};
pub use utils::currency::format_currency;
