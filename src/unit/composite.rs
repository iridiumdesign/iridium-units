//! Composite unit definitions.

use crate::dimension::{Dimension, Rational8};
use std::fmt;

/// A component of a composite unit: a unit symbol with its power.
#[derive(Clone, Debug, PartialEq)]
pub struct UnitComponent {
    /// The unit symbol (e.g., "m", "s", "kg")
    pub symbol: String,
    /// The dimension of this component
    pub dimension: Dimension,
    /// The scale factor of this component
    pub scale: f64,
    /// The power/exponent of this component
    pub power: Rational8,
}

impl UnitComponent {
    /// Create a new unit component.
    pub fn new(symbol: impl Into<String>, dimension: Dimension, scale: f64, power: Rational8) -> Self {
        UnitComponent {
            symbol: symbol.into(),
            dimension,
            scale,
            power,
        }
    }

    /// Get the effective dimension (dimension * power).
    pub fn effective_dimension(&self) -> Dimension {
        self.dimension.pow(self.power)
    }

    /// Get the effective scale (scale ^ power).
    pub fn effective_scale(&self) -> f64 {
        self.scale.powf(self.power.to_f64())
    }
}

/// A composite unit built from multiple base units with powers.
///
/// Examples: m/s (velocity), kg·m/s² (force), W/m²/Hz (spectral flux density)
#[derive(Clone, Debug, PartialEq)]
pub struct CompositeUnit {
    /// Additional scale factor applied to the whole unit
    pub scale: f64,
    /// Component units with their symbols, dimensions, scales, and powers
    pub components: Vec<UnitComponent>,
}

impl CompositeUnit {
    /// Create a new composite unit from components.
    pub fn new(scale: f64, components: Vec<UnitComponent>) -> Self {
        CompositeUnit { scale, components }
    }

    /// Create a dimensionless composite unit with just a scale factor.
    pub fn dimensionless(scale: f64) -> Self {
        CompositeUnit {
            scale,
            components: Vec::new(),
        }
    }

    /// Create a composite unit from a single base unit.
    pub fn from_base(symbol: impl Into<String>, dimension: Dimension, scale: f64) -> Self {
        CompositeUnit {
            scale: 1.0,
            components: vec![UnitComponent::new(symbol, dimension, scale, Rational8::ONE)],
        }
    }

    /// Get the total dimension of this composite unit.
    pub fn dimension(&self) -> Dimension {
        let mut dim = Dimension::DIMENSIONLESS;
        for comp in &self.components {
            dim = dim.mul(&comp.effective_dimension());
        }
        dim
    }

    /// Get the total scale factor relative to SI base units.
    pub fn total_scale(&self) -> f64 {
        let mut s = self.scale;
        for comp in &self.components {
            s *= comp.effective_scale();
        }
        s
    }

    /// Multiply two composite units.
    pub fn mul(&self, other: &CompositeUnit) -> CompositeUnit {
        let mut components = self.components.clone();

        // For each component in other, try to combine with existing or add new
        for other_comp in &other.components {
            let mut found = false;
            for comp in &mut components {
                if comp.symbol == other_comp.symbol && comp.dimension == other_comp.dimension {
                    comp.power = comp.power + other_comp.power;
                    found = true;
                    break;
                }
            }
            if !found {
                components.push(other_comp.clone());
            }
        }

        // Remove components with zero power
        components.retain(|c| !c.power.is_zero());

        CompositeUnit {
            scale: self.scale * other.scale,
            components,
        }
    }

    /// Divide this composite unit by another.
    pub fn div(&self, other: &CompositeUnit) -> CompositeUnit {
        self.mul(&other.inv())
    }

    /// Invert this composite unit.
    pub fn inv(&self) -> CompositeUnit {
        CompositeUnit {
            scale: 1.0 / self.scale,
            components: self
                .components
                .iter()
                .map(|c| UnitComponent {
                    symbol: c.symbol.clone(),
                    dimension: c.dimension,
                    scale: c.scale,
                    power: -c.power,
                })
                .collect(),
        }
    }

    /// Raise this composite unit to a power.
    pub fn pow(&self, power: Rational8) -> CompositeUnit {
        CompositeUnit {
            scale: self.scale.powf(power.to_f64()),
            components: self
                .components
                .iter()
                .map(|c| UnitComponent {
                    symbol: c.symbol.clone(),
                    dimension: c.dimension,
                    scale: c.scale,
                    power: c.power * power,
                })
                .collect(),
        }
    }
}

impl fmt::Display for CompositeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.components.is_empty() {
            if (self.scale - 1.0).abs() < 1e-15 {
                return write!(f, "dimensionless");
            } else {
                return write!(f, "{}", self.scale);
            }
        }

        let mut positive = Vec::new();
        let mut negative = Vec::new();

        for comp in &self.components {
            if comp.power.numer > 0 {
                positive.push(comp);
            } else if comp.power.numer < 0 {
                negative.push(comp);
            }
        }

        // Format positive powers
        let pos_str: Vec<String> = positive
            .iter()
            .map(|c| {
                if c.power == Rational8::ONE {
                    c.symbol.clone()
                } else {
                    format!("{}^{}", c.symbol, c.power)
                }
            })
            .collect();

        // Format negative powers (with absolute value)
        let neg_str: Vec<String> = negative
            .iter()
            .map(|c| {
                let abs_power = Rational8::new(-c.power.numer, c.power.denom);
                if abs_power == Rational8::ONE {
                    c.symbol.clone()
                } else {
                    format!("{}^{}", c.symbol, abs_power)
                }
            })
            .collect();

        let scale_prefix = if (self.scale - 1.0).abs() < 1e-15 {
            String::new()
        } else {
            format!("{} ", self.scale)
        };

        if negative.is_empty() {
            write!(f, "{}{}", scale_prefix, pos_str.join(" "))
        } else if positive.is_empty() {
            write!(f, "{}1 / {}", scale_prefix, neg_str.join(" "))
        } else {
            write!(f, "{}{} / {}", scale_prefix, pos_str.join(" "), neg_str.join(" "))
        }
    }
}
