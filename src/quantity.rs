//! Physical quantities with values and units.

use crate::dimension::Rational16;
use crate::error::{UnitError, UnitResult};
use crate::unit::base::BaseUnit;
use crate::unit::Unit;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A physical quantity: a numeric value with an associated unit.
///
/// # Arithmetic
///
/// - **Multiplication and division** always succeed and combine dimensions
///   automatically (e.g., `m * m` → `m²`, `m / s` → `m·s⁻¹`).
/// - **Addition and subtraction** via `+` and `-` **panic** if the two
///   quantities have incompatible dimensions. Use [`checked_add`](Quantity::checked_add)
///   and [`checked_sub`](Quantity::checked_sub) for fallible versions that
///   return `Result`.
///
/// # Examples
///
/// ```
/// use iridium_units::prelude::*;
///
/// let distance = 100.0 * M;
/// let time = 9.58 * S;
/// let speed = &distance / &time;
///
/// let speed_kmh = speed.to(&(KM / H)).unwrap();
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
    pub fn to(&self, target: impl Into<Unit>) -> UnitResult<Quantity> {
        let target = target.into();
        if self.unit.dimension() != target.dimension() {
            return Err(UnitError::DimensionMismatch {
                from: self.unit.to_string(),
                to: target.to_string(),
            });
        }
        let si_value = self.unit.to_si(self.value);
        Ok(Quantity::new(target.from_si(si_value), target))
    }

    /// Get the value in a target unit.
    ///
    /// Shorthand for `.to(target)?.value()`.
    pub fn to_value(&self, target: impl Into<Unit>) -> UnitResult<f64> {
        Ok(self.to(target)?.value())
    }

    /// Decompose to SI base units.
    pub fn decompose(&self) -> Quantity {
        Quantity::new(
            self.unit.to_si(self.value),
            Unit::Composite(crate::unit::composite::CompositeUnit::new(
                1.0,
                self.dimension_components(),
            )),
        )
    }

    /// Get the dimension components for the decomposed unit.
    fn dimension_components(&self) -> Vec<crate::unit::composite::UnitComponent> {
        use crate::dimension::{Dimension, Rational16};
        use crate::unit::composite::UnitComponent;

        let dim = self.unit.dimension();
        let mut components = Vec::new();

        let add_if_nonzero =
            |comps: &mut Vec<UnitComponent>, symbol: &str, base_dim: Dimension, exp: Rational16| {
                if !exp.is_zero() {
                    comps.push(UnitComponent::new(symbol, base_dim, 1.0, exp));
                }
            };

        add_if_nonzero(&mut components, "m", Dimension::LENGTH, dim.length);
        add_if_nonzero(&mut components, "s", Dimension::TIME, dim.time);
        add_if_nonzero(&mut components, "kg", Dimension::MASS, dim.mass);
        add_if_nonzero(&mut components, "A", Dimension::CURRENT, dim.current);
        add_if_nonzero(
            &mut components,
            "K",
            Dimension::TEMPERATURE,
            dim.temperature,
        );
        add_if_nonzero(&mut components, "rad", Dimension::ANGLE, dim.angle);
        add_if_nonzero(
            &mut components,
            "sr",
            Dimension::SOLID_ANGLE,
            dim.solid_angle,
        );
        add_if_nonzero(
            &mut components,
            "cd",
            Dimension::LUMINOUS_INTENSITY,
            dim.luminous_intensity,
        );
        add_if_nonzero(&mut components, "mag", Dimension::MAGNITUDE, dim.magnitude);
        add_if_nonzero(&mut components, "mol", Dimension::AMOUNT, dim.amount);
        add_if_nonzero(&mut components, "ph", Dimension::PHOTON, dim.photon);

        components
    }

    /// Raise this quantity to a power.
    ///
    /// Accepts any type convertible to `Rational16`, including `Rational16` itself.
    /// When passing an `i32`, the value must fit in `i16` range.
    ///
    /// Uses `powi` for integer exponents and `powf` for fractional exponents.
    pub fn pow(&self, exp: impl Into<Rational16>) -> Quantity {
        let exp = exp.into();
        let value = if exp.denom == 1 {
            self.value.powi(exp.numer as i32)
        } else {
            self.value.powf(exp.to_f64())
        };
        Quantity::new(value, self.unit.pow(exp))
    }

    /// Take the square root of this quantity.
    pub fn sqrt(&self) -> Quantity {
        Quantity::new(self.value.sqrt(), self.unit.sqrt())
    }

    /// Get the absolute value of this quantity.
    pub fn abs(&self) -> Quantity {
        Quantity::new(self.value.abs(), self.unit.clone())
    }

    // =========================================================================
    // Logarithmic Unit Helpers
    // =========================================================================

    /// Check if this quantity has a logarithmic unit (magnitude dimension).
    ///
    /// Logarithmic units include magnitudes, decibels, and dex.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "logarithmic")]
    /// # fn main() {
    /// use iridium_units::prelude::*;
    /// use iridium_units::systems::logarithmic::MAG;
    ///
    /// let mag = 5.0 * MAG;
    /// assert!(mag.is_logarithmic());
    ///
    /// let length = 10.0 * M;
    /// assert!(!length.is_logarithmic());
    /// # }
    /// # #[cfg(not(feature = "logarithmic"))]
    /// # fn main() {}
    /// ```
    pub fn is_logarithmic(&self) -> bool {
        self.unit.dimension() == crate::dimension::Dimension::MAGNITUDE
    }

    /// Convert a magnitude quantity to a flux ratio.
    ///
    /// Uses the Pogson formula: F/F₀ = 10^(-0.4 * m)
    ///
    /// Returns `Err` if this quantity does not have magnitude dimension.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "logarithmic")]
    /// # fn main() {
    /// use iridium_units::prelude::*;
    /// use iridium_units::systems::logarithmic::MAG;
    ///
    /// let star = 5.0 * MAG;  // 5th magnitude
    /// let flux = star.mag_to_flux_ratio().unwrap();
    /// assert!((flux - 0.01).abs() < 1e-10);  // 1/100 of reference flux
    /// # }
    /// # #[cfg(not(feature = "logarithmic"))]
    /// # fn main() {}
    /// ```
    pub fn mag_to_flux_ratio(&self) -> UnitResult<f64> {
        if !self.is_logarithmic() {
            return Err(UnitError::LogarithmicError(
                "quantity is not a magnitude".to_string(),
            ));
        }
        // Convert to standard magnitude (scale = 1)
        let mag = self.value * self.unit.scale();
        Ok(10.0_f64.powf(-0.4 * mag))
    }

    /// Convert a decibel quantity to a power ratio.
    ///
    /// Uses the formula: P/P₀ = 10^(dB/10)
    ///
    /// Returns `Err` if this quantity does not have magnitude dimension.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "logarithmic")]
    /// # fn main() {
    /// use iridium_units::prelude::*;
    /// use iridium_units::systems::logarithmic::DB;
    ///
    /// let signal = 10.0 * DB;  // 10 dB
    /// let power = signal.db_to_power_ratio().unwrap();
    /// assert!((power - 10.0).abs() < 1e-10);  // 10x power
    /// # }
    /// # #[cfg(not(feature = "logarithmic"))]
    /// # fn main() {}
    /// ```
    pub fn db_to_power_ratio(&self) -> UnitResult<f64> {
        if !self.is_logarithmic() {
            return Err(UnitError::LogarithmicError(
                "quantity is not in decibels".to_string(),
            ));
        }
        let db = self.value * self.unit.scale();
        Ok(10.0_f64.powf(db / 10.0))
    }

    /// Convert a dex quantity to a linear ratio.
    ///
    /// Uses the formula: x/x₀ = 10^dex
    ///
    /// Returns `Err` if this quantity does not have magnitude dimension.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "logarithmic")]
    /// # fn main() {
    /// use iridium_units::prelude::*;
    /// use iridium_units::systems::logarithmic::DEX;
    ///
    /// let order = 2.0 * DEX;  // 2 orders of magnitude
    /// let ratio = order.dex_to_ratio().unwrap();
    /// assert!((ratio - 100.0).abs() < 1e-10);  // factor of 100
    /// # }
    /// # #[cfg(not(feature = "logarithmic"))]
    /// # fn main() {}
    /// ```
    pub fn dex_to_ratio(&self) -> UnitResult<f64> {
        if !self.is_logarithmic() {
            return Err(UnitError::LogarithmicError(
                "quantity is not in dex".to_string(),
            ));
        }
        let dex = self.value * self.unit.scale();
        Ok(10.0_f64.powf(dex))
    }
}

// =============================================================================
// Checked Arithmetic
// =============================================================================

impl Quantity {
    /// Add two quantities, returning an error if their dimensions don't match.
    ///
    /// This is the fallible version of the `+` operator.
    ///
    /// ```
    /// use iridium_units::prelude::*;
    ///
    /// let a = 1.0 * KM;
    /// let b = 500.0 * M;
    /// let c = a.checked_add(&b).unwrap();
    /// assert!((c.value() - 1.5).abs() < 1e-10);
    /// ```
    pub fn checked_add(&self, rhs: &Quantity) -> UnitResult<Quantity> {
        if self.unit.dimension() != rhs.unit.dimension() {
            return Err(UnitError::IncompatibleDimensions {
                lhs: self.unit.to_string(),
                rhs: rhs.unit.to_string(),
            });
        }
        let rhs_converted = self.unit.from_si(rhs.unit.to_si(rhs.value));
        Ok(Quantity::new(self.value + rhs_converted, self.unit.clone()))
    }

    /// Subtract two quantities, returning an error if their dimensions don't match.
    ///
    /// This is the fallible version of the `-` operator.
    ///
    /// ```
    /// use iridium_units::prelude::*;
    ///
    /// let a = 1.0 * KM;
    /// let b = 500.0 * M;
    /// let c = a.checked_sub(&b).unwrap();
    /// assert!((c.value() - 0.5).abs() < 1e-10);
    /// ```
    pub fn checked_sub(&self, rhs: &Quantity) -> UnitResult<Quantity> {
        if self.unit.dimension() != rhs.unit.dimension() {
            return Err(UnitError::IncompatibleDimensions {
                lhs: self.unit.to_string(),
                rhs: rhs.unit.to_string(),
            });
        }
        let rhs_converted = self.unit.from_si(rhs.unit.to_si(rhs.value));
        Ok(Quantity::new(self.value - rhs_converted, self.unit.clone()))
    }
}

// =============================================================================
// Batch Conversion API
// =============================================================================

/// Convert a slice of values from one unit to another.
///
/// This is more efficient than creating individual `Quantity` objects when
/// processing large datasets, as it computes the conversion factor once
/// and applies it to all values.
///
/// Accepts any `impl Into<Unit>`, including `BaseUnit`, `&BaseUnit`, `Unit`,
/// and `&Unit`. When passing a borrowed unit, it is cloned once (amortized
/// over the batch). Pass an owned value to avoid the clone.
///
/// Note: This uses a single multiplicative factor, so it does not support
/// offset units like Celsius or Fahrenheit. Use [`Quantity::to()`] for those.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::quantity::batch_convert;
///
/// let distances_km = vec![1.0, 2.0, 3.0, 100.0, 42.195];
/// let distances_m = batch_convert(&distances_km, &KM, &M).unwrap();
/// assert!((distances_m[0] - 1000.0).abs() < 1e-10);
/// assert!((distances_m[4] - 42195.0).abs() < 1e-10);
/// ```
pub fn batch_convert(
    values: &[f64],
    from: impl Into<Unit>,
    to: impl Into<Unit>,
) -> UnitResult<Vec<f64>> {
    let from = from.into();
    let to = to.into();
    let factor = from.conversion_factor(&to)?;
    Ok(values.iter().map(|v| v * factor).collect())
}

/// Convert a slice of values from one unit to another, writing into a pre-allocated buffer.
///
/// This variant avoids allocation when the output buffer already exists.
/// Returns `Err` if the units have incompatible dimensions, if either unit
/// has an additive offset, or if the output slice length doesn't match the input.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::quantity::batch_convert_into;
///
/// let distances_km = [1.0, 2.0, 3.0];
/// let mut distances_m = [0.0; 3];
/// batch_convert_into(&distances_km, &KM, &M, &mut distances_m).unwrap();
/// assert!((distances_m[0] - 1000.0).abs() < 1e-10);
/// ```
pub fn batch_convert_into(
    values: &[f64],
    from: impl Into<Unit>,
    to: impl Into<Unit>,
    out: &mut [f64],
) -> UnitResult<()> {
    if values.len() != out.len() {
        return Err(UnitError::BatchError(format!(
            "input length {} doesn't match output length {}",
            values.len(),
            out.len()
        )));
    }
    let from = from.into();
    let to = to.into();
    let factor = from.conversion_factor(&to)?;
    for (i, v) in values.iter().enumerate() {
        out[i] = v * factor;
    }
    Ok(())
}

/// Get the conversion factor between two units for manual batch operations.
///
/// This is useful when you need to apply the conversion factor yourself,
/// such as in SIMD operations or when working with external array libraries.
///
/// Does not support offset units (Celsius, Fahrenheit). Use [`Quantity::to()`]
/// for those.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::quantity::conversion_factor;
///
/// let factor = conversion_factor(&KM, &M).unwrap();
/// assert!((factor - 1000.0).abs() < 1e-10);
///
/// // Apply manually to any data structure
/// let value_km = 5.0;
/// let value_m = value_km * factor;
/// ```
pub fn conversion_factor(from: impl Into<Unit>, to: impl Into<Unit>) -> UnitResult<f64> {
    let from = from.into();
    let to = to.into();
    from.conversion_factor(&to)
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
        let self_si = self.unit.to_si(self.value);
        let other_si = other.unit.to_si(other.value);
        (self_si - other_si).abs() < 1e-15 * self_si.abs().max(other_si.abs()).max(1e-15)
    }
}

// Quantity + Quantity
//
// Panics on dimension mismatch. Use `checked_add` for a fallible version.
impl Add for Quantity {
    type Output = Quantity;

    fn add(self, rhs: Quantity) -> Quantity {
        if self.unit.dimension() != rhs.unit.dimension() {
            panic!(
                "{}",
                UnitError::IncompatibleDimensions {
                    lhs: self.unit.to_string(),
                    rhs: rhs.unit.to_string(),
                }
            );
        }
        let rhs_converted = self.unit.from_si(rhs.unit.to_si(rhs.value));
        Quantity::new(self.value + rhs_converted, self.unit)
    }
}

impl Add for &Quantity {
    type Output = Quantity;

    fn add(self, rhs: &Quantity) -> Quantity {
        self.checked_add(rhs).unwrap_or_else(|e| panic!("{}", e))
    }
}

// Quantity - Quantity
//
// Panics on dimension mismatch. Use `checked_sub` for a fallible version.
impl Sub for Quantity {
    type Output = Quantity;

    fn sub(self, rhs: Quantity) -> Quantity {
        if self.unit.dimension() != rhs.unit.dimension() {
            panic!(
                "{}",
                UnitError::IncompatibleDimensions {
                    lhs: self.unit.to_string(),
                    rhs: rhs.unit.to_string(),
                }
            );
        }
        let rhs_converted = self.unit.from_si(rhs.unit.to_si(rhs.value));
        Quantity::new(self.value - rhs_converted, self.unit)
    }
}

impl Sub for &Quantity {
    type Output = Quantity;

    fn sub(self, rhs: &Quantity) -> Quantity {
        self.checked_sub(rhs).unwrap_or_else(|e| panic!("{}", e))
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

// f64 * BaseUnit → Quantity
impl Mul<BaseUnit> for f64 {
    type Output = Quantity;

    fn mul(self, unit: BaseUnit) -> Quantity {
        Quantity::new(self, Unit::from(unit))
    }
}

// BaseUnit * f64 → Quantity
impl Mul<f64> for BaseUnit {
    type Output = Quantity;

    fn mul(self, value: f64) -> Quantity {
        Quantity::new(value, Unit::from(self))
    }
}

// Quantity * BaseUnit
impl Mul<BaseUnit> for Quantity {
    type Output = Quantity;

    fn mul(self, unit: BaseUnit) -> Quantity {
        self * Unit::from(unit)
    }
}

impl Mul<BaseUnit> for &Quantity {
    type Output = Quantity;

    fn mul(self, unit: BaseUnit) -> Quantity {
        Quantity::new(self.value, &self.unit * unit)
    }
}

// Quantity / BaseUnit
impl Div<BaseUnit> for Quantity {
    type Output = Quantity;

    fn div(self, unit: BaseUnit) -> Quantity {
        self / Unit::from(unit)
    }
}

impl Div<BaseUnit> for &Quantity {
    type Output = Quantity;

    fn div(self, unit: BaseUnit) -> Quantity {
        Quantity::new(self.value, &self.unit / unit)
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
        Unit::Base(BaseUnit::new(
            "kilometer",
            "km",
            &[],
            Dimension::LENGTH,
            1000.0,
        ))
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
        let c = a + b;
        assert!((c.value() - 1.5).abs() < 1e-10); // 1.5 km
    }

    #[test]
    fn test_quantity_subtraction() {
        let a = 1.0 * kilometer();
        let b = 500.0 * meter();
        let c = a - b;
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
    #[should_panic(expected = "cannot add/subtract quantities with different dimensions")]
    fn test_incompatible_addition() {
        let a = 1.0 * meter();
        let b = 1.0 * second();
        let _ = a + b;
    }

    #[test]
    fn test_checked_add_incompatible() {
        let a = 1.0 * meter();
        let b = 1.0 * second();
        let result = a.checked_add(&b);
        assert!(matches!(
            result,
            Err(UnitError::IncompatibleDimensions { .. })
        ));
    }

    #[test]
    fn test_checked_sub_incompatible() {
        let a = 1.0 * meter();
        let b = 1.0 * second();
        let result = a.checked_sub(&b);
        assert!(matches!(
            result,
            Err(UnitError::IncompatibleDimensions { .. })
        ));
    }

    #[test]
    fn test_quantity_display() {
        let q = 5.5 * meter();
        assert_eq!(format!("{}", q), "5.5 m");
    }

    #[test]
    fn test_ref_addition_no_clone() {
        // Test that reference addition works correctly
        let a = 1.0 * kilometer();
        let b = 500.0 * meter();
        let c = &a + &b;
        assert!((c.value() - 1.5).abs() < 1e-10); // 1.5 km
                                                  // Original values should still be accessible
        assert!((a.value() - 1.0).abs() < 1e-10);
        assert!((b.value() - 500.0).abs() < 1e-10);
    }

    #[test]
    fn test_ref_subtraction_no_clone() {
        // Test that reference subtraction works correctly
        let a = 1.0 * kilometer();
        let b = 500.0 * meter();
        let c = &a - &b;
        assert!((c.value() - 0.5).abs() < 1e-10); // 0.5 km
    }

    #[test]
    fn test_batch_convert() {
        use super::batch_convert;
        let values = vec![1.0, 2.0, 3.0, 100.0];
        let converted = batch_convert(&values, &kilometer(), &meter()).unwrap();
        assert_eq!(converted.len(), 4);
        assert!((converted[0] - 1000.0).abs() < 1e-10);
        assert!((converted[1] - 2000.0).abs() < 1e-10);
        assert!((converted[2] - 3000.0).abs() < 1e-10);
        assert!((converted[3] - 100000.0).abs() < 1e-10);
    }

    #[test]
    fn test_batch_convert_into() {
        use super::batch_convert_into;
        let values = [1.0, 2.0, 3.0];
        let mut out = [0.0; 3];
        batch_convert_into(&values, &kilometer(), &meter(), &mut out).unwrap();
        assert!((out[0] - 1000.0).abs() < 1e-10);
        assert!((out[1] - 2000.0).abs() < 1e-10);
        assert!((out[2] - 3000.0).abs() < 1e-10);
    }

    #[test]
    fn test_batch_convert_into_length_mismatch() {
        use super::batch_convert_into;
        let values = [1.0, 2.0, 3.0];
        let mut out = [0.0; 2]; // Wrong length
        let result = batch_convert_into(&values, &kilometer(), &meter(), &mut out);
        assert!(result.is_err());
    }

    #[test]
    fn test_batch_convert_incompatible_units() {
        use super::batch_convert;
        let values = vec![1.0, 2.0];
        let result = batch_convert(&values, &meter(), &second());
        assert!(result.is_err());
    }

    #[test]
    fn test_conversion_factor() {
        use super::conversion_factor;
        let factor = conversion_factor(&kilometer(), &meter()).unwrap();
        assert!((factor - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_pow_integer() {
        let m = 3.0 * meter();
        let m2 = m.pow(2);
        assert!((m2.value() - 9.0).abs() < 1e-10);
        assert_eq!(
            m2.unit().dimension(),
            Dimension {
                length: Rational16::new(2, 1),
                ..Dimension::DIMENSIONLESS
            }
        );
    }

    #[test]
    fn test_pow_fractional() {
        // sqrt(4 m²) = 2 m
        let area = 4.0 * meter().pow(Rational16::new(2, 1));
        let root = area.pow(Rational16::new(1, 2));
        assert!((root.value() - 2.0).abs() < 1e-10);
        assert_eq!(root.unit().dimension(), Dimension::LENGTH);
    }
}
