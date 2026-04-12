//! Error types for the iridium-units library.
//!
//! All fallible operations return [`UnitResult<T>`], which is an alias for
//! `Result<T, UnitError>`. Errors include helpful context like unit names
//! and, for unknown units, spelling suggestions.
//!
//! # Examples
//!
//! ```
//! use iridium_units::prelude::*;
//! use iridium_units::error::UnitError;
//!
//! // Incompatible conversion
//! let mass = 1.0 * KG;
//! let result = mass.to(M);
//! assert!(matches!(result, Err(UnitError::DimensionMismatch { .. })));
//!
//! // Incompatible addition
//! let distance = 10.0 * M;
//! let time = 5.0 * S;
//! let result = distance.checked_add(&time);
//! assert!(matches!(result, Err(UnitError::IncompatibleDimensions { .. })));
//!
//! // Unknown unit with suggestion
//! let result = parse_unit("metrs");
//! if let Err(UnitError::UnknownUnit { suggestions, .. }) = result {
//!     assert!(!suggestions.is_empty());
//! }
//! ```

use thiserror::Error;

/// Errors that can occur during unit operations.
#[derive(Error, Debug, Clone)]
pub enum UnitError {
    /// Attempted to convert between incompatible dimensions.
    #[error("cannot convert between incompatible dimensions: {from} -> {to}")]
    DimensionMismatch { from: String, to: String },

    /// Attempted to add or subtract quantities with different dimensions.
    #[error("cannot add/subtract quantities with different dimensions: {lhs} vs {rhs}")]
    IncompatibleDimensions { lhs: String, rhs: String },

    /// Unknown unit name in string parsing.
    #[error("unknown unit: '{name}'{}",
        if suggestions.is_empty() { String::new() }
        else { format!(", did you mean '{}'?", suggestions.join("' or '")) }
    )]
    UnknownUnit {
        name: String,
        suggestions: Vec<String>,
    },

    /// Failed to parse a unit string.
    #[error("failed to parse unit string: {0}")]
    ParseError(String),

    /// No equivalency available for the requested conversion.
    #[error("no equivalency found for conversion: {from} -> {to}")]
    NoEquivalency { from: String, to: String },

    /// Attempted to get scalar value from non-dimensionless quantity.
    #[error("cannot convert non-dimensionless quantity to scalar")]
    NotDimensionless,

    /// Invalid operation on logarithmic unit.
    #[error("invalid operation on logarithmic unit: {0}")]
    LogarithmicError(String),

    /// Overflow in dimension exponent calculation.
    #[error("overflow in dimension exponent calculation")]
    DimensionOverflow,

    /// Zero denominator in rational exponent.
    #[error("zero denominator in rational exponent")]
    ZeroDenominator,

    /// Attempted to use a simple scale factor for offset unit conversion.
    /// Use `Quantity::to()` instead of `Unit::conversion_factor()` for offset units.
    #[error("cannot use simple scaling for offset units: {from} -> {to}, use Quantity::to() instead")]
    OffsetConversion { from: String, to: String },

    /// Invalid input for batch operation.
    #[error("batch operation error: {0}")]
    BatchError(String),
}

/// Result type alias for unit operations.
pub type UnitResult<T> = Result<T, UnitError>;
