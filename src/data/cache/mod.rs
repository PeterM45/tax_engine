use crate::errors::TaxError;
use crate::models::{Jurisdiction, TaxEntityType, TaxSchedule};
use async_trait::async_trait;

#[async_trait]
pub trait TaxDataCache: Send + Sync {
    async fn get(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
    ) -> Option<TaxSchedule>;

    async fn set(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
        schedule: TaxSchedule,
    ) -> Result<(), TaxError>;
}

pub mod memory;
