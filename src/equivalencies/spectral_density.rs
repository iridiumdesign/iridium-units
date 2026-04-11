//! Spectral flux density equivalencies.
//!
//! This module provides equivalencies for converting between different
//! representations of spectral flux density:
//!
//! - **Fλ ↔ Fν conversion**: Convert between flux per wavelength and flux per frequency
//! - **AB magnitude ↔ Fν**: Convert between AB magnitudes and flux density
//!
//! # Physics Background
//!
//! ## Fλ ↔ Fν Conversion
//!
//! Spectral flux density can be expressed per unit wavelength (Fλ) or per unit
//! frequency (Fν). Since energy is conserved: Fλ |dλ| = Fν |dν|
//!
//! Using c = λν and |dλ| = c/ν² |dν|:
//! - Fλ = Fν × c/λ² = Fν × ν²/c
//! - Fν = Fλ × λ²/c
//!
//! ## AB Magnitude System
//!
//! The AB magnitude system is defined as:
//! - m_AB = -2.5 log10(Fν / 3631 Jy)
//!
//! Where 3631 Jy is the zero-point flux density.
//!
//! # Example
//!
//! ```
//! use iridium_units::prelude::*;
//! use iridium_units::equivalencies::spectral_density::{spectral_density, ab_magnitude};
//! use iridium_units::systems::logarithmic::MAG;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Convert Fν to Fλ at 500 nm
//!     let wavelength = 500.0 * &*NM;
//!     let f_nu = 1.0 * &*JANSKY;
//!     let flambda_unit = &*W / (&*M * &*M * &*M);
//!     let f_lambda = f_nu.to_equiv(&flambda_unit, spectral_density(wavelength))?;
//!
//!     // Convert flux density to AB magnitude
//!     let flux = 3631.0 * &*JANSKY;  // Zero point
//!     let mag = flux.to_equiv(&MAG, ab_magnitude())?;  // Should be 0
//!     Ok(())
//! }
//! ```

use super::{Converter, Equivalency};
use crate::constants::SPEED_OF_LIGHT;
use crate::dimension::{Dimension, Rational16};
use crate::quantity::Quantity;
use crate::unit::Unit;

/// The AB magnitude zero point in Jansky (3631 Jy).
pub const AB_ZERO_POINT_JY: f64 = 3631.0;

/// The AB magnitude zero point in SI units (W/m²/Hz).
/// 3631 Jy = 3631 × 10⁻²⁶ W/m²/Hz
pub const AB_ZERO_POINT_SI: f64 = 3631.0e-26;

// Dimension helpers

/// Dimension of Fν (spectral flux density per frequency): M T⁻²
/// This is W/m²/Hz = kg·s⁻²
fn flux_per_frequency_dimension() -> Dimension {
    Dimension::MASS.mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
}

/// Dimension of Fλ (spectral flux density per wavelength): M L⁻¹ T⁻³
/// This is W/m²/m = W/m³ = kg·m⁻¹·s⁻³
fn flux_per_wavelength_dimension() -> Dimension {
    Dimension::MASS
        .mul(&Dimension::LENGTH.pow(Rational16::new(-1, 1)))
        .mul(&Dimension::TIME.pow(Rational16::new(-3, 1)))
}

/// Check if a unit has flux-per-frequency dimension (Fν).
fn is_flux_per_frequency(unit: &Unit) -> bool {
    unit.dimension() == flux_per_frequency_dimension()
}

/// Check if a unit has flux-per-wavelength dimension (Fλ).
fn is_flux_per_wavelength(unit: &Unit) -> bool {
    unit.dimension() == flux_per_wavelength_dimension()
}

/// Check if a unit has magnitude dimension.
fn is_magnitude(unit: &Unit) -> bool {
    unit.dimension() == Dimension::MAGNITUDE
}


/// Spectral density equivalency for converting between Fλ and Fν.
///
/// This equivalency requires a spectral coordinate (wavelength or frequency)
/// to define the conversion. The conversion uses:
/// - Fλ = Fν × c/λ²
/// - Fν = Fλ × λ²/c
///
/// # Arguments
///
/// * `spectral_coord` - A wavelength or frequency quantity that defines
///   the spectral position for the conversion.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::spectral_density::spectral_density;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // At 500 nm wavelength
///     let wavelength = 500.0 * &*NM;
///
///     // Convert 1 Jy to W/m²/m (Fλ)
///     let f_nu = 1.0 * &*JANSKY;
///     let flambda_unit = &*W / (&*M * &*M * &*M);
///     let f_lambda = f_nu.to_equiv(&flambda_unit, spectral_density(wavelength))?;
///     Ok(())
/// }
/// ```
pub fn spectral_density(spectral_coord: Quantity) -> Equivalency {
    // Determine if we have wavelength or frequency
    let coord_dim = spectral_coord.unit().dimension();
    let coord_si = spectral_coord.value() * spectral_coord.unit().scale();

    let is_wavelength = coord_dim == Dimension::LENGTH;
    let is_freq = coord_dim == Dimension::TIME.inv();

    // Convert to wavelength in SI (meters)
    let lambda_si = if is_wavelength {
        coord_si
    } else if is_freq {
        // λ = c/ν
        SPEED_OF_LIGHT / coord_si
    } else {
        // Invalid spectral coordinate - will return None for all conversions
        0.0
    };

    Equivalency::new("spectral_density", move |from, to| {
        // Don't process if lambda is invalid
        if lambda_si <= 0.0 {
            return None;
        }

        let c = SPEED_OF_LIGHT;
        let lambda_squared = lambda_si * lambda_si;

        // Fν → Fλ: Fλ = Fν × c/λ²
        if is_flux_per_frequency(from) && is_flux_per_wavelength(to) {
            let factor = c / lambda_squared;
            return Some(Converter::new(
                move |f_nu_si| {
                    if f_nu_si < 0.0 {
                        return Err("flux density cannot be negative".to_string());
                    }
                    Ok(f_nu_si * factor)
                },
                move |f_lambda_si| {
                    if f_lambda_si < 0.0 {
                        return Err("flux density cannot be negative".to_string());
                    }
                    Ok(f_lambda_si / factor)
                },
            ));
        }

        // Fλ → Fν: Fν = Fλ × λ²/c
        if is_flux_per_wavelength(from) && is_flux_per_frequency(to) {
            let factor = lambda_squared / c;
            return Some(Converter::new(
                move |f_lambda_si| {
                    if f_lambda_si < 0.0 {
                        return Err("flux density cannot be negative".to_string());
                    }
                    Ok(f_lambda_si * factor)
                },
                move |f_nu_si| {
                    if f_nu_si < 0.0 {
                        return Err("flux density cannot be negative".to_string());
                    }
                    Ok(f_nu_si / factor)
                },
            ));
        }

        None
    })
}

/// AB magnitude equivalency for converting between AB magnitudes and Fν.
///
/// The AB magnitude system is defined as:
/// - m_AB = -2.5 log10(Fν / Fν0)
///
/// where Fν0 = 3631 Jy is the zero-point flux density.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::spectral_density::ab_magnitude;
/// use iridium_units::systems::logarithmic::MAG;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // The AB zero point (3631 Jy) corresponds to magnitude 0
///     let flux = 3631.0 * &*JANSKY;
///     let mag = flux.to_equiv(&MAG, ab_magnitude())?;
///     assert!((mag.value() - 0.0).abs() < 1e-10);
///
///     // 1 Jy is about 8.9 AB mag
///     let flux = 1.0 * &*JANSKY;
///     let mag = flux.to_equiv(&MAG, ab_magnitude())?;
///     assert!((mag.value() - 8.9).abs() < 0.1);
///     Ok(())
/// }
/// ```
pub fn ab_magnitude() -> Equivalency {
    Equivalency::new("ab_magnitude", move |from, to| {
        // Fν → AB magnitude
        if is_flux_per_frequency(from) && is_magnitude(to) {
            return Some(Converter::new(
                move |f_nu_si| {
                    if f_nu_si <= 0.0 {
                        return Err("flux density must be positive for magnitude conversion".to_string());
                    }
                    // m_AB = -2.5 log10(Fν / Fν0)
                    let mag = -2.5 * (f_nu_si / AB_ZERO_POINT_SI).log10();
                    Ok(mag)
                },
                move |mag| {
                    // Fν = Fν0 × 10^(-0.4 × m_AB)
                    let f_nu = AB_ZERO_POINT_SI * 10.0_f64.powf(-0.4 * mag);
                    Ok(f_nu)
                },
            ));
        }

        // AB magnitude → Fν
        if is_magnitude(from) && is_flux_per_frequency(to) {
            return Some(Converter::new(
                move |mag| {
                    // Fν = Fν0 × 10^(-0.4 × m_AB)
                    let f_nu = AB_ZERO_POINT_SI * 10.0_f64.powf(-0.4 * mag);
                    Ok(f_nu)
                },
                move |f_nu_si| {
                    if f_nu_si <= 0.0 {
                        return Err("flux density must be positive for magnitude conversion".to_string());
                    }
                    // m_AB = -2.5 log10(Fν / Fν0)
                    let mag = -2.5 * (f_nu_si / AB_ZERO_POINT_SI).log10();
                    Ok(mag)
                },
            ));
        }

        None
    })
}

/// AB magnitude to Fλ equivalency.
///
/// Combines AB magnitude conversion with spectral density conversion
/// to allow direct conversion between AB magnitudes and Fλ at a given
/// wavelength.
///
/// # Arguments
///
/// * `wavelength` - The wavelength at which to perform the conversion.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::spectral_density::ab_magnitude_lambda;
/// use iridium_units::systems::logarithmic::MAG;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let wavelength = 500.0 * &*NM;
///     let flambda_unit = &*W / (&*M * &*M * &*M);
///     let f_lambda = 1e-10 * flambda_unit;
///     let mag = f_lambda.to_equiv(&MAG, ab_magnitude_lambda(wavelength))?;
///     Ok(())
/// }
/// ```
pub fn ab_magnitude_lambda(wavelength: Quantity) -> Equivalency {
    // Get wavelength in SI (meters)
    let lambda_si = wavelength.value() * wavelength.unit().scale();

    Equivalency::new("ab_magnitude_lambda", move |from, to| {
        if lambda_si <= 0.0 {
            return None;
        }

        let c = SPEED_OF_LIGHT;
        let lambda_squared = lambda_si * lambda_si;

        // Fλ → AB magnitude
        // First convert Fλ → Fν: Fν = Fλ × λ²/c
        // Then convert Fν → mag: m = -2.5 log10(Fν / Fν0)
        if is_flux_per_wavelength(from) && is_magnitude(to) {
            let factor = lambda_squared / c;
            return Some(Converter::new(
                move |f_lambda_si| {
                    if f_lambda_si <= 0.0 {
                        return Err("flux density must be positive for magnitude conversion".to_string());
                    }
                    let f_nu_si = f_lambda_si * factor;
                    let mag = -2.5 * (f_nu_si / AB_ZERO_POINT_SI).log10();
                    Ok(mag)
                },
                move |mag| {
                    let f_nu_si = AB_ZERO_POINT_SI * 10.0_f64.powf(-0.4 * mag);
                    let f_lambda_si = f_nu_si / factor;
                    Ok(f_lambda_si)
                },
            ));
        }

        // AB magnitude → Fλ
        if is_magnitude(from) && is_flux_per_wavelength(to) {
            let factor = lambda_squared / c;
            return Some(Converter::new(
                move |mag| {
                    let f_nu_si = AB_ZERO_POINT_SI * 10.0_f64.powf(-0.4 * mag);
                    let f_lambda_si = f_nu_si / factor;
                    Ok(f_lambda_si)
                },
                move |f_lambda_si| {
                    if f_lambda_si <= 0.0 {
                        return Err("flux density must be positive for magnitude conversion".to_string());
                    }
                    let f_nu_si = f_lambda_si * factor;
                    let mag = -2.5 * (f_nu_si / AB_ZERO_POINT_SI).log10();
                    Ok(mag)
                },
            ));
        }

        None
    })
}

/// Convert AB magnitude to flux density in Jansky.
///
/// Convenience function: Fν(Jy) = 3631 × 10^(-0.4 × m_AB)
///
/// # Example
///
/// ```
/// use iridium_units::equivalencies::spectral_density::ab_mag_to_jansky;
///
/// let flux_jy = ab_mag_to_jansky(0.0);
/// assert!((flux_jy - 3631.0).abs() < 1e-10);
///
/// let flux_jy = ab_mag_to_jansky(20.0);
/// assert!((flux_jy - 3.631e-5).abs() < 1e-10);
/// ```
pub fn ab_mag_to_jansky(mag: f64) -> f64 {
    AB_ZERO_POINT_JY * 10.0_f64.powf(-0.4 * mag)
}

/// Convert flux density in Jansky to AB magnitude.
///
/// Convenience function: m_AB = -2.5 × log10(Fν / 3631)
///
/// # Example
///
/// ```
/// use iridium_units::equivalencies::spectral_density::jansky_to_ab_mag;
///
/// let mag = jansky_to_ab_mag(3631.0);
/// assert!((mag - 0.0).abs() < 1e-10);
///
/// let mag = jansky_to_ab_mag(1.0);
/// assert!((mag - 8.9).abs() < 0.01);
/// ```
pub fn jansky_to_ab_mag(flux_jy: f64) -> f64 {
    -2.5 * (flux_jy / AB_ZERO_POINT_JY).log10()
}

/// Convert Fν to Fλ at a given wavelength.
///
/// Formula: Fλ = Fν × c/λ²
///
/// # Arguments
///
/// * `f_nu` - Flux density per frequency in SI units (W/m²/Hz)
/// * `wavelength_m` - Wavelength in meters
///
/// # Returns
///
/// Flux density per wavelength in SI units (W/m³)
///
/// # Example
///
/// ```
/// use iridium_units::equivalencies::spectral_density::f_nu_to_f_lambda;
///
/// // 1 Jy at 500 nm
/// let f_nu_si = 1e-26;  // W/m²/Hz
/// let wavelength_m = 500e-9;  // 500 nm in meters
/// let f_lambda = f_nu_to_f_lambda(f_nu_si, wavelength_m);
/// ```
pub fn f_nu_to_f_lambda(f_nu: f64, wavelength_m: f64) -> f64 {
    f_nu * SPEED_OF_LIGHT / (wavelength_m * wavelength_m)
}

/// Convert Fλ to Fν at a given wavelength.
///
/// Formula: Fν = Fλ × λ²/c
///
/// # Arguments
///
/// * `f_lambda` - Flux density per wavelength in SI units (W/m³)
/// * `wavelength_m` - Wavelength in meters
///
/// # Returns
///
/// Flux density per frequency in SI units (W/m²/Hz)
///
/// # Example
///
/// ```
/// use iridium_units::equivalencies::spectral_density::f_lambda_to_f_nu;
///
/// let f_lambda_si = 1e-10;  // W/m³
/// let wavelength_m = 500e-9;  // 500 nm
/// let f_nu = f_lambda_to_f_nu(f_lambda_si, wavelength_m);
/// ```
pub fn f_lambda_to_f_nu(f_lambda: f64, wavelength_m: f64) -> f64 {
    f_lambda * wavelength_m * wavelength_m / SPEED_OF_LIGHT
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::astrophysical::JANSKY;
    #[cfg(feature = "logarithmic")]
    use crate::systems::logarithmic::MAG;
    use crate::systems::si::{NM, M, HZ, W};

    // Create Fλ unit: W/m³ (W/m²/m)
    fn f_lambda_unit() -> Unit {
        &*W / (&*M * &*M * &*M)
    }

    // Create Fν unit (same dimension as Jansky): W/m²/Hz
    fn f_nu_unit() -> Unit {
        &*W / (&*M * &*M * &*HZ)
    }

    #[cfg(feature = "logarithmic")]
    #[test]
    fn test_ab_zero_point() {
        // 3631 Jy should be 0 AB mag
        let flux = 3631.0 * &*JANSKY;
        let mag = flux.to_equiv(&MAG, ab_magnitude()).unwrap();
        assert!((mag.value() - 0.0).abs() < 1e-10);
    }

    #[cfg(feature = "logarithmic")]
    #[test]
    fn test_ab_mag_1jy() {
        // 1 Jy should be about 8.9 AB mag
        let flux = 1.0 * &*JANSKY;
        let mag = flux.to_equiv(&MAG, ab_magnitude()).unwrap();
        let expected = -2.5 * (1.0 / 3631.0_f64).log10();
        assert!((mag.value() - expected).abs() < 1e-10);
        assert!((mag.value() - 8.9).abs() < 0.05);
    }

    #[cfg(feature = "logarithmic")]
    #[test]
    fn test_ab_mag_to_flux() {
        // 0 AB mag should be 3631 Jy
        let mag = 0.0 * &*MAG;
        let flux = mag.to_equiv(&JANSKY, ab_magnitude()).unwrap();
        assert!((flux.value() - 3631.0).abs() < 1e-6);
    }

    #[cfg(feature = "logarithmic")]
    #[test]
    fn test_ab_mag_20() {
        // 20 AB mag should be 3631 * 10^-8 Jy = 3.631e-5 Jy
        let mag = 20.0 * &*MAG;
        let flux = mag.to_equiv(&JANSKY, ab_magnitude()).unwrap();
        let expected = 3631.0 * 10.0_f64.powf(-0.4 * 20.0);
        assert!((flux.value() - expected).abs() / expected < 1e-10);
    }

    #[cfg(feature = "logarithmic")]
    #[test]
    fn test_ab_mag_roundtrip() {
        let original_flux = 100.0 * &*JANSKY;
        let mag = original_flux.to_equiv(&MAG, ab_magnitude()).unwrap();
        let recovered = mag.to_equiv(&JANSKY, ab_magnitude()).unwrap();
        assert!((recovered.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_f_nu_to_f_lambda_conversion() {
        // Convert 1 Jy at 500 nm to Fλ
        let wavelength = 500.0 * &*NM;
        let f_nu = 1.0 * &*JANSKY;

        let f_lambda = f_nu.to_equiv(&f_lambda_unit(), spectral_density(wavelength)).unwrap();

        // Manual calculation: Fλ = Fν × c/λ²
        let lambda_m = 500e-9;
        let expected = 1e-26 * SPEED_OF_LIGHT / (lambda_m * lambda_m);
        let actual_si = f_lambda.value() * f_lambda.unit().scale();

        assert!((actual_si - expected).abs() / expected < 1e-10);
    }

    #[test]
    fn test_f_lambda_to_f_nu_conversion() {
        // Create Fλ and convert to Fν
        let wavelength = 500.0 * &*NM;

        // Start with some Fλ value
        let f_lambda_val = 1e-10; // W/m³
        let f_lambda = f_lambda_val * f_lambda_unit();

        let f_nu = f_lambda.to_equiv(&f_nu_unit(), spectral_density(wavelength.clone())).unwrap();

        // Manual calculation: Fν = Fλ × λ²/c
        let lambda_m = 500e-9;
        let expected = f_lambda_val * lambda_m * lambda_m / SPEED_OF_LIGHT;
        let actual_si = f_nu.value() * f_nu.unit().scale();

        assert!((actual_si - expected).abs() / expected < 1e-10);
    }

    #[test]
    fn test_spectral_density_roundtrip() {
        let wavelength = 600.0 * &*NM;
        let original = 1.0 * &*JANSKY;

        // Fν → Fλ → Fν
        let f_lambda = original.to_equiv(&f_lambda_unit(), spectral_density(wavelength.clone())).unwrap();
        let recovered = f_lambda.to_equiv(&JANSKY, spectral_density(wavelength)).unwrap();

        assert!((recovered.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_spectral_density_with_frequency() {
        // Can also use frequency as the spectral coordinate
        let frequency = 6e14 * &*HZ; // ~500 nm
        let f_nu = 1.0 * &*JANSKY;

        let f_lambda = f_nu.to_equiv(&f_lambda_unit(), spectral_density(frequency)).unwrap();

        // Should give same result as using wavelength
        let lambda_m = SPEED_OF_LIGHT / 6e14;
        let expected = 1e-26 * SPEED_OF_LIGHT / (lambda_m * lambda_m);
        let actual_si = f_lambda.value() * f_lambda.unit().scale();

        assert!((actual_si - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_negative_flux_fails() {
        let wavelength = 500.0 * &*NM;
        let f_nu = -1.0 * &*JANSKY;

        let result = f_nu.to_equiv(&f_lambda_unit(), spectral_density(wavelength));
        assert!(result.is_err());
    }

    #[test]
    fn test_ab_negative_flux_fails() {
        // Create a negative flux quantity
        let flux = -1.0 * &*JANSKY;
        let result = flux.to_equiv(&MAG, ab_magnitude());
        assert!(result.is_err());
    }

    #[test]
    fn test_convenience_functions() {
        // Test ab_mag_to_jansky
        assert!((ab_mag_to_jansky(0.0) - 3631.0).abs() < 1e-10);
        assert!((ab_mag_to_jansky(20.0) - 3.631e-5).abs() < 1e-10);

        // Test jansky_to_ab_mag
        assert!((jansky_to_ab_mag(3631.0) - 0.0).abs() < 1e-10);
        assert!((jansky_to_ab_mag(1.0) - 8.9).abs() < 0.01);

        // Roundtrip
        let mag = 15.5;
        let flux = ab_mag_to_jansky(mag);
        let recovered = jansky_to_ab_mag(flux);
        assert!((recovered - mag).abs() < 1e-10);
    }

    #[test]
    fn test_f_nu_f_lambda_functions() {
        let wavelength_m = 500e-9;
        let f_nu = 1e-26; // 1 Jy in SI

        let f_lambda = f_nu_to_f_lambda(f_nu, wavelength_m);
        let f_nu_back = f_lambda_to_f_nu(f_lambda, wavelength_m);

        assert!((f_nu_back - f_nu).abs() / f_nu < 1e-10);
    }

    #[test]
    fn test_ab_magnitude_lambda() {
        // Test direct Fλ ↔ AB mag conversion
        let wavelength = 500.0 * &*NM;

        // First get reference: what Fλ corresponds to 0 AB mag at 500 nm?
        // At 0 AB mag, Fν = 3631 Jy
        // Fλ = Fν × c/λ² = 3631e-26 × c / (500e-9)²
        let lambda_m = 500e-9;
        let f_lambda_zero_mag = AB_ZERO_POINT_SI * SPEED_OF_LIGHT / (lambda_m * lambda_m);

        let f_lambda = f_lambda_zero_mag * f_lambda_unit();
        let mag = f_lambda.to_equiv(&MAG, ab_magnitude_lambda(wavelength.clone())).unwrap();

        // Should be close to 0
        assert!((mag.value() - 0.0).abs() < 1e-6);
    }
}
