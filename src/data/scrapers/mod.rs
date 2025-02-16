//! Tax rate scraping functionality for various jurisdictions.
//!
//! Provides interfaces and implementations for fetching tax rates
//! from official government sources.

use crate::errors::TaxError;
use crate::models::{Jurisdiction, TaxEntityType, TaxSchedule};
use async_trait::async_trait;

/// Defines the interface for tax rate scraping implementations.
#[async_trait]
pub trait TaxRateScraper {
    /// Fetches tax rates from the appropriate source.
    ///
    /// # Arguments
    ///
    /// * `jurisdiction` - The tax jurisdiction to fetch rates for
    /// * `entity_type` - The type of tax entity
    /// * `tax_year` - The tax year
    ///
    /// # Returns
    ///
    /// Returns a Result containing either a TaxSchedule or an error.
    async fn fetch_rates(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
    ) -> Result<TaxSchedule, TaxError>;

    /// Checks if this scraper supports a given jurisdiction.
    ///
    /// # Arguments
    ///
    /// * `jurisdiction` - The jurisdiction to check
    ///
    /// # Returns
    ///
    /// Returns true if this scraper can handle the jurisdiction.
    fn supports_jurisdiction(&self, jurisdiction: &Jurisdiction) -> bool;
}

pub mod canada_federal;
pub mod us_federal;
