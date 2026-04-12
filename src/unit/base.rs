//! Base unit definitions.
//!
//! Base units are the fundamental building blocks of the unit system.
//! They are `Copy` types defined as `const` values in [`crate::systems`].
//!
//! # Examples
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! // Predefined base units are const and Copy
//! let distance = 42.195 * KM;
//! let in_miles = distance.to(MILE).unwrap();
//!
//! // Create composite units with arithmetic
//! let accel = 9.8 * &(M / S.pow(2));
//!
//! // Access unit metadata
//! assert_eq!(M.symbol, "m");
//! assert_eq!(M.name, "meter");
//! assert_eq!(M.dimension(), Dimension::LENGTH);
//! ```

use crate::dimension::Dimension;
use std::fmt;

/// An irreducible base unit like meter, second, or kilogram.
#[derive(Clone, Copy, PartialEq)]
pub struct BaseUnit {
    /// Primary name (e.g., "meter")
    pub name: &'static str,
    /// Short symbol (e.g., "m")
    pub symbol: &'static str,
    /// Alternative names/aliases
    pub aliases: &'static [&'static str],
    /// The dimension this unit represents
    pub dimension: Dimension,
    /// Scale relative to the canonical SI base unit (1.0 for SI base units)
    pub scale: f64,
    /// Additive offset for converting to SI: `SI = (value + offset) * scale`.
    /// Zero for most units. Celsius has offset 273.15 (K = °C + 273.15).
    pub offset: f64,
}

impl BaseUnit {
    /// Create a new base unit.
    pub const fn new(
        name: &'static str,
        symbol: &'static str,
        aliases: &'static [&'static str],
        dimension: Dimension,
        scale: f64,
    ) -> Self {
        BaseUnit {
            name,
            symbol,
            aliases,
            dimension,
            scale,
            offset: 0.0,
        }
    }

    /// Create a new base unit with an additive offset.
    /// Offset units use affine conversion via their `scale` and `offset`.
    pub const fn with_offset(
        name: &'static str,
        symbol: &'static str,
        aliases: &'static [&'static str],
        dimension: Dimension,
        scale: f64,
        offset: f64,
    ) -> Self {
        BaseUnit {
            name,
            symbol,
            aliases,
            dimension,
            scale,
            offset,
        }
    }
}

impl BaseUnit {
    /// Get the dimension of this unit.
    pub const fn dimension(&self) -> Dimension {
        self.dimension
    }

    /// Raise this unit to a power, returning a [`Unit`](crate::Unit).
    pub fn pow(&self, exp: impl Into<crate::dimension::Rational16>) -> crate::Unit {
        crate::Unit::from(*self).pow(exp)
    }
}

impl fmt::Debug for BaseUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BaseUnit({})", self.symbol)
    }
}

impl fmt::Display for BaseUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
