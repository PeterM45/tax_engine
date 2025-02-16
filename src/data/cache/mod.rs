//! Caching infrastructure for tax data.
//!
//! This module provides traits and implementations for caching tax schedules
//! to reduce network calls and improve performance.

use crate::errors::TaxError;
use crate::models::{Jurisdiction, TaxEntityType, TaxSchedule};
use async_trait::async_trait;

/// Defines the interface for tax data caching implementations.
#[async_trait]
pub trait TaxDataCache: Send + Sync {
    /// Retrieves a tax schedule from the cache.
    ///
    /// # Arguments
    ///
    /// * `jurisdiction` - The tax jurisdiction
    /// * `entity_type` - The type of tax entity
    /// * `tax_year` - The tax year
    ///
    /// # Returns
    ///
    /// * `Some(TaxSchedule)` if found in cache and not expired
    /// * `None` if not found or expired
    async fn get(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
    ) -> Option<TaxSchedule>;

    /// Stores a tax schedule in the cache.
    ///
    /// # Arguments
    ///
    /// * `jurisdiction` - The tax jurisdiction
    /// * `entity_type` - The type of tax entity
    /// * `tax_year` - The tax year
    /// * `schedule` - The tax schedule to cache
    ///
    /// # Returns
    ///
    /// * `Ok(())` if successful
    /// * `Err(TaxError)` if caching fails
    async fn set(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
        schedule: TaxSchedule,
    ) -> Result<(), TaxError>;
}

pub mod memory;
