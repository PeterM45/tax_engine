use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaxError {
    #[error("Tax year mismatch between entity and schedule")]
    YearMismatch,
    #[error("Invalid tax bracket configuration")]
    InvalidBrackets,
    #[error("Failed to fetch tax data: {0}")]
    FetchError(String),
    #[error("Failed to parse tax data: {0}")]
    ParseError(String),
    #[error("Unsupported jurisdiction")]
    UnsupportedJurisdiction,
    #[error("Rate not available for year {0}")]
    RateNotAvailable(u16),
    #[error("Network error: {0}")]
    NetworkError(String),
}
