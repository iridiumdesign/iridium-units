//! Dimensional analysis types.
//!
//! This module provides the [`Dimension`] type which represents physical dimensions
//! as a product of powers of 11 base dimensions.
//!
//! # Rational Exponents
//!
//! Physical dimensions use rational (fractional) exponents to support operations
//! like square roots. For example:
//!
//! - Velocity has dimension L¹T⁻¹ (length per time)
//! - Energy has dimension M¹L²T⁻² (mass × length² / time²)
//! - Square root of area: √(L²) = L¹ (length^(2 × 1/2) = length^1)
//!
//! The [`Rational16`] type stores these exponents as exact fractions (numerator/denominator)
//! rather than floating-point, ensuring precise dimensional analysis without rounding errors.
//!
//! # Example: Why Rationals Matter
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! // Square root of area (m²) should give length (m)
//! let area_unit = M.pow(Rational16::new(2, 1));  // m²
//! let length_unit = area_unit.pow(Rational16::new(1, 2));  // √(m²) = m
//!
//! // The exponent 2 × 1/2 = 1 exactly (no floating-point error)
//! assert_eq!(length_unit.dimension().length, Rational16::ONE);
//! ```

use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// A rational number (fraction) for exact dimensional exponents.
///
/// Rational numbers are used instead of floating-point to represent dimensional
/// exponents because they provide **exact arithmetic** without rounding errors.
/// This is critical for dimensional analysis where we need to verify that
/// dimensions match exactly.
///
/// # Why Rationals Instead of Floats?
///
/// Consider checking if two dimensions are equal:
/// - With floats: `0.5 + 0.5 == 1.0` might fail due to rounding
/// - With rationals: `1/2 + 1/2 == 1/1` is always exactly true
///
/// # Storage
///
/// Uses `i16` for numerator and denominator, providing:
/// - Range: -32,768 to 32,767 for both numerator and denominator
/// - Intermediate calculations use `i32` to prevent overflow
/// - Automatically normalized to lowest terms (e.g., 2/4 → 1/2)
/// - Denominator is always positive after normalization
///
/// # Common Values
///
/// | Exponent | Rational | Meaning |
/// |----------|----------|---------|
/// | 1 | 1/1 | Linear (e.g., length) |
/// | 2 | 2/1 | Squared (e.g., area) |
/// | -1 | -1/1 | Inverse (e.g., frequency = 1/time) |
/// | -2 | -2/1 | Inverse squared (e.g., acceleration = length/time²) |
/// | 1/2 | 1/2 | Square root |
/// | 1/3 | 1/3 | Cube root |
/// | 3/2 | 3/2 | Square root of cube (e.g., Kepler's third law) |
///
/// # Example
///
/// ```
/// use iridium_units::Rational16;
///
/// let half = Rational16::new(1, 2);
/// let third = Rational16::new(1, 3);
///
/// // Addition: 1/2 + 1/3 = 5/6
/// let sum = half + third;
/// assert_eq!(sum.numer, 5);
/// assert_eq!(sum.denom, 6);
///
/// // Multiplication: 1/2 × 1/3 = 1/6
/// let product = half * third;
/// assert_eq!(product.numer, 1);
/// assert_eq!(product.denom, 6);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rational16 {
    /// Numerator (can be negative)
    pub numer: i16,
    /// Denominator (always positive after normalization)
    pub denom: i16,
}

impl Rational16 {
    /// Zero
    pub const ZERO: Rational16 = Rational16 { numer: 0, denom: 1 };

    /// One
    pub const ONE: Rational16 = Rational16 { numer: 1, denom: 1 };

    /// Create a new rational number, automatically normalized.
    ///
    /// # Panics
    ///
    /// Panics if `denom` is zero.
    pub const fn new(numer: i16, denom: i16) -> Self {
        if denom == 0 {
            panic!("denominator cannot be zero");
        }
        if numer == 0 {
            return Rational16 { numer: 0, denom: 1 };
        }
        // Ensure positive denominator
        let (n, d) = if denom < 0 {
            (if numer == i16::MIN { i16::MAX } else { -numer },
             if denom == i16::MIN { i16::MAX } else { -denom })
        } else {
            (numer, denom)
        };
        // Absolute values for GCD (cast via i32 to handle i16::MIN)
        let abs_n = (-(n as i32 * ((n < 0) as i32 * 2 - 1))) as u16;
        let abs_d = d as u16;
        let g = gcd(abs_n, abs_d);
        Rational16 { numer: n / (g as i16), denom: d / (g as i16) }
    }

    /// Create a new rational number, returning an error if the denominator is zero.
    pub fn checked_new(numer: i16, denom: i16) -> Result<Self, crate::error::UnitError> {
        if denom == 0 {
            return Err(crate::error::UnitError::ZeroDenominator);
        }
        Ok(Self::new(numer, denom))
    }

    /// Check if this rational is zero.
    pub const fn is_zero(&self) -> bool {
        self.numer == 0
    }

    /// Convert to f64.
    pub fn to_f64(self) -> f64 {
        self.numer as f64 / self.denom as f64
    }

    /// Const-compatible addition.
    pub const fn const_add(self, rhs: Self) -> Self {
        let numer = self.numer as i32 * rhs.denom as i32
            + rhs.numer as i32 * self.denom as i32;
        let denom = self.denom as i32 * rhs.denom as i32;
        rational16_from_i32(numer, denom)
    }

    /// Const-compatible negation.
    pub const fn const_neg(self) -> Self {
        Rational16::new(if self.numer == i16::MIN { i16::MAX } else { -self.numer }, self.denom)
    }

    /// Const-compatible subtraction.
    pub const fn const_sub(self, rhs: Self) -> Self {
        self.const_add(rhs.const_neg())
    }

    /// Const-compatible multiplication.
    pub const fn const_mul(self, rhs: Self) -> Self {
        let numer = self.numer as i32 * rhs.numer as i32;
        let denom = self.denom as i32 * rhs.denom as i32;
        rational16_from_i32(numer, denom)
    }
}

/// Greatest common divisor using Euclidean algorithm.
const fn gcd(mut a: u16, mut b: u16) -> u16 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 { 1 } else { a }
}

/// GCD for i32 values (used in overflow-safe arithmetic).
const fn gcd_i32(mut a: i32, mut b: i32) -> i32 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 { 1 } else { a }
}

/// Create a Rational16 from i32 numerator and denominator, reducing first
/// and panicking if the reduced result doesn't fit in i16.
const fn rational16_from_i32(numer: i32, denom: i32) -> Rational16 {
    if denom == 0 {
        panic!("denominator cannot be zero in dimensional arithmetic");
    }

    // Normalize sign: ensure positive denominator
    let (numer, denom) = if denom < 0 {
        (-numer, -denom)
    } else {
        (numer, denom)
    };

    // Reduce before casting to i16
    let g = gcd_i32(numer, denom);
    let numer = numer / g;
    let denom = denom / g;

    if numer < i16::MIN as i32 || numer > i16::MAX as i32 {
        panic!("dimension exponent overflow: numerator does not fit in i16");
    }
    if denom < i16::MIN as i32 || denom > i16::MAX as i32 {
        panic!("dimension exponent overflow: denominator does not fit in i16");
    }

    Rational16 { numer: numer as i16, denom: denom as i16 }
}

impl Add for Rational16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // a/b + c/d = (ad + bc) / bd
        // Use i32 for intermediate calculations, reduce before casting back
        let numer = self.numer as i32 * rhs.denom as i32
            + rhs.numer as i32 * self.denom as i32;
        let denom = self.denom as i32 * rhs.denom as i32;
        rational16_from_i32(numer, denom)
    }
}

impl Sub for Rational16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Neg for Rational16 {
    type Output = Self;

    fn neg(self) -> Self {
        Rational16::new(-self.numer, self.denom)
    }
}

impl Mul for Rational16 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // Use i32 for intermediate calculations, reduce before casting back
        let numer = self.numer as i32 * rhs.numer as i32;
        let denom = self.denom as i32 * rhs.denom as i32;
        rational16_from_i32(numer, denom)
    }
}

impl Mul<i8> for Rational16 {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self {
        Rational16::new(self.numer * rhs as i16, self.denom)
    }
}

impl fmt::Debug for Rational16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{}/{}", self.numer, self.denom)
        }
    }
}

impl fmt::Display for Rational16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{}/{}", self.numer, self.denom)
        }
    }
}

impl From<i8> for Rational16 {
    fn from(n: i8) -> Self {
        Rational16::new(n as i16, 1)
    }
}

impl From<i16> for Rational16 {
    fn from(n: i16) -> Self {
        Rational16::new(n, 1)
    }
}

impl From<i32> for Rational16 {
    fn from(n: i32) -> Self {
        let numer = i16::try_from(n).unwrap_or_else(|_| {
            panic!("value {} does not fit in Rational16 (i16 range)", n)
        });
        Rational16::new(numer, 1)
    }
}

/// Represents the dimensional exponents for 11 base physical dimensions.
///
/// Each field stores the rational power of that base dimension.
/// For example, velocity (m/s) has length=1, time=-1, all others=0.
///
/// The 11 base dimensions are:
/// - length, time, mass, current, temperature (SI base)
/// - angle, solid_angle (angular)
/// - luminous_intensity, magnitude (photometric)
/// - amount (moles), photon (photon count)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Dimension {
    pub length: Rational16,
    pub time: Rational16,
    pub mass: Rational16,
    pub current: Rational16,
    pub temperature: Rational16,
    pub angle: Rational16,
    pub solid_angle: Rational16,
    pub luminous_intensity: Rational16,
    pub magnitude: Rational16,
    pub amount: Rational16,
    pub photon: Rational16,
}

impl Dimension {
    /// Dimensionless (all exponents zero).
    pub const DIMENSIONLESS: Dimension = Dimension {
        length: Rational16::ZERO,
        time: Rational16::ZERO,
        mass: Rational16::ZERO,
        current: Rational16::ZERO,
        temperature: Rational16::ZERO,
        angle: Rational16::ZERO,
        solid_angle: Rational16::ZERO,
        luminous_intensity: Rational16::ZERO,
        magnitude: Rational16::ZERO,
        amount: Rational16::ZERO,
        photon: Rational16::ZERO,
    };

    /// Length dimension (L).
    pub const LENGTH: Dimension = Dimension {
        length: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Time dimension (T).
    pub const TIME: Dimension = Dimension {
        time: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Mass dimension (M).
    pub const MASS: Dimension = Dimension {
        mass: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Electric current dimension (I).
    pub const CURRENT: Dimension = Dimension {
        current: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Temperature dimension (Θ).
    pub const TEMPERATURE: Dimension = Dimension {
        temperature: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Angle dimension.
    pub const ANGLE: Dimension = Dimension {
        angle: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Solid angle dimension.
    pub const SOLID_ANGLE: Dimension = Dimension {
        solid_angle: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Luminous intensity dimension (J).
    pub const LUMINOUS_INTENSITY: Dimension = Dimension {
        luminous_intensity: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Magnitude dimension (for stellar magnitudes).
    pub const MAGNITUDE: Dimension = Dimension {
        magnitude: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Amount of substance dimension (N).
    pub const AMOUNT: Dimension = Dimension {
        amount: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Photon count dimension.
    pub const PHOTON: Dimension = Dimension {
        photon: Rational16::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Check if this dimension is dimensionless (all exponents zero).
    pub const fn is_dimensionless(&self) -> bool {
        self.length.numer == 0
            && self.time.numer == 0
            && self.mass.numer == 0
            && self.current.numer == 0
            && self.temperature.numer == 0
            && self.angle.numer == 0
            && self.solid_angle.numer == 0
            && self.luminous_intensity.numer == 0
            && self.magnitude.numer == 0
            && self.amount.numer == 0
            && self.photon.numer == 0
    }

    /// Multiply dimensions (add exponents).
    pub const fn mul(&self, other: &Dimension) -> Dimension {
        Dimension {
            length: self.length.const_add(other.length),
            time: self.time.const_add(other.time),
            mass: self.mass.const_add(other.mass),
            current: self.current.const_add(other.current),
            temperature: self.temperature.const_add(other.temperature),
            angle: self.angle.const_add(other.angle),
            solid_angle: self.solid_angle.const_add(other.solid_angle),
            luminous_intensity: self.luminous_intensity.const_add(other.luminous_intensity),
            magnitude: self.magnitude.const_add(other.magnitude),
            amount: self.amount.const_add(other.amount),
            photon: self.photon.const_add(other.photon),
        }
    }

    /// Divide dimensions (subtract exponents).
    pub const fn div(&self, other: &Dimension) -> Dimension {
        Dimension {
            length: self.length.const_sub(other.length),
            time: self.time.const_sub(other.time),
            mass: self.mass.const_sub(other.mass),
            current: self.current.const_sub(other.current),
            temperature: self.temperature.const_sub(other.temperature),
            angle: self.angle.const_sub(other.angle),
            solid_angle: self.solid_angle.const_sub(other.solid_angle),
            luminous_intensity: self.luminous_intensity.const_sub(other.luminous_intensity),
            magnitude: self.magnitude.const_sub(other.magnitude),
            amount: self.amount.const_sub(other.amount),
            photon: self.photon.const_sub(other.photon),
        }
    }

    /// Raise dimension to a power (multiply all exponents).
    pub const fn pow(&self, power: Rational16) -> Dimension {
        Dimension {
            length: self.length.const_mul(power),
            time: self.time.const_mul(power),
            mass: self.mass.const_mul(power),
            current: self.current.const_mul(power),
            temperature: self.temperature.const_mul(power),
            angle: self.angle.const_mul(power),
            solid_angle: self.solid_angle.const_mul(power),
            luminous_intensity: self.luminous_intensity.const_mul(power),
            magnitude: self.magnitude.const_mul(power),
            amount: self.amount.const_mul(power),
            photon: self.photon.const_mul(power),
        }
    }

    /// Invert dimension (negate all exponents).
    pub const fn inv(&self) -> Dimension {
        self.pow(Rational16::new(-1, 1))
    }
}

impl fmt::Debug for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dimensionless() {
            return write!(f, "dimensionless");
        }

        let mut parts = Vec::new();
        let dims = [
            ("L", self.length),
            ("T", self.time),
            ("M", self.mass),
            ("I", self.current),
            ("Θ", self.temperature),
            ("A", self.angle),
            ("Ω", self.solid_angle),
            ("J", self.luminous_intensity),
            ("mag", self.magnitude),
            ("N", self.amount),
            ("ph", self.photon),
        ];

        for (name, exp) in dims {
            if !exp.is_zero() {
                if exp == Rational16::ONE {
                    parts.push(name.to_string());
                } else {
                    parts.push(format!("{}^{}", name, exp));
                }
            }
        }

        write!(f, "{}", parts.join(" "))
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational_basic() {
        let r = Rational16::new(2, 4);
        assert_eq!(r.numer, 1);
        assert_eq!(r.denom, 2);
    }

    #[test]
    fn test_rational_negative_denom() {
        let r = Rational16::new(1, -2);
        assert_eq!(r.numer, -1);
        assert_eq!(r.denom, 2);
    }

    #[test]
    fn test_rational_add() {
        let a = Rational16::new(1, 2);
        let b = Rational16::new(1, 3);
        let c = a + b;
        assert_eq!(c.numer, 5);
        assert_eq!(c.denom, 6);
    }

    #[test]
    fn test_dimension_velocity() {
        let velocity = Dimension::LENGTH.div(&Dimension::TIME);
        assert_eq!(velocity.length, Rational16::ONE);
        assert_eq!(velocity.time, Rational16::new(-1, 1));
    }

    #[test]
    fn test_dimension_energy() {
        // Energy = M L^2 T^-2
        let energy = Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
        assert_eq!(energy.mass, Rational16::ONE);
        assert_eq!(energy.length, Rational16::new(2, 1));
        assert_eq!(energy.time, Rational16::new(-2, 1));
    }

    #[test]
    fn test_dimensionless() {
        let d = Dimension::LENGTH.div(&Dimension::LENGTH);
        assert!(d.is_dimensionless());
    }

    #[test]
    fn test_rational_large_denominators() {
        // This test would have caused overflow with i8 (12 * 12 = 144 > 127)
        // but works correctly with i16
        let a = Rational16::new(1, 12);
        let b = Rational16::new(1, 12);
        let c = a + b;
        // 1/12 + 1/12 = 2/12 = 1/6
        assert_eq!(c.numer, 1);
        assert_eq!(c.denom, 6);
    }

    #[test]
    fn test_rational_multiply_large() {
        // Test multiplication that would overflow i8
        let a = Rational16::new(50, 1);
        let b = Rational16::new(50, 1);
        let c = a * b;
        // 50 * 50 = 2500, which overflows i8 but fits in i16
        assert_eq!(c.numer, 2500);
        assert_eq!(c.denom, 1);
    }

    #[test]
    fn test_rational_complex_fraction() {
        // Test with denominators that would overflow when multiplied in i8
        let a = Rational16::new(1, 15);
        let b = Rational16::new(1, 15);
        let c = a * b;
        // 1/15 * 1/15 = 1/225
        assert_eq!(c.numer, 1);
        assert_eq!(c.denom, 225);
    }

    #[test]
    fn test_dimension_high_power() {
        // Test dimension with larger exponents
        let high_power = Dimension::LENGTH.pow(Rational16::new(100, 1));
        assert_eq!(high_power.length.numer, 100);
        assert_eq!(high_power.length.denom, 1);
    }

    // Boundary tests for Rational16 overflow safety (#7)

    #[test]
    fn test_rational_add_reduces_before_cast() {
        // 1/200 + 1/200: intermediate denom = 200*200 = 40000 > i16::MAX
        // intermediate numer = 1*200 + 1*200 = 400, so 400/40000 reduces to 1/100, which fits
        let a = Rational16::new(1, 200);
        let b = Rational16::new(1, 200);
        let c = a + b;
        assert_eq!(c.numer, 1);
        assert_eq!(c.denom, 100);
    }

    #[test]
    #[should_panic(expected = "dimension exponent overflow")]
    fn test_rational_add_overflow_panics() {
        // Two values whose sum genuinely can't fit after reduction
        let a = Rational16::new(i16::MAX, 1);
        let b = Rational16::new(i16::MAX, 1);
        let _ = a + b;
    }

    #[test]
    #[should_panic(expected = "dimension exponent overflow")]
    fn test_rational_mul_overflow_panics() {
        // i16::MAX * 2 = 65534, doesn't fit in i16
        let a = Rational16::new(i16::MAX, 1);
        let b = Rational16::new(2, 1);
        let _ = a * b;
    }

    #[test]
    fn test_rational_mul_reduces_before_cast() {
        // 200/1 * 1/200 = 200/200 = 1/1 (reduces before cast)
        let a = Rational16::new(200, 1);
        let b = Rational16::new(1, 200);
        let c = a * b;
        assert_eq!(c.numer, 1);
        assert_eq!(c.denom, 1);
    }

    #[test]
    #[should_panic(expected = "does not fit")]
    fn test_rational_from_i32_overflow_panics() {
        let _ = Rational16::from(50000i32);
    }

    #[test]
    fn test_checked_new_zero_denom() {
        let result = Rational16::checked_new(1, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_checked_new_valid() {
        let result = Rational16::checked_new(3, 6);
        assert!(result.is_ok());
        let r = result.unwrap();
        assert_eq!(r.numer, 1);
        assert_eq!(r.denom, 2);
    }
}
