//! Unit types and operations.

pub mod base;
pub mod composite;

use crate::dimension::{Dimension, Rational16};
use crate::error::{UnitError, UnitResult};
use base::BaseUnit;
use composite::CompositeUnit;
use std::fmt;
use std::ops::{Div, Mul};

/// A physical unit.
///
/// Units can be base units (like meter), named derived units (like newton),
/// composite units (like m/s), or dimensionless.
#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    /// A base irreducible unit (meter, second, kilogram, etc.)
    Base(BaseUnit),

    /// A composite unit from arithmetic operations (m/s, kg·m/s², etc.)
    Composite(CompositeUnit),

    /// Dimensionless with optional scale factor
    Dimensionless { scale: f64 },
}

impl Unit {
    /// Create a dimensionless unit with scale 1.
    pub fn dimensionless() -> Self {
        Unit::Dimensionless { scale: 1.0 }
    }

    /// Create a dimensionless unit with a scale factor.
    pub fn dimensionless_scaled(scale: f64) -> Self {
        Unit::Dimensionless { scale }
    }

    /// Create a unit from a base unit.
    pub fn from_base(base: &BaseUnit) -> Self {
        Unit::Base(base.clone())
    }

    /// Get the dimension of this unit.
    pub fn dimension(&self) -> Dimension {
        match self {
            Unit::Base(b) => b.dimension,
            Unit::Composite(c) => c.dimension(),
            Unit::Dimensionless { .. } => Dimension::DIMENSIONLESS,
        }
    }

    /// Get the scale factor relative to SI base units.
    pub fn scale(&self) -> f64 {
        match self {
            Unit::Base(b) => b.scale,
            Unit::Composite(c) => c.total_scale(),
            Unit::Dimensionless { scale } => *scale,
        }
    }

    /// Check if this unit is dimensionless.
    pub fn is_dimensionless(&self) -> bool {
        self.dimension().is_dimensionless()
    }

    /// Get the conversion factor to convert from this unit to another.
    ///
    /// Returns `Err` if the units have incompatible dimensions.
    pub fn conversion_factor(&self, to: &Unit) -> UnitResult<f64> {
        if self.dimension() != to.dimension() {
            return Err(UnitError::DimensionMismatch {
                from: self.to_string(),
                to: to.to_string(),
            });
        }
        Ok(self.scale() / to.scale())
    }

    /// Convert this unit to a composite representation.
    pub fn to_composite(&self) -> CompositeUnit {
        match self {
            Unit::Base(b) => CompositeUnit::from_base(b.symbol, b.dimension, b.scale),
            Unit::Composite(c) => c.clone(),
            Unit::Dimensionless { scale } => CompositeUnit::dimensionless(*scale),
        }
    }

    /// Raise this unit to a power.
    pub fn pow(&self, exp: impl Into<Rational16>) -> Unit {
        let exp = exp.into();
        if exp.is_zero() {
            return Unit::dimensionless();
        }
        Unit::Composite(self.to_composite().pow(exp))
    }

    /// Take the square root of this unit.
    pub fn sqrt(&self) -> Unit {
        self.pow(Rational16::new(1, 2))
    }

    /// Invert this unit (raise to power -1).
    pub fn inv(&self) -> Unit {
        self.pow(Rational16::new(-1, 1))
    }

    /// Get the symbol/string representation for this unit.
    pub fn symbol(&self) -> String {
        match self {
            Unit::Base(b) => b.symbol.to_string(),
            Unit::Composite(c) => c.to_string(),
            Unit::Dimensionless { scale } => {
                if (*scale - 1.0).abs() < 1e-15 {
                    "".to_string()
                } else {
                    format!("{}", scale)
                }
            }
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Base(b) => write!(f, "{}", b.symbol),
            Unit::Composite(c) => write!(f, "{}", c),
            Unit::Dimensionless { scale } => {
                if (*scale - 1.0).abs() < 1e-15 {
                    write!(f, "dimensionless")
                } else {
                    write!(f, "{}", scale)
                }
            }
        }
    }
}

// Unit * Unit
impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        Unit::Composite(self.to_composite().mul(&rhs.to_composite()))
    }
}

impl Mul for &Unit {
    type Output = Unit;

    fn mul(self, rhs: &Unit) -> Unit {
        Unit::Composite(self.to_composite().mul(&rhs.to_composite()))
    }
}

impl Mul<&Unit> for Unit {
    type Output = Unit;

    fn mul(self, rhs: &Unit) -> Unit {
        Unit::Composite(self.to_composite().mul(&rhs.to_composite()))
    }
}

impl Mul<Unit> for &Unit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        Unit::Composite(self.to_composite().mul(&rhs.to_composite()))
    }
}

// Unit / Unit
impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Unit) -> Unit {
        Unit::Composite(self.to_composite().div(&rhs.to_composite()))
    }
}

impl Div for &Unit {
    type Output = Unit;

    fn div(self, rhs: &Unit) -> Unit {
        Unit::Composite(self.to_composite().div(&rhs.to_composite()))
    }
}

impl Div<&Unit> for Unit {
    type Output = Unit;

    fn div(self, rhs: &Unit) -> Unit {
        Unit::Composite(self.to_composite().div(&rhs.to_composite()))
    }
}

impl Div<Unit> for &Unit {
    type Output = Unit;

    fn div(self, rhs: Unit) -> Unit {
        Unit::Composite(self.to_composite().div(&rhs.to_composite()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn meter() -> Unit {
        Unit::Base(BaseUnit::new("meter", "m", &[], Dimension::LENGTH, 1.0))
    }

    fn second() -> Unit {
        Unit::Base(BaseUnit::new("second", "s", &[], Dimension::TIME, 1.0))
    }

    fn kilometer() -> Unit {
        Unit::Base(BaseUnit::new("kilometer", "km", &[], Dimension::LENGTH, 1000.0))
    }

    #[test]
    fn test_unit_division() {
        let velocity = meter() / second();
        let dim = velocity.dimension();
        assert_eq!(dim.length, Rational16::ONE);
        assert_eq!(dim.time, Rational16::new(-1, 1));
    }

    #[test]
    fn test_conversion_factor() {
        let m = meter();
        let km = kilometer();
        let factor = km.conversion_factor(&m).unwrap();
        assert!((factor - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_incompatible_conversion() {
        let m = meter();
        let s = second();
        let result = m.conversion_factor(&s);
        assert!(matches!(result, Err(UnitError::DimensionMismatch { .. })));
    }

    #[test]
    fn test_unit_power() {
        let m = meter();
        let m2 = m.pow(2);
        let dim = m2.dimension();
        assert_eq!(dim.length, Rational16::new(2, 1));
    }

    #[test]
    fn test_unit_sqrt() {
        let m = meter();
        let m2 = &m * &m;
        let sqrt_m2 = m2.sqrt();
        let dim = sqrt_m2.dimension();
        assert_eq!(dim.length, Rational16::ONE);
    }
}
