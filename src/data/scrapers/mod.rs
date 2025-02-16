use crate::errors::TaxError;
use crate::models::{Jurisdiction, TaxEntityType, TaxSchedule};
use async_trait::async_trait;

#[async_trait]
pub trait TaxRateScraper {
    async fn fetch_rates(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
    ) -> Result<TaxSchedule, TaxError>;

    fn supports_jurisdiction(&self, jurisdiction: &Jurisdiction) -> bool;
}

pub mod canada_federal;
pub mod us_federal;
