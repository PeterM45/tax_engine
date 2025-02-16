//! Defines the core error types for tax operations.
//!
//! This module uses the `thiserror` crate to provide detailed error
//! handling with proper error message formatting and type safety.

use thiserror::Error;

/// Represents all possible errors that can occur during tax operations.
#[derive(Error, Debug)]
pub enum TaxError {
    /// Occurs when the tax year of an entity doesn't match its tax schedule.
    ///
    /// This typically happens when trying to calculate taxes using a schedule
    /// from a different year than the entity's tax year.
    #[error("Tax year mismatch between entity and schedule")]
    YearMismatch,

    /// Indicates invalid tax bracket configuration.
    ///
    /// This can occur when:
    /// - Brackets overlap
    /// - Brackets have gaps
    /// - Bracket rates are invalid (e.g., negative or over 100%)
    #[error("Invalid tax bracket configuration")]
    InvalidBrackets,

    /// Represents failures when fetching tax data from external sources.
    ///
    /// The String parameter provides details about the specific fetch error.
    /// This commonly occurs during network operations or when external
    /// services are unavailable.
    #[error("Failed to fetch tax data: {0}")]
    FetchError(String),

    /// Indicates failures in parsing tax data from external sources.
    ///
    /// The String parameter provides details about what went wrong during parsing.
    /// This can happen when:
    /// - The data format is unexpected
    /// - Required fields are missing
    /// - Numbers can't be parsed
    #[error("Failed to parse tax data: {0}")]
    ParseError(String),

    /// Indicates an attempt to use an unsupported tax jurisdiction.
    ///
    /// This occurs when trying to perform operations on tax jurisdictions
    /// that aren't implemented in the current version.
    #[error("Unsupported jurisdiction")]
    UnsupportedJurisdiction,

    /// Indicates that tax rates are not available for a specific year.
    ///
    /// The u16 parameter specifies which tax year's rates were unavailable.
    /// This can happen when:
    /// - The year is too far in the future
    /// - The year is too far in the past
    /// - The rates haven't been published yet
    #[error("Rate not available for year {0}")]
    RateNotAvailable(u16),

    /// Represents network-related errors during operations.
    ///
    /// The String parameter provides details about the specific network error.
    /// This can occur due to:
    /// - Connection timeouts
    /// - DNS resolution failures
    /// - TLS/SSL errors
    /// - Rate limiting
    #[error("Network error: {0}")]
    NetworkError(String),
}
