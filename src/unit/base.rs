//! Base unit definitions.

use crate::dimension::Dimension;
use std::fmt;

/// An irreducible base unit like meter, second, or kilogram.
#[derive(Clone, PartialEq)]
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
