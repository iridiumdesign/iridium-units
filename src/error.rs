//! Error types for the iridium-units library.

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
}

/// Result type alias for unit operations.
pub type UnitResult<T> = Result<T, UnitError>;
