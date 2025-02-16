use super::TaxDataCache;
use crate::errors::TaxError;
use crate::models::{Jurisdiction, TaxEntityType, TaxSchedule};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// An in-memory cache implementation for tax schedules
pub struct MemoryCache {
    data: Arc<RwLock<HashMap<CacheKey, CacheEntry>>>,
    ttl: Duration,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct CacheKey {
    pub jurisdiction: Jurisdiction,
    pub entity_type: TaxEntityType,
    pub tax_year: u16,
}

#[derive(Clone, Debug)]
pub struct CacheEntry {
    pub schedule: TaxSchedule,
    pub timestamp: Instant,
}

impl MemoryCache {
    /// Creates a new MemoryCache instance with the specified TTL
    pub fn new(ttl: Duration) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    /// Gets the current TTL duration
    pub fn ttl(&self) -> Duration {
        self.ttl
    }
}

#[async_trait]
impl TaxDataCache for MemoryCache {
    async fn get(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
    ) -> Option<TaxSchedule> {
        let key = CacheKey {
            jurisdiction: jurisdiction.clone(),
            entity_type: entity_type.clone(),
            tax_year,
        };

        let cache = self.data.read().await;
        if let Some(entry) = cache.get(&key) {
            if entry.timestamp.elapsed() < self.ttl {
                return Some(entry.schedule.clone());
            }
        }
        None
    }

    async fn set(
        &self,
        jurisdiction: &Jurisdiction,
        entity_type: &TaxEntityType,
        tax_year: u16,
        schedule: TaxSchedule,
    ) -> Result<(), TaxError> {
        let key = CacheKey {
            jurisdiction: jurisdiction.clone(),
            entity_type: entity_type.clone(),
            tax_year,
        };

        let entry = CacheEntry {
            schedule,
            timestamp: Instant::now(),
        };

        let mut cache = self.data.write().await;
        cache.insert(key, entry);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_cache_set_get() {
        let cache = MemoryCache::new(Duration::from_secs(60));
        let jurisdiction = Jurisdiction::Federal(crate::models::Country::USA);
        let entity_type = TaxEntityType::Individual;
        let tax_year = 2024;

        let schedule = TaxSchedule::new(
            tax_year,
            vec![crate::models::TaxBracket {
                lower_bound: dec!(0),
                upper_bound: Some(dec!(50000)),
                rate: dec!(0.10),
            }],
        );

        // Test set
        let set_result = cache
            .set(&jurisdiction, &entity_type, tax_year, schedule.clone())
            .await;
        assert!(set_result.is_ok());

        // Test get
        let get_result = cache.get(&jurisdiction, &entity_type, tax_year).await;
        assert!(get_result.is_some());
        assert_eq!(get_result.unwrap().tax_year, tax_year);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = MemoryCache::new(Duration::from_millis(100));
        let jurisdiction = Jurisdiction::Federal(crate::models::Country::USA);
        let entity_type = TaxEntityType::Individual;
        let tax_year = 2024;

        let schedule = TaxSchedule::new(
            tax_year,
            vec![crate::models::TaxBracket {
                lower_bound: dec!(0),
                upper_bound: Some(dec!(50000)),
                rate: dec!(0.10),
            }],
        );

        // Set the value
        let _ = cache
            .set(&jurisdiction, &entity_type, tax_year, schedule)
            .await;

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should be expired
        let result = cache.get(&jurisdiction, &entity_type, tax_year).await;
        assert!(result.is_none());
    }
}
