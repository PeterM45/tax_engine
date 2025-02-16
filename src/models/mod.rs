//! Core domain models for the tax engine.
//!
//! This module contains the fundamental data structures and types used throughout
//! the tax calculation system, including tax brackets, entities, and jurisdictions.

mod bracket;
mod entity;
mod jurisdiction;

pub use bracket::{TaxBracket, TaxSchedule};
pub use entity::{Deduction, DeductionType, TaxEntity, TaxEntityType};
pub use jurisdiction::{CanadianProvince, Country, Jurisdiction, USState};
