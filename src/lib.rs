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
