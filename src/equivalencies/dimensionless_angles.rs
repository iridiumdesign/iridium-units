//! Dimensionless angles equivalency.
//!
//! This module provides an equivalency that treats angles (radians, steradians)
//! as dimensionless quantities. This is necessary for many physics equations
//! where angles are mathematically dimensionless but tracked separately for
//! type safety.
//!
//! # Background
//!
//! Angles are geometrically defined as ratios (arc length / radius), making
//! them fundamentally dimensionless. However, tracking angle dimensions
//! separately helps catch errors like adding meters to radians.
//!
//! The trade-off is that some physics equations produce "polluted" dimensions:
//!
//! - Rotational energy: E = ½Iω² gives kg·m²·rad²/s² instead of J (kg·m²/s²)
//! - Angular frequency: ω = 2πf gives rad/s instead of 1/s
//! - Torque × angle: τθ gives N·m·rad instead of J
//!
//! This equivalency allows converting between these representations.
//!
//! # Example
//!
//! ```
//! use iridium_units::prelude::*;
//! use iridium_units::equivalencies::dimensionless_angles;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Rotational kinetic energy
//!     let inertia = 2.0 * KG * M * M;      // Moment of inertia
//!     let omega = 10.0 * RAD / S;             // Angular velocity
//!
//!     let e_with_rad = 0.5 * &inertia * &omega * &omega;  // kg·m²·rad²/s²
//!     let e = e_with_rad.to_equiv(&J, dimensionless_angles())?;  // J
//!     Ok(())
//! }
//! ```

use super::{Converter, Equivalency};
use crate::dimension::{Dimension, Rational16};
use crate::unit::Unit;

/// Create a dimensionless angles equivalency.
///
/// This equivalency treats angle (rad) and solid angle (sr) dimensions as
/// dimensionless (exponent = 0), allowing conversion between units that
/// differ only in their angle dimensions.
///
/// # Supported Conversions
///
/// Any conversion where the source and target units have the same dimensions
/// except for angles/solid angles:
///
/// - `rad/s` ↔ `1/s` (angular frequency ↔ frequency)
/// - `kg·m²·rad²/s²` ↔ `kg·m²/s²` (rotational energy ↔ energy)
/// - `N·m·rad` ↔ `N·m` (work from torque ↔ energy)
/// - `W/sr` ↔ `W` (radiant intensity ↔ power, per solid angle)
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::dimensionless_angles;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Angular frequency to regular frequency
///     // dimensionless_angles treats rad as 1, so 2π rad/s = 2π Hz
///     let omega = 2.0 * std::f64::consts::PI * RAD / S;
///     let f = omega.to_equiv(&HZ, dimensionless_angles())?;
///     assert!((f.value() - 2.0 * std::f64::consts::PI).abs() < 1e-10);
///
///     // Rotational energy
///     let inertia = 1.0 * KG * M * M;
///     let omega = 2.0 * RAD / S;
///     let e = (0.5 * &inertia * &omega * &omega)
///         .to_equiv(&J, dimensionless_angles())?;
///     assert!((e.value() - 2.0).abs() < 1e-10);  // ½ × 1 × 4 = 2 J
///     Ok(())
/// }
/// ```
pub fn dimensionless_angles() -> Equivalency {
    Equivalency::new("dimensionless_angles", |from, to| {
        let from_dim = from.dimension();
        let to_dim = to.dimension();

        // Check if dimensions match when angles are treated as dimensionless
        let from_no_angles = remove_angle_dimensions(&from_dim);
        let to_no_angles = remove_angle_dimensions(&to_dim);

        if from_no_angles != to_no_angles {
            return None;
        }

        // If the dimensions without angles are equal, we can convert
        // The conversion factor accounts for any scale differences
        let from_scale = from.scale();
        let to_scale = to.scale();

        // For angles treated as dimensionless, 1 rad = 1 and 1 sr = 1
        // So we just need to handle the regular scale factors
        let factor = from_scale / to_scale;

        Some(Converter::new_infallible(
            move |x| x * factor,
            move |x| x / factor,
        ))
    })
}

/// Remove angle and solid angle dimensions from a Dimension.
///
/// Returns a new Dimension with angle and solid_angle exponents set to zero.
fn remove_angle_dimensions(dim: &Dimension) -> Dimension {
    Dimension {
        length: dim.length,
        time: dim.time,
        mass: dim.mass,
        current: dim.current,
        temperature: dim.temperature,
        angle: Rational16::ZERO,       // Treat as dimensionless
        solid_angle: Rational16::ZERO, // Treat as dimensionless
        luminous_intensity: dim.luminous_intensity,
        magnitude: dim.magnitude,
        amount: dim.amount,
        photon: dim.photon,
    }
}

/// Check if a quantity's dimension contains angle components.
///
/// Returns true if the dimension has non-zero angle or solid_angle exponents.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::dimensionless_angles::has_angle_dimension;
///
/// let omega = 1.0 * RAD / S;
/// assert!(has_angle_dimension(omega.unit()));
///
/// let f = 1.0 * HZ;
/// assert!(!has_angle_dimension(f.unit()));
/// ```
pub fn has_angle_dimension(unit: &Unit) -> bool {
    let dim = unit.dimension();
    !dim.angle.is_zero() || !dim.solid_angle.is_zero()
}

/// Get the total angle power in a unit's dimension.
///
/// Returns the sum of angle and solid_angle exponents. Useful for
/// understanding how "polluted" a dimension is with angle terms.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::dimensionless_angles::angle_power;
///
/// let omega_sq = (RAD / S).pow(2);  // rad²/s²
/// assert_eq!(angle_power(&omega_sq), 2);  // Two powers of rad
/// ```
pub fn angle_power(unit: &Unit) -> i32 {
    let dim = unit.dimension();
    // Convert rational to approximate integer (for display purposes)
    let angle_exp = dim.angle.to_f64() as i32;
    let solid_angle_exp = dim.solid_angle.to_f64() as i32;
    angle_exp + solid_angle_exp
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{HZ, J, KG, M, RAD, S, SR, W};
    use std::f64::consts::PI;

    #[test]
    fn test_angular_frequency_to_frequency() {
        // The equivalency treats rad as dimensionless (= 1)
        // So 2π rad/s → 2π Hz (same numeric value, just drops the rad)
        let omega = 2.0 * PI * RAD / S;
        let f = omega.to_equiv(&HZ, dimensionless_angles()).unwrap();
        assert!((f.value() - 2.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_frequency_to_angular_frequency() {
        // 1 Hz = 1 rad/s (treating rad as 1)
        let f = 1.0 * HZ;
        let rad_per_s = RAD / S;
        let omega = f.to_equiv(&rad_per_s, dimensionless_angles()).unwrap();
        assert!((omega.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_rotational_energy() {
        // E = ½Iω²
        // I = 2 kg·m², ω = 3 rad/s
        // E = ½ × 2 × 9 = 9 J
        let I = 2.0 * KG * M * M;
        let omega = 3.0 * RAD / S;
        let omega_squared = &omega * &omega;
        let E_raw = 0.5 * &I * omega_squared;

        // Raw dimension is kg·m²·rad²/s²
        assert!(has_angle_dimension(E_raw.unit()));

        // Convert to Joules
        let E = E_raw.to_equiv(&J, dimensionless_angles()).unwrap();
        assert!((E.value() - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_torque_times_angle() {
        // Work = τ × θ
        // τ = 10 N·m, θ = 2 rad
        // W = 20 J
        let tau = 10.0 * KG * M * M / (S * S); // N·m
        let theta = 2.0 * RAD;
        let work_raw = &tau * &theta;

        // Raw dimension includes rad
        assert!(has_angle_dimension(work_raw.unit()));

        // Convert to Joules
        let work = work_raw.to_equiv(&J, dimensionless_angles()).unwrap();
        assert!((work.value() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_solid_angle_intensity() {
        // Radiant intensity to power: I × Ω = P
        // I = 100 W/sr, Ω = 0.5 sr
        // P = 50 W
        let intensity = 100.0 * W / SR;
        let solid_angle = 0.5 * SR;
        let power_raw = &intensity * &solid_angle;

        // Should already be W (sr cancels), but let's verify
        let power = power_raw.to_equiv(&W, dimensionless_angles()).unwrap();
        assert!((power.value() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_has_angle_dimension() {
        assert!(has_angle_dimension(&(RAD / S)));
        assert!(has_angle_dimension(&Unit::from(SR)));
        assert!(!has_angle_dimension(&Unit::from(HZ)));
        assert!(!has_angle_dimension(&Unit::from(J)));
    }

    #[test]
    fn test_angle_power() {
        let omega = RAD / S;
        assert_eq!(angle_power(&omega), 1);

        let omega_sq = omega.pow(2);
        assert_eq!(angle_power(&omega_sq), 2);

        assert_eq!(angle_power(&Unit::from(HZ)), 0);
    }

    #[test]
    fn test_incompatible_dimensions_fail() {
        // Can't convert kg to Hz even with dimensionless angles
        let mass = 1.0 * KG;
        let result = mass.to_equiv(&HZ, dimensionless_angles());
        assert!(result.is_err());
    }

    #[test]
    fn test_pure_angle_to_dimensionless() {
        // 1 rad should convert to dimensionless 1
        let angle = 1.0 * RAD;
        let dimless = angle
            .to_equiv(&Unit::dimensionless(), dimensionless_angles())
            .unwrap();
        assert!((dimless.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_pi_radians() {
        // π rad = π (dimensionless)
        let angle = PI * RAD;
        let dimless = angle
            .to_equiv(&Unit::dimensionless(), dimensionless_angles())
            .unwrap();
        assert!((dimless.value() - PI).abs() < 1e-10);
    }
}
