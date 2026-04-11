//! Brightness temperature equivalencies for radio astronomy.
//!
//! Brightness temperature is the temperature a blackbody would need to have
//! to produce the observed flux density at a given frequency. It's commonly
//! used in radio astronomy to characterize emission.
//!
//! # Formulas
//!
//! ## Rayleigh-Jeans Approximation (hν << kT)
//!
//! For radio frequencies where the Rayleigh-Jeans approximation is valid:
//!
//! ```text
//! T_b = S_ν × c² / (2k × ν² × Ω)
//! ```
//!
//! Where:
//! - T_b = brightness temperature (K)
//! - S_ν = flux density (W/m²/Hz)
//! - c = speed of light
//! - k = Boltzmann constant
//! - ν = frequency (Hz)
//! - Ω = solid angle (sr)
//!
//! ## Full Planck Function
//!
//! For the general case:
//!
//! ```text
//! B_ν(T) = (2hν³/c²) / (exp(hν/kT) - 1)
//! ```
//!
//! Solving for T requires numerical methods (Newton-Raphson iteration).
//!
//! # Example
//!
//! ```
//! use iridium_units::prelude::*;
//! use iridium_units::equivalencies::brightness_temperature;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Observe 1 Jy at 1.4 GHz with a 1 arcmin² beam
//!     let freq = 1.4e9 * HZ;
//!     let beam = (1.0 / 3600.0_f64.powi(2)) * (std::f64::consts::PI / 180.0).powi(2) * SR;
//!
//!     let flux = 1.0 * JANSKY;
//!     let temp = flux.to_equiv(&K, brightness_temperature(freq, beam))?;
//!     Ok(())
//! }
//! ```

use super::{Converter, Equivalency};
use crate::constants::{BOLTZMANN_CONSTANT, PLANCK_CONSTANT, SPEED_OF_LIGHT};
use crate::dimension::{Dimension, Rational16};
use crate::quantity::Quantity;
use crate::unit::Unit;

// =============================================================================
// Helper Functions
// =============================================================================

/// Check if a unit has flux density dimension (M T⁻²)
/// This is the dimension of W/(m² Hz) = Jansky × 10⁻²⁶
fn is_flux_density(unit: &Unit) -> bool {
    let flux_density_dim = Dimension::MASS
        .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
    unit.dimension() == flux_density_dim
}

/// Check if a unit has temperature dimension
fn is_temperature(unit: &Unit) -> bool {
    unit.dimension() == Dimension::TEMPERATURE
}

/// Check if a unit has spectral radiance dimension (M T⁻² sr⁻¹)
/// This is the dimension of W/(m² Hz sr) = Jansky/sr × 10⁻²⁶
fn is_spectral_radiance(unit: &Unit) -> bool {
    let radiance_dim = Dimension::MASS
        .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
        .mul(&Dimension::SOLID_ANGLE.pow(Rational16::new(-1, 1)));
    unit.dimension() == radiance_dim
}

// =============================================================================
// Rayleigh-Jeans Brightness Temperature
// =============================================================================

/// Create a Rayleigh-Jeans brightness temperature equivalency.
///
/// This converts between flux density and brightness temperature using the
/// Rayleigh-Jeans approximation, which is valid when hν << kT (radio frequencies).
///
/// # Arguments
///
/// * `frequency` - The observation frequency
/// * `beam_solid_angle` - The beam solid angle (e.g., telescope beam or source size)
///
/// # Formulas
///
/// ```text
/// T_b = S_ν × c² / (2k × ν² × Ω)
/// S_ν = T_b × 2k × ν² × Ω / c²
/// ```
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::brightness_temperature;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // 21 cm hydrogen line at 1.420 GHz
///     let freq = 1.420405751768e9 * HZ;
///     // Small beam solid angle
///     let beam = 1e-6 * SR;
///
///     let flux = 1.0 * JANSKY;
///     let temp = flux.to_equiv(&K, brightness_temperature(freq, beam))?;
///     Ok(())
/// }
/// ```
///
/// # Validity
///
/// The Rayleigh-Jeans approximation is valid when hν << kT, which gives:
/// - T >> hν/k ≈ 0.048 × ν\[GHz\] Kelvin
/// - For 1 GHz: valid for T >> 0.048 K
/// - For 100 GHz: valid for T >> 4.8 K
///
/// For higher frequencies or lower temperatures, use [`brightness_temperature_planck`].
pub fn brightness_temperature(frequency: Quantity, beam_solid_angle: Quantity) -> Equivalency {
    // Extract frequency in Hz (SI)
    let nu_hz = frequency.value() * frequency.unit().scale();

    // Extract beam solid angle in sr (SI)
    let omega_sr = beam_solid_angle.value() * beam_solid_angle.unit().scale();

    // Pre-compute constants
    let c_squared = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
    let two_k = 2.0 * BOLTZMANN_CONSTANT;
    let nu_squared = nu_hz * nu_hz;

    // Conversion factor: c² / (2k × ν² × Ω)
    let flux_to_temp_factor = c_squared / (two_k * nu_squared * omega_sr);

    // Inverse: 2k × ν² × Ω / c²
    let temp_to_flux_factor = two_k * nu_squared * omega_sr / c_squared;

    Equivalency::new("brightness_temperature", move |from, to| {
        // Flux density → Temperature
        if is_flux_density(from) && is_temperature(to) {
            return Some(Converter::new(
                move |s_nu_si| {
                    if s_nu_si < 0.0 {
                        return Err("flux density must be non-negative".to_string());
                    }
                    Ok(s_nu_si * flux_to_temp_factor)
                },
                move |t_b_si| {
                    if t_b_si < 0.0 {
                        return Err("brightness temperature must be non-negative".to_string());
                    }
                    Ok(t_b_si * temp_to_flux_factor)
                },
            ));
        }

        // Temperature → Flux density
        if is_temperature(from) && is_flux_density(to) {
            return Some(Converter::new(
                move |t_b_si| {
                    if t_b_si < 0.0 {
                        return Err("brightness temperature must be non-negative".to_string());
                    }
                    Ok(t_b_si * temp_to_flux_factor)
                },
                move |s_nu_si| {
                    if s_nu_si < 0.0 {
                        return Err("flux density must be non-negative".to_string());
                    }
                    Ok(s_nu_si * flux_to_temp_factor)
                },
            ));
        }

        None
    })
}

/// Create a Rayleigh-Jeans brightness temperature equivalency for spectral radiance.
///
/// This variant converts between spectral radiance (intensity per steradian)
/// and brightness temperature, without requiring a beam solid angle.
///
/// # Arguments
///
/// * `frequency` - The observation frequency
///
/// # Formula
///
/// ```text
/// I_ν = 2kTν²/c²    (Rayleigh-Jeans)
/// T_b = I_ν × c² / (2k × ν²)
/// ```
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::brightness_temperature_intensity;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let freq = 1.0e9 * HZ;
///     // Spectral radiance in W/(m² Hz sr)
///     let intensity = 1e-20 * (W / (M.pow(2) * HZ * SR));
///     let temp = intensity.to_equiv(&K, brightness_temperature_intensity(freq))?;
///     Ok(())
/// }
/// ```
pub fn brightness_temperature_intensity(frequency: Quantity) -> Equivalency {
    // Extract frequency in Hz (SI)
    let nu_hz = frequency.value() * frequency.unit().scale();

    // Pre-compute constants
    let c_squared = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
    let two_k = 2.0 * BOLTZMANN_CONSTANT;
    let nu_squared = nu_hz * nu_hz;

    // Conversion factor: c² / (2k × ν²)
    let intensity_to_temp = c_squared / (two_k * nu_squared);

    // Inverse: 2k × ν² / c²
    let temp_to_intensity = two_k * nu_squared / c_squared;

    Equivalency::new("brightness_temperature_intensity", move |from, to| {
        // Spectral radiance → Temperature
        if is_spectral_radiance(from) && is_temperature(to) {
            return Some(Converter::new(
                move |i_nu_si| {
                    if i_nu_si < 0.0 {
                        return Err("spectral radiance must be non-negative".to_string());
                    }
                    Ok(i_nu_si * intensity_to_temp)
                },
                move |t_b_si| {
                    if t_b_si < 0.0 {
                        return Err("brightness temperature must be non-negative".to_string());
                    }
                    Ok(t_b_si * temp_to_intensity)
                },
            ));
        }

        // Temperature → Spectral radiance
        if is_temperature(from) && is_spectral_radiance(to) {
            return Some(Converter::new(
                move |t_b_si| {
                    if t_b_si < 0.0 {
                        return Err("brightness temperature must be non-negative".to_string());
                    }
                    Ok(t_b_si * temp_to_intensity)
                },
                move |i_nu_si| {
                    if i_nu_si < 0.0 {
                        return Err("spectral radiance must be non-negative".to_string());
                    }
                    Ok(i_nu_si * intensity_to_temp)
                },
            ));
        }

        None
    })
}

// =============================================================================
// Full Planck Brightness Temperature
// =============================================================================

/// Create a full Planck brightness temperature equivalency.
///
/// This converts between flux density and brightness temperature using the
/// complete Planck function, valid at all frequencies and temperatures.
///
/// # Arguments
///
/// * `frequency` - The observation frequency
/// * `beam_solid_angle` - The beam solid angle
///
/// # Formula
///
/// The Planck function for spectral radiance is:
///
/// ```text
/// B_ν(T) = (2hν³/c²) / (exp(hν/kT) - 1)
/// ```
///
/// For flux density S_ν = B_ν × Ω, solving for T:
///
/// ```text
/// T = (hν/k) / ln(1 + 2hν³Ω/(c²S_ν))
/// ```
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::brightness_temperature_planck;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Submillimeter observation at 345 GHz
///     let freq = 345.0e9 * HZ;
///     let beam = 1e-8 * SR;  // Small beam
///
///     let flux = 100.0 * JANSKY;
///     let temp = flux.to_equiv(&K, brightness_temperature_planck(freq, beam))?;
///     Ok(())
/// }
/// ```
///
/// # When to Use
///
/// Use the full Planck function when:
/// - Working at submillimeter or infrared frequencies
/// - Brightness temperatures are low (< 10 K for mm wavelengths)
/// - High accuracy is required
///
/// The Rayleigh-Jeans approximation [`brightness_temperature`] is faster and
/// sufficient for most radio astronomy applications.
pub fn brightness_temperature_planck(frequency: Quantity, beam_solid_angle: Quantity) -> Equivalency {
    // Extract frequency in Hz (SI)
    let nu_hz = frequency.value() * frequency.unit().scale();

    // Extract beam solid angle in sr (SI)
    let omega_sr = beam_solid_angle.value() * beam_solid_angle.unit().scale();

    // Pre-compute constants
    let h = PLANCK_CONSTANT;
    let k = BOLTZMANN_CONSTANT;
    let c = SPEED_OF_LIGHT;
    let c_squared = c * c;

    // hν/k - temperature scale for this frequency
    let h_nu_over_k = h * nu_hz / k;

    // 2hν³/c² - Planck function prefactor
    let two_h_nu3_over_c2 = 2.0 * h * nu_hz.powi(3) / c_squared;

    // For flux density: multiply by Ω
    let planck_prefactor_flux = two_h_nu3_over_c2 * omega_sr;

    Equivalency::new("brightness_temperature_planck", move |from, to| {
        // Flux density → Temperature (using full Planck)
        if is_flux_density(from) && is_temperature(to) {
            return Some(Converter::new(
                move |s_nu_si| {
                    if s_nu_si <= 0.0 {
                        return Err("flux density must be positive for Planck conversion".to_string());
                    }
                    // T = (hν/k) / ln(1 + 2hν³Ω/(c²S_ν))
                    let x = planck_prefactor_flux / s_nu_si;
                    if x <= 0.0 {
                        return Err("invalid flux density for Planck inversion".to_string());
                    }
                    let ln_arg = 1.0 + x;
                    if ln_arg <= 1.0 {
                        // This shouldn't happen with positive s_nu and positive x
                        return Err("numerical error in Planck inversion".to_string());
                    }
                    Ok(h_nu_over_k / ln_arg.ln())
                },
                move |t_b_si| {
                    if t_b_si <= 0.0 {
                        return Err("brightness temperature must be positive".to_string());
                    }
                    // S_ν = (2hν³Ω/c²) / (exp(hν/kT) - 1)
                    let exp_arg = h_nu_over_k / t_b_si;
                    if exp_arg > 700.0 {
                        // Would overflow; result is essentially zero
                        return Ok(0.0);
                    }
                    let denominator = exp_arg.exp() - 1.0;
                    if denominator <= 0.0 {
                        return Err("numerical error in Planck function".to_string());
                    }
                    Ok(planck_prefactor_flux / denominator)
                },
            ));
        }

        // Temperature → Flux density (using full Planck)
        if is_temperature(from) && is_flux_density(to) {
            return Some(Converter::new(
                move |t_b_si| {
                    if t_b_si <= 0.0 {
                        return Err("brightness temperature must be positive".to_string());
                    }
                    // S_ν = (2hν³Ω/c²) / (exp(hν/kT) - 1)
                    let exp_arg = h_nu_over_k / t_b_si;
                    if exp_arg > 700.0 {
                        return Ok(0.0);
                    }
                    let denominator = exp_arg.exp() - 1.0;
                    if denominator <= 0.0 {
                        return Err("numerical error in Planck function".to_string());
                    }
                    Ok(planck_prefactor_flux / denominator)
                },
                move |s_nu_si| {
                    if s_nu_si <= 0.0 {
                        return Err("flux density must be positive for Planck conversion".to_string());
                    }
                    let x = planck_prefactor_flux / s_nu_si;
                    let ln_arg = 1.0 + x;
                    if ln_arg <= 1.0 {
                        return Err("numerical error in Planck inversion".to_string());
                    }
                    Ok(h_nu_over_k / ln_arg.ln())
                },
            ));
        }

        None
    })
}

/// Calculate the temperature at which the Rayleigh-Jeans approximation
/// has a given fractional error.
///
/// The Rayleigh-Jeans approximation has error ≈ hν/(2kT) at low hν/kT.
/// This returns T such that the error is approximately `fractional_error`.
///
/// # Example
///
/// ```
/// use iridium_units::equivalencies::brightness_temperature::rayleigh_jeans_validity_temperature;
///
/// // At what temperature is R-J accurate to 1% at 100 GHz?
/// let t_min = rayleigh_jeans_validity_temperature(100.0e9, 0.01);
/// // t_min ≈ 240 K
/// ```
pub fn rayleigh_jeans_validity_temperature(frequency_hz: f64, fractional_error: f64) -> f64 {
    // Error ≈ hν/(2kT), so T ≈ hν/(2k × error)
    let h_nu = PLANCK_CONSTANT * frequency_hz;
    h_nu / (2.0 * BOLTZMANN_CONSTANT * fractional_error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::astrophysical::JANSKY;
    use crate::systems::si::{HZ, K, SR, W, M, GHZ};

    fn spectral_radiance_unit() -> Unit {
        // W/(m² Hz sr)
        W / (&M.pow(2) * HZ * SR)
    }

    #[test]
    fn test_rayleigh_jeans_roundtrip() {
        let freq = 1.0e9 * HZ.clone();  // 1 GHz
        let beam = 1e-6 * SR.clone();   // 1 µsr

        let flux = 1.0 * JANSKY.clone();
        let temp = flux.to_equiv(&K, brightness_temperature(freq.clone(), beam.clone())).unwrap();

        // Convert back
        let flux_back = temp.to_equiv(&JANSKY, brightness_temperature(freq, beam)).unwrap();

        assert!((flux_back.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_rayleigh_jeans_known_value() {
        // Test against known formula: T_b = S_ν × c² / (2k × ν² × Ω)
        let freq_hz: f64 = 1.0e9;  // 1 GHz
        let omega_sr: f64 = 1e-6;  // 1 µsr
        let s_nu_jy: f64 = 1.0;
        let s_nu_si = s_nu_jy * 1e-26;  // Convert to W/(m² Hz)

        let expected_t = s_nu_si * SPEED_OF_LIGHT.powi(2)
            / (2.0 * BOLTZMANN_CONSTANT * freq_hz.powi(2) * omega_sr);

        let freq = freq_hz * HZ.clone();
        let beam = omega_sr * SR.clone();
        let flux = s_nu_jy * JANSKY.clone();

        let temp = flux.to_equiv(&K, brightness_temperature(freq, beam)).unwrap();

        assert!((temp.value() - expected_t).abs() / expected_t < 1e-10);
    }

    #[test]
    fn test_planck_vs_rayleigh_jeans_high_temp() {
        // At high temperature, Planck and R-J should agree
        let freq = 1.0e9 * HZ.clone();  // 1 GHz
        let beam = 1e-6 * SR.clone();

        // Start with a high temperature (R-J valid: T >> hν/k ≈ 0.048 K)
        let temp_high = 1000.0 * K.clone();

        let flux_rj = temp_high.to_equiv(&JANSKY, brightness_temperature(freq.clone(), beam.clone())).unwrap();
        let flux_planck = temp_high.to_equiv(&JANSKY, brightness_temperature_planck(freq, beam)).unwrap();

        // Should agree to better than 0.1% at this temperature
        let rel_diff = (flux_rj.value() - flux_planck.value()).abs() / flux_rj.value();
        assert!(rel_diff < 0.001, "R-J and Planck differ by {} at 1000 K", rel_diff);
    }

    #[test]
    fn test_planck_low_temp_differs() {
        // At low temperature (relative to hν/k), Planck and R-J should differ
        let freq = 100.0e9 * HZ.clone();  // 100 GHz -> hν/k ≈ 4.8 K
        let beam = 1e-6 * SR.clone();

        // Temperature comparable to hν/k
        let temp_low = 10.0 * K.clone();

        let flux_rj = temp_low.to_equiv(&JANSKY, brightness_temperature(freq.clone(), beam.clone())).unwrap();
        let flux_planck = temp_low.to_equiv(&JANSKY, brightness_temperature_planck(freq, beam)).unwrap();

        // Should differ noticeably (Planck gives less flux than R-J at low T)
        assert!(flux_planck.value() < flux_rj.value());
    }

    #[test]
    fn test_planck_roundtrip() {
        let freq = 345.0e9 * HZ.clone();  // 345 GHz (submm)
        let beam = 1e-8 * SR.clone();

        let flux = 100.0 * JANSKY.clone();
        let temp = flux.to_equiv(&K, brightness_temperature_planck(freq.clone(), beam.clone())).unwrap();

        // Convert back
        let flux_back = temp.to_equiv(&JANSKY, brightness_temperature_planck(freq, beam)).unwrap();

        assert!((flux_back.value() - 100.0).abs() / 100.0 < 1e-10);
    }

    #[test]
    fn test_intensity_equivalency() {
        let freq = 1.0e9 * HZ.clone();

        // Test with spectral radiance
        let intensity = 1e-20 * spectral_radiance_unit();
        let temp = intensity.to_equiv(&K, brightness_temperature_intensity(freq.clone())).unwrap();

        // Convert back
        let intensity_back = temp.to_equiv(&spectral_radiance_unit(), brightness_temperature_intensity(freq)).unwrap();

        assert!((intensity_back.value() - 1e-20).abs() / 1e-20 < 1e-10);
    }

    #[test]
    fn test_rj_validity_temperature() {
        // At 100 GHz, for 1% accuracy
        let t_min = rayleigh_jeans_validity_temperature(100.0e9, 0.01);

        // hν/k at 100 GHz ≈ 4.8 K
        // For 1% error: T ≈ hν/(2k × 0.01) ≈ 240 K
        assert!((t_min - 240.0).abs() / 240.0 < 0.1);
    }

    #[test]
    fn test_negative_flux_fails() {
        let freq = 1.0e9 * HZ.clone();
        let beam = 1e-6 * SR.clone();

        let flux = -1.0 * JANSKY.clone();
        let result = flux.to_equiv(&K, brightness_temperature(freq, beam));
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_temp_fails() {
        let freq = 1.0e9 * HZ.clone();
        let beam = 1e-6 * SR.clone();

        let temp = -100.0 * K.clone();
        let result = temp.to_equiv(&JANSKY, brightness_temperature(freq, beam));
        assert!(result.is_err());
    }

    #[test]
    fn test_ghz_frequency() {
        // Test that GHz units work correctly
        let freq = 1.4 * GHZ.clone();  // 1.4 GHz (21 cm line)
        let beam = 1e-6 * SR.clone();

        let flux = 1.0 * JANSKY.clone();
        let temp = flux.to_equiv(&K, brightness_temperature(freq, beam)).unwrap();

        // Should get a reasonable temperature (not crazy high or low)
        assert!(temp.value() > 0.0);
        assert!(temp.value() < 1e10);  // Sanity check
    }
}
