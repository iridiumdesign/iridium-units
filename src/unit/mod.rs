//! Unit types and operations.
//!
//! This module provides the [`Unit`] type, which represents a physical unit
//! like meter, kilogram, or m/s. Most users interact with units through
//! the predefined constants in [`crate::systems`] (e.g., `M`, `KG`, `HZ`).
//!
//! # Creating quantities
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! let distance = 100.0 * KM;
//! let time = 2.0 * H;
//! let speed = &distance / &time;
//!
//! // Convert to a different unit
//! let speed_ms = speed.to(M / S).unwrap();
//! assert!((speed_ms.value() - 13.8888).abs() < 0.001);
//! ```
//!
//! # Composite units from arithmetic
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! let velocity_unit = M / S;           // m/s
//! let accel_unit = M / S.pow(2);       // m/s²
//! let force_unit = KG * M / S.pow(2);  // kg·m/s² (newton)
//! ```

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
///
/// # Examples
///
/// ```
/// use iridium_units::prelude::*;
///
/// // Base units are Copy — use them directly
/// let mass = 10.0 * KG;
///
/// // Arithmetic creates composite units
/// let velocity_unit = KM / H;
///
/// // Check dimensions
/// assert_eq!((M / S).dimension(), (KM / H).dimension());
///
/// // Convert between compatible units
/// let speed = 100.0 * &(KM / H);
/// let in_ms = speed.to(M / S).unwrap();
/// ```
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

    /// Get the additive offset relative to the SI base unit.
    ///
    /// Most units have offset 0.0. Offset units like Celsius (273.15)
    /// and Fahrenheit (459.67) use this for affine conversions.
    /// The formula is: `SI_value = (value + offset) * scale`.
    ///
    /// Offsets are only defined for [`Unit::Base`]. Composite units are
    /// treated as interval units, so composing an offset base unit does
    /// **not** preserve its additive offset.
    pub fn offset(&self) -> f64 {
        match self {
            Unit::Base(b) => b.offset,
            Unit::Composite(_) | Unit::Dimensionless { .. } => 0.0,
        }
    }

    /// Check if this unit has an additive offset (e.g., Celsius, Fahrenheit).
    ///
    /// Only true for [`Unit::Base`] values with a non-zero offset.
    pub fn has_offset(&self) -> bool {
        self.offset() != 0.0
    }

    /// Convert a value in this unit to SI base units.
    ///
    /// For pure scale units: `value * scale`
    /// For offset base units: `(value + offset) * scale`
    ///
    /// Affine offsets are only applied for [`Unit::Base`]. Composite
    /// units are converted using scale alone (interval semantics).
    pub fn to_si(&self, value: f64) -> f64 {
        (value + self.offset()) * self.scale()
    }

    /// Convert a value from SI base units to this unit.
    ///
    /// For simple units: `si_value / scale`
    /// For offset units: `si_value / scale - offset`
    pub fn from_si(&self, si_value: f64) -> f64 {
        si_value / self.scale() - self.offset()
    }

    /// Get the conversion factor to convert from this unit to another.
    ///
    /// This returns a single multiplicative factor, which only works for
    /// units without additive offsets. For offset units like Celsius or
    /// Fahrenheit, use [`Quantity::to`](crate::Quantity::to) instead.
    ///
    /// Returns `Err` if the units have incompatible dimensions or if
    /// either unit has an additive offset.
    pub fn conversion_factor(&self, to: &Unit) -> UnitResult<f64> {
        if self.dimension() != to.dimension() {
            return Err(UnitError::DimensionMismatch {
                from: self.to_string(),
                to: to.to_string(),
            });
        }
        // Identity conversion is always valid, even for offset units
        if self == to {
            return Ok(1.0);
        }
        if self.has_offset() || to.has_offset() {
            return Err(UnitError::OffsetConversion {
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
        if exp == Rational16::ONE {
            return self.clone();
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

impl From<BaseUnit> for Unit {
    fn from(b: BaseUnit) -> Unit {
        Unit::Base(b)
    }
}

impl From<&BaseUnit> for Unit {
    fn from(b: &BaseUnit) -> Unit {
        Unit::Base(*b)
    }
}

impl From<&Unit> for Unit {
    fn from(u: &Unit) -> Unit {
        u.clone()
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

// BaseUnit * BaseUnit → Unit
impl Mul for BaseUnit {
    type Output = Unit;

    fn mul(self, rhs: BaseUnit) -> Unit {
        Unit::from(self) * Unit::from(rhs)
    }
}

// BaseUnit / BaseUnit → Unit
impl Div for BaseUnit {
    type Output = Unit;

    fn div(self, rhs: BaseUnit) -> Unit {
        Unit::from(self) / Unit::from(rhs)
    }
}

// BaseUnit * Unit → Unit
impl Mul<Unit> for BaseUnit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        Unit::from(self) * rhs
    }
}

// Unit * BaseUnit → Unit
impl Mul<BaseUnit> for Unit {
    type Output = Unit;

    fn mul(self, rhs: BaseUnit) -> Unit {
        self * Unit::from(rhs)
    }
}

// BaseUnit / Unit → Unit
impl Div<Unit> for BaseUnit {
    type Output = Unit;

    fn div(self, rhs: Unit) -> Unit {
        Unit::from(self) / rhs
    }
}

// Unit / BaseUnit → Unit
impl Div<BaseUnit> for Unit {
    type Output = Unit;

    fn div(self, rhs: BaseUnit) -> Unit {
        self / Unit::from(rhs)
    }
}

// &Unit * BaseUnit, &Unit / BaseUnit
impl Mul<BaseUnit> for &Unit {
    type Output = Unit;

    fn mul(self, rhs: BaseUnit) -> Unit {
        self * &Unit::from(rhs)
    }
}

impl Div<BaseUnit> for &Unit {
    type Output = Unit;

    fn div(self, rhs: BaseUnit) -> Unit {
        self / &Unit::from(rhs)
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

    #[test]
    fn test_offset_unit_conversion_factor_rejected() {
        let kelvin = Unit::Base(BaseUnit::new("kelvin", "K", &[], Dimension::TEMPERATURE, 1.0));
        let celsius = Unit::Base(BaseUnit::with_offset(
            "celsius", "°C", &[], Dimension::TEMPERATURE, 1.0, 273.15,
        ));
        let result = celsius.conversion_factor(&kelvin);
        assert!(matches!(result, Err(UnitError::OffsetConversion { .. })));
    }

    #[test]
    fn test_offset_unit_identity_conversion_ok() {
        let celsius = Unit::Base(BaseUnit::with_offset(
            "celsius", "°C", &[], Dimension::TEMPERATURE, 1.0, 273.15,
        ));
        let result = celsius.conversion_factor(&celsius);
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_pow_one_preserves_unit() {
        let celsius = Unit::Base(BaseUnit::with_offset(
            "celsius", "°C", &[], Dimension::TEMPERATURE, 1.0, 273.15,
        ));
        let powered = celsius.pow(1);
        assert!(powered.has_offset());
        assert_eq!(celsius, powered);
    }
}
