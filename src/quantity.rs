//! Physical quantities with values and units.

use crate::error::{UnitError, UnitResult};
use crate::unit::Unit;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A physical quantity: a numeric value with an associated unit.
///
/// # Examples
///
/// ```
/// use iridium_units::prelude::*;
///
/// let distance = 100.0 * &*M;
/// let time = 9.58 * &*S;
/// let speed = &distance / &time;
///
/// let speed_kmh = speed.to(&(&*KM / &*H)).unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Quantity {
    value: f64,
    unit: Unit,
}

impl Quantity {
    /// Create a new quantity with a value and unit.
    pub fn new(value: f64, unit: Unit) -> Self {
        Quantity { value, unit }
    }

    /// Get the numeric value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the unit.
    pub fn unit(&self) -> &Unit {
        &self.unit
    }

    /// Check if this quantity is dimensionless.
    pub fn is_dimensionless(&self) -> bool {
        self.unit.is_dimensionless()
    }

    /// Get the value as a dimensionless scalar.
    ///
    /// Returns `Err` if the quantity is not dimensionless.
    pub fn dimensionless_value(&self) -> UnitResult<f64> {
        if !self.is_dimensionless() {
            return Err(UnitError::NotDimensionless);
        }
        Ok(self.value * self.unit.scale())
    }

    /// Convert to another unit.
    ///
    /// Returns `Err` if the units have incompatible dimensions.
    pub fn to(&self, target: &Unit) -> UnitResult<Quantity> {
        let factor = self.unit.conversion_factor(target)?;
        Ok(Quantity::new(self.value * factor, target.clone()))
    }

    /// Get the value in a target unit.
    ///
    /// Shorthand for `.to(target)?.value()`.
    pub fn to_value(&self, target: &Unit) -> UnitResult<f64> {
        Ok(self.to(target)?.value())
    }

    /// Decompose to SI base units.
    pub fn decompose(&self) -> Quantity {
        // Convert to SI base units by applying the scale factor
        Quantity::new(
            self.value * self.unit.scale(),
            Unit::Composite(crate::unit::composite::CompositeUnit::new(
                1.0,
                self.dimension_components(),
            )),
        )
    }

    /// Get the dimension components for the decomposed unit.
    fn dimension_components(&self) -> Vec<crate::unit::composite::UnitComponent> {
        use crate::dimension::{Dimension, Rational8};
        use crate::unit::composite::UnitComponent;

        let dim = self.unit.dimension();
        let mut components = Vec::new();

        let add_if_nonzero = |comps: &mut Vec<UnitComponent>, symbol: &str, base_dim: Dimension, exp: Rational8| {
            if !exp.is_zero() {
                comps.push(UnitComponent::new(symbol, base_dim, 1.0, exp));
            }
        };

        add_if_nonzero(&mut components, "m", Dimension::LENGTH, dim.length);
        add_if_nonzero(&mut components, "s", Dimension::TIME, dim.time);
        add_if_nonzero(&mut components, "kg", Dimension::MASS, dim.mass);
        add_if_nonzero(&mut components, "A", Dimension::CURRENT, dim.current);
        add_if_nonzero(&mut components, "K", Dimension::TEMPERATURE, dim.temperature);
        add_if_nonzero(&mut components, "rad", Dimension::ANGLE, dim.angle);
        add_if_nonzero(&mut components, "sr", Dimension::SOLID_ANGLE, dim.solid_angle);
        add_if_nonzero(&mut components, "cd", Dimension::LUMINOUS_INTENSITY, dim.luminous_intensity);
        add_if_nonzero(&mut components, "mag", Dimension::MAGNITUDE, dim.magnitude);
        add_if_nonzero(&mut components, "mol", Dimension::AMOUNT, dim.amount);
        add_if_nonzero(&mut components, "ph", Dimension::PHOTON, dim.photon);

        components
    }

    /// Raise this quantity to a power.
    pub fn pow(&self, exp: i32) -> Quantity {
        Quantity::new(
            self.value.powi(exp),
            self.unit.pow(exp),
        )
    }

    /// Take the square root of this quantity.
    pub fn sqrt(&self) -> Quantity {
        Quantity::new(self.value.sqrt(), self.unit.sqrt())
    }

    /// Get the absolute value of this quantity.
    pub fn abs(&self) -> Quantity {
        Quantity::new(self.value.abs(), self.unit.clone())
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_str = self.unit.to_string();
        if unit_str.is_empty() || unit_str == "dimensionless" {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, unit_str)
        }
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        // Two quantities are equal if they have the same dimension and
        // the same value when converted to the same unit
        if self.unit.dimension() != other.unit.dimension() {
            return false;
        }
        let self_si = self.value * self.unit.scale();
        let other_si = other.value * other.unit.scale();
        (self_si - other_si).abs() < 1e-15 * self_si.abs().max(other_si.abs()).max(1e-15)
    }
}

// Quantity + Quantity
impl Add for Quantity {
    type Output = UnitResult<Quantity>;

    fn add(self, rhs: Quantity) -> UnitResult<Quantity> {
        if self.unit.dimension() != rhs.unit.dimension() {
            return Err(UnitError::IncompatibleDimensions {
                lhs: self.unit.to_string(),
                rhs: rhs.unit.to_string(),
            });
        }
        // Convert rhs to self's unit
        let rhs_converted = rhs.to(&self.unit)?;
        Ok(Quantity::new(self.value + rhs_converted.value, self.unit))
    }
}

impl Add for &Quantity {
    type Output = UnitResult<Quantity>;

    fn add(self, rhs: &Quantity) -> UnitResult<Quantity> {
        self.clone() + rhs.clone()
    }
}

// Quantity - Quantity
impl Sub for Quantity {
    type Output = UnitResult<Quantity>;

    fn sub(self, rhs: Quantity) -> UnitResult<Quantity> {
        if self.unit.dimension() != rhs.unit.dimension() {
            return Err(UnitError::IncompatibleDimensions {
                lhs: self.unit.to_string(),
                rhs: rhs.unit.to_string(),
            });
        }
        let rhs_converted = rhs.to(&self.unit)?;
        Ok(Quantity::new(self.value - rhs_converted.value, self.unit))
    }
}

impl Sub for &Quantity {
    type Output = UnitResult<Quantity>;

    fn sub(self, rhs: &Quantity) -> UnitResult<Quantity> {
        self.clone() - rhs.clone()
    }
}

// Quantity * Quantity
impl Mul for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity {
        Quantity::new(self.value * rhs.value, self.unit * rhs.unit)
    }
}

impl Mul for &Quantity {
    type Output = Quantity;

    fn mul(self, rhs: &Quantity) -> Quantity {
        Quantity::new(self.value * rhs.value, &self.unit * &rhs.unit)
    }
}

impl Mul<&Quantity> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: &Quantity) -> Quantity {
        Quantity::new(self.value * rhs.value, self.unit * &rhs.unit)
    }
}

impl Mul<Quantity> for &Quantity {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity {
        Quantity::new(self.value * rhs.value, &self.unit * rhs.unit)
    }
}

// Quantity / Quantity
impl Div for Quantity {
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::new(self.value / rhs.value, self.unit / rhs.unit)
    }
}

impl Div for &Quantity {
    type Output = Quantity;

    fn div(self, rhs: &Quantity) -> Quantity {
        Quantity::new(self.value / rhs.value, &self.unit / &rhs.unit)
    }
}

impl Div<&Quantity> for Quantity {
    type Output = Quantity;

    fn div(self, rhs: &Quantity) -> Quantity {
        Quantity::new(self.value / rhs.value, self.unit / &rhs.unit)
    }
}

impl Div<Quantity> for &Quantity {
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::new(self.value / rhs.value, &self.unit / rhs.unit)
    }
}

// Quantity * f64
impl Mul<f64> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: f64) -> Quantity {
        Quantity::new(self.value * rhs, self.unit)
    }
}

impl Mul<f64> for &Quantity {
    type Output = Quantity;

    fn mul(self, rhs: f64) -> Quantity {
        Quantity::new(self.value * rhs, self.unit.clone())
    }
}

// f64 * Quantity
impl Mul<Quantity> for f64 {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity {
        Quantity::new(self * rhs.value, rhs.unit)
    }
}

impl Mul<&Quantity> for f64 {
    type Output = Quantity;

    fn mul(self, rhs: &Quantity) -> Quantity {
        Quantity::new(self * rhs.value, rhs.unit.clone())
    }
}

// Quantity / f64
impl Div<f64> for Quantity {
    type Output = Quantity;

    fn div(self, rhs: f64) -> Quantity {
        Quantity::new(self.value / rhs, self.unit)
    }
}

impl Div<f64> for &Quantity {
    type Output = Quantity;

    fn div(self, rhs: f64) -> Quantity {
        Quantity::new(self.value / rhs, self.unit.clone())
    }
}

// f64 / Quantity
impl Div<Quantity> for f64 {
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::new(self / rhs.value, rhs.unit.inv())
    }
}

impl Div<&Quantity> for f64 {
    type Output = Quantity;

    fn div(self, rhs: &Quantity) -> Quantity {
        Quantity::new(self / rhs.value, rhs.unit.inv())
    }
}

// -Quantity
impl Neg for Quantity {
    type Output = Quantity;

    fn neg(self) -> Quantity {
        Quantity::new(-self.value, self.unit)
    }
}

impl Neg for &Quantity {
    type Output = Quantity;

    fn neg(self) -> Quantity {
        Quantity::new(-self.value, self.unit.clone())
    }
}

// f64 * Unit -> Quantity (the main way to create quantities)
impl Mul<Unit> for f64 {
    type Output = Quantity;

    fn mul(self, unit: Unit) -> Quantity {
        Quantity::new(self, unit)
    }
}

impl Mul<&Unit> for f64 {
    type Output = Quantity;

    fn mul(self, unit: &Unit) -> Quantity {
        Quantity::new(self, unit.clone())
    }
}

// Quantity * Unit
impl Mul<Unit> for Quantity {
    type Output = Quantity;

    fn mul(self, unit: Unit) -> Quantity {
        Quantity::new(self.value, self.unit * unit)
    }
}

impl Mul<&Unit> for Quantity {
    type Output = Quantity;

    fn mul(self, unit: &Unit) -> Quantity {
        Quantity::new(self.value, self.unit * unit)
    }
}

impl Mul<Unit> for &Quantity {
    type Output = Quantity;

    fn mul(self, unit: Unit) -> Quantity {
        Quantity::new(self.value, &self.unit * unit)
    }
}

impl Mul<&Unit> for &Quantity {
    type Output = Quantity;

    fn mul(self, unit: &Unit) -> Quantity {
        Quantity::new(self.value, &self.unit * unit)
    }
}

// Quantity / Unit
impl Div<Unit> for Quantity {
    type Output = Quantity;

    fn div(self, unit: Unit) -> Quantity {
        Quantity::new(self.value, self.unit / unit)
    }
}

impl Div<&Unit> for Quantity {
    type Output = Quantity;

    fn div(self, unit: &Unit) -> Quantity {
        Quantity::new(self.value, self.unit / unit)
    }
}

impl Div<Unit> for &Quantity {
    type Output = Quantity;

    fn div(self, unit: Unit) -> Quantity {
        Quantity::new(self.value, &self.unit / unit)
    }
}

impl Div<&Unit> for &Quantity {
    type Output = Quantity;

    fn div(self, unit: &Unit) -> Quantity {
        Quantity::new(self.value, &self.unit / unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimension::Dimension;
    use crate::unit::base::BaseUnit;

    fn meter() -> Unit {
        Unit::Base(BaseUnit::new("meter", "m", &[], Dimension::LENGTH, 1.0))
    }

    fn kilometer() -> Unit {
        Unit::Base(BaseUnit::new("kilometer", "km", &[], Dimension::LENGTH, 1000.0))
    }

    fn second() -> Unit {
        Unit::Base(BaseUnit::new("second", "s", &[], Dimension::TIME, 1.0))
    }

    #[test]
    fn test_quantity_creation() {
        let q = 5.0 * meter();
        assert_eq!(q.value(), 5.0);
    }

    #[test]
    fn test_quantity_conversion() {
        let q = 1.0 * kilometer();
        let q_m = q.to(&meter()).unwrap();
        assert!((q_m.value() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_quantity_addition() {
        let a = 1.0 * kilometer();
        let b = 500.0 * meter();
        let c = (a + b).unwrap();
        assert!((c.value() - 1.5).abs() < 1e-10); // 1.5 km
    }

    #[test]
    fn test_quantity_subtraction() {
        let a = 1.0 * kilometer();
        let b = 500.0 * meter();
        let c = (a - b).unwrap();
        assert!((c.value() - 0.5).abs() < 1e-10); // 0.5 km
    }

    #[test]
    fn test_quantity_multiplication() {
        let a = 10.0 * meter();
        let b = 5.0 * second();
        let c = a * b;
        assert!((c.value() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_quantity_division() {
        let dist = 100.0 * meter();
        let time = 10.0 * second();
        let speed = dist / time;
        assert!((speed.value() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_incompatible_addition() {
        let a = 1.0 * meter();
        let b = 1.0 * second();
        let result = a + b;
        assert!(matches!(result, Err(UnitError::IncompatibleDimensions { .. })));
    }

    #[test]
    fn test_quantity_display() {
        let q = 5.5 * meter();
        assert_eq!(format!("{}", q), "5.5 m");
    }
}
