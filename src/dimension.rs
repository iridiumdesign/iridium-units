//! Dimensional analysis types.
//!
//! This module provides the [`Dimension`] type which represents physical dimensions
//! as a product of powers of 11 base dimensions, following AstroPy's model.

use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// A compact rational number using i8 for numerator and denominator.
///
/// Used to represent dimensional exponents, which can be fractional
/// (e.g., sqrt(meter) has length exponent 1/2).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rational8 {
    /// Numerator
    pub numer: i8,
    /// Denominator (always positive after normalization)
    pub denom: i8,
}

impl Rational8 {
    /// Zero
    pub const ZERO: Rational8 = Rational8 { numer: 0, denom: 1 };

    /// One
    pub const ONE: Rational8 = Rational8 { numer: 1, denom: 1 };

    /// Create a new rational number, automatically normalized.
    pub fn new(numer: i8, denom: i8) -> Self {
        if denom == 0 {
            panic!("denominator cannot be zero");
        }
        let mut r = Rational8 { numer, denom };
        r.normalize();
        r
    }

    /// Normalize the rational (reduce to lowest terms, ensure positive denominator).
    fn normalize(&mut self) {
        if self.numer == 0 {
            self.denom = 1;
            return;
        }

        // Ensure positive denominator
        if self.denom < 0 {
            self.numer = self.numer.saturating_neg();
            self.denom = self.denom.saturating_neg();
        }

        // Reduce to lowest terms
        let g = gcd(self.numer.unsigned_abs(), self.denom.unsigned_abs());
        self.numer /= g as i8;
        self.denom /= g as i8;
    }

    /// Check if this rational is zero.
    pub fn is_zero(&self) -> bool {
        self.numer == 0
    }

    /// Convert to f64.
    pub fn to_f64(self) -> f64 {
        self.numer as f64 / self.denom as f64
    }
}

/// Greatest common divisor using Euclidean algorithm.
fn gcd(mut a: u8, mut b: u8) -> u8 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

impl Add for Rational8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // a/b + c/d = (ad + bc) / bd
        let numer = (self.numer as i16 * rhs.denom as i16
            + rhs.numer as i16 * self.denom as i16) as i8;
        let denom = (self.denom as i16 * rhs.denom as i16) as i8;
        Rational8::new(numer, denom)
    }
}

impl Sub for Rational8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Neg for Rational8 {
    type Output = Self;

    fn neg(self) -> Self {
        Rational8::new(-self.numer, self.denom)
    }
}

impl Mul for Rational8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let numer = (self.numer as i16 * rhs.numer as i16) as i8;
        let denom = (self.denom as i16 * rhs.denom as i16) as i8;
        Rational8::new(numer, denom)
    }
}

impl Mul<i8> for Rational8 {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self {
        Rational8::new(self.numer * rhs, self.denom)
    }
}

impl fmt::Debug for Rational8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{}/{}", self.numer, self.denom)
        }
    }
}

impl fmt::Display for Rational8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{}/{}", self.numer, self.denom)
        }
    }
}

impl From<i8> for Rational8 {
    fn from(n: i8) -> Self {
        Rational8::new(n, 1)
    }
}

impl From<i32> for Rational8 {
    fn from(n: i32) -> Self {
        Rational8::new(n as i8, 1)
    }
}

/// Represents the dimensional exponents for 11 base physical dimensions.
///
/// Each field stores the rational power of that base dimension.
/// For example, velocity (m/s) has length=1, time=-1, all others=0.
///
/// The 11 base dimensions follow AstroPy's model:
/// - length, time, mass, current, temperature (SI base)
/// - angle, solid_angle (angular)
/// - luminous_intensity, magnitude (photometric)
/// - amount (moles), photon (photon count)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Dimension {
    pub length: Rational8,
    pub time: Rational8,
    pub mass: Rational8,
    pub current: Rational8,
    pub temperature: Rational8,
    pub angle: Rational8,
    pub solid_angle: Rational8,
    pub luminous_intensity: Rational8,
    pub magnitude: Rational8,
    pub amount: Rational8,
    pub photon: Rational8,
}

impl Dimension {
    /// Dimensionless (all exponents zero).
    pub const DIMENSIONLESS: Dimension = Dimension {
        length: Rational8::ZERO,
        time: Rational8::ZERO,
        mass: Rational8::ZERO,
        current: Rational8::ZERO,
        temperature: Rational8::ZERO,
        angle: Rational8::ZERO,
        solid_angle: Rational8::ZERO,
        luminous_intensity: Rational8::ZERO,
        magnitude: Rational8::ZERO,
        amount: Rational8::ZERO,
        photon: Rational8::ZERO,
    };

    /// Length dimension (L).
    pub const LENGTH: Dimension = Dimension {
        length: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Time dimension (T).
    pub const TIME: Dimension = Dimension {
        time: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Mass dimension (M).
    pub const MASS: Dimension = Dimension {
        mass: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Electric current dimension (I).
    pub const CURRENT: Dimension = Dimension {
        current: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Temperature dimension (Θ).
    pub const TEMPERATURE: Dimension = Dimension {
        temperature: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Angle dimension.
    pub const ANGLE: Dimension = Dimension {
        angle: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Solid angle dimension.
    pub const SOLID_ANGLE: Dimension = Dimension {
        solid_angle: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Luminous intensity dimension (J).
    pub const LUMINOUS_INTENSITY: Dimension = Dimension {
        luminous_intensity: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Magnitude dimension (for stellar magnitudes).
    pub const MAGNITUDE: Dimension = Dimension {
        magnitude: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Amount of substance dimension (N).
    pub const AMOUNT: Dimension = Dimension {
        amount: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Photon count dimension.
    pub const PHOTON: Dimension = Dimension {
        photon: Rational8::ONE,
        ..Self::DIMENSIONLESS
    };

    /// Check if this dimension is dimensionless (all exponents zero).
    pub fn is_dimensionless(&self) -> bool {
        *self == Self::DIMENSIONLESS
    }

    /// Multiply dimensions (add exponents).
    pub fn mul(&self, other: &Dimension) -> Dimension {
        Dimension {
            length: self.length + other.length,
            time: self.time + other.time,
            mass: self.mass + other.mass,
            current: self.current + other.current,
            temperature: self.temperature + other.temperature,
            angle: self.angle + other.angle,
            solid_angle: self.solid_angle + other.solid_angle,
            luminous_intensity: self.luminous_intensity + other.luminous_intensity,
            magnitude: self.magnitude + other.magnitude,
            amount: self.amount + other.amount,
            photon: self.photon + other.photon,
        }
    }

    /// Divide dimensions (subtract exponents).
    pub fn div(&self, other: &Dimension) -> Dimension {
        Dimension {
            length: self.length - other.length,
            time: self.time - other.time,
            mass: self.mass - other.mass,
            current: self.current - other.current,
            temperature: self.temperature - other.temperature,
            angle: self.angle - other.angle,
            solid_angle: self.solid_angle - other.solid_angle,
            luminous_intensity: self.luminous_intensity - other.luminous_intensity,
            magnitude: self.magnitude - other.magnitude,
            amount: self.amount - other.amount,
            photon: self.photon - other.photon,
        }
    }

    /// Raise dimension to a power (multiply all exponents).
    pub fn pow(&self, power: Rational8) -> Dimension {
        Dimension {
            length: self.length * power,
            time: self.time * power,
            mass: self.mass * power,
            current: self.current * power,
            temperature: self.temperature * power,
            angle: self.angle * power,
            solid_angle: self.solid_angle * power,
            luminous_intensity: self.luminous_intensity * power,
            magnitude: self.magnitude * power,
            amount: self.amount * power,
            photon: self.photon * power,
        }
    }

    /// Invert dimension (negate all exponents).
    pub fn inv(&self) -> Dimension {
        self.pow(Rational8::new(-1, 1))
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
                if exp == Rational8::ONE {
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
        let r = Rational8::new(2, 4);
        assert_eq!(r.numer, 1);
        assert_eq!(r.denom, 2);
    }

    #[test]
    fn test_rational_negative_denom() {
        let r = Rational8::new(1, -2);
        assert_eq!(r.numer, -1);
        assert_eq!(r.denom, 2);
    }

    #[test]
    fn test_rational_add() {
        let a = Rational8::new(1, 2);
        let b = Rational8::new(1, 3);
        let c = a + b;
        assert_eq!(c.numer, 5);
        assert_eq!(c.denom, 6);
    }

    #[test]
    fn test_dimension_velocity() {
        let velocity = Dimension::LENGTH.div(&Dimension::TIME);
        assert_eq!(velocity.length, Rational8::ONE);
        assert_eq!(velocity.time, Rational8::new(-1, 1));
    }

    #[test]
    fn test_dimension_energy() {
        // Energy = M L^2 T^-2
        let energy = Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1)));
        assert_eq!(energy.mass, Rational8::ONE);
        assert_eq!(energy.length, Rational8::new(2, 1));
        assert_eq!(energy.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_dimensionless() {
        let d = Dimension::LENGTH.div(&Dimension::LENGTH);
        assert!(d.is_dimensionless());
    }
}
