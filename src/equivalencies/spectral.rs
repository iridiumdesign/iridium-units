//! Spectral equivalencies for wavelength, frequency, energy, and wavenumber.
//!
//! The spectral equivalency allows conversions between:
//! - Wavelength (length)
//! - Frequency (1/time)
//! - Energy (mass * length² / time²)
//! - Wavenumber (1/length)
//!
//! These are related by:
//! - c = λν (speed of light = wavelength × frequency)
//! - E = hν (energy = Planck constant × frequency)
//! - k = 1/λ (wavenumber = 1/wavelength)

use super::{Converter, Equivalency};
use crate::constants::{PLANCK_CONSTANT, SPEED_OF_LIGHT};
use crate::dimension::{Dimension, Rational8};
use crate::unit::Unit;

/// Physical type classification for spectral equivalency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpectralType {
    Wavelength,
    Frequency,
    Energy,
    Wavenumber,
}

fn classify_spectral(unit: &Unit) -> Option<SpectralType> {
    let dim = unit.dimension();

    // Wavelength: L^1
    if dim == Dimension::LENGTH {
        return Some(SpectralType::Wavelength);
    }

    // Frequency: T^-1
    if dim == Dimension::TIME.inv() {
        return Some(SpectralType::Frequency);
    }

    // Energy: M L^2 T^-2
    let energy_dim = Dimension::MASS
        .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
        .mul(&Dimension::TIME.pow(Rational8::new(-2, 1)));
    if dim == energy_dim {
        return Some(SpectralType::Energy);
    }

    // Wavenumber: L^-1
    if dim == Dimension::LENGTH.inv() {
        return Some(SpectralType::Wavenumber);
    }

    None
}

/// Create a spectral equivalency for wavelength ↔ frequency ↔ energy ↔ wavenumber.
///
/// # Example
///
/// ```ignore
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::spectral;
///
/// // Convert 500 nm to Hz
/// let wavelength = 500.0 * NM;
/// let frequency = wavelength.to_equiv(&HZ, spectral()).unwrap();
///
/// // Convert 1 eV to wavelength
/// let energy = 1.0 * EV;
/// let wavelength = energy.to_equiv(&NM, spectral()).unwrap();
/// ```
pub fn spectral() -> Equivalency {
    Equivalency::new("spectral", |from, to| {
        let from_type = classify_spectral(from)?;
        let to_type = classify_spectral(to)?;

        if from_type == to_type {
            // Same type, no equivalency needed
            return None;
        }

        // Get scale factors to convert to/from SI base units
        let from_scale = from.scale();
        let to_scale = to.scale();

        Some(create_spectral_converter(
            from_type, to_type, from_scale, to_scale,
        ))
    })
}

fn create_spectral_converter(
    from_type: SpectralType,
    to_type: SpectralType,
    from_scale: f64,
    to_scale: f64,
) -> Converter {
    use SpectralType::*;

    // All conversions go through SI units (m, Hz, J, 1/m)
    let forward: Box<dyn Fn(f64) -> f64 + Send + Sync> = match (from_type, to_type) {
        // Wavelength -> Frequency: ν = c/λ
        (Wavelength, Frequency) => Box::new(move |lambda_si| SPEED_OF_LIGHT / lambda_si),

        // Frequency -> Wavelength: λ = c/ν
        (Frequency, Wavelength) => Box::new(move |nu_si| SPEED_OF_LIGHT / nu_si),

        // Wavelength -> Energy: E = hc/λ
        (Wavelength, Energy) => {
            Box::new(move |lambda_si| PLANCK_CONSTANT * SPEED_OF_LIGHT / lambda_si)
        }

        // Energy -> Wavelength: λ = hc/E
        (Energy, Wavelength) => {
            Box::new(move |e_si| PLANCK_CONSTANT * SPEED_OF_LIGHT / e_si)
        }

        // Wavelength -> Wavenumber: k = 1/λ
        (Wavelength, Wavenumber) => Box::new(move |lambda_si| 1.0 / lambda_si),

        // Wavenumber -> Wavelength: λ = 1/k
        (Wavenumber, Wavelength) => Box::new(move |k_si| 1.0 / k_si),

        // Frequency -> Energy: E = hν
        (Frequency, Energy) => Box::new(move |nu_si| PLANCK_CONSTANT * nu_si),

        // Energy -> Frequency: ν = E/h
        (Energy, Frequency) => Box::new(move |e_si| e_si / PLANCK_CONSTANT),

        // Frequency -> Wavenumber: k = ν/c
        (Frequency, Wavenumber) => Box::new(move |nu_si| nu_si / SPEED_OF_LIGHT),

        // Wavenumber -> Frequency: ν = kc
        (Wavenumber, Frequency) => Box::new(move |k_si| k_si * SPEED_OF_LIGHT),

        // Energy -> Wavenumber: k = E/(hc)
        (Energy, Wavenumber) => {
            Box::new(move |e_si| e_si / (PLANCK_CONSTANT * SPEED_OF_LIGHT))
        }

        // Wavenumber -> Energy: E = hck
        (Wavenumber, Energy) => {
            Box::new(move |k_si| PLANCK_CONSTANT * SPEED_OF_LIGHT * k_si)
        }

        // Same type shouldn't reach here
        _ => Box::new(|x| x),
    };

    // The backward conversion is the inverse operation
    let backward: Box<dyn Fn(f64) -> f64 + Send + Sync> = match (from_type, to_type) {
        (Wavelength, Frequency) | (Frequency, Wavelength) => {
            Box::new(move |x| SPEED_OF_LIGHT / x)
        }
        (Wavelength, Energy) | (Energy, Wavelength) => {
            Box::new(move |x| PLANCK_CONSTANT * SPEED_OF_LIGHT / x)
        }
        (Wavelength, Wavenumber) | (Wavenumber, Wavelength) => Box::new(move |x| 1.0 / x),
        (Frequency, Energy) => Box::new(move |e_si| e_si / PLANCK_CONSTANT),
        (Energy, Frequency) => Box::new(move |nu_si| PLANCK_CONSTANT * nu_si),
        (Frequency, Wavenumber) => Box::new(move |k_si| k_si * SPEED_OF_LIGHT),
        (Wavenumber, Frequency) => Box::new(move |nu_si| nu_si / SPEED_OF_LIGHT),
        (Energy, Wavenumber) => {
            Box::new(move |k_si| PLANCK_CONSTANT * SPEED_OF_LIGHT * k_si)
        }
        (Wavenumber, Energy) => {
            Box::new(move |e_si| e_si / (PLANCK_CONSTANT * SPEED_OF_LIGHT))
        }
        _ => Box::new(|x| x),
    };

    Converter { forward, backward }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{HZ, NM, M, EV, J};
    use crate::Quantity;

    #[test]
    fn test_wavelength_to_frequency() {
        // 500 nm should be about 6e14 Hz
        let wavelength = 500.0 * NM.clone();
        let frequency = wavelength.to_equiv(&HZ, spectral()).unwrap();

        let expected = SPEED_OF_LIGHT / 500e-9;
        assert!((frequency.value() - expected).abs() / expected < 1e-9);
    }

    #[test]
    fn test_frequency_to_wavelength() {
        // 6e14 Hz should be about 500 nm
        let frequency = 6e14 * HZ.clone();
        let wavelength = frequency.to_equiv(&NM, spectral()).unwrap();

        let expected = SPEED_OF_LIGHT / 6e14 * 1e9; // in nm
        assert!((wavelength.value() - expected).abs() / expected < 1e-9);
    }

    #[test]
    fn test_wavelength_to_energy() {
        // Test conversion from wavelength to energy
        let wavelength = 500.0 * NM.clone();
        let energy = wavelength.to_equiv(&J, spectral()).unwrap();

        let expected = PLANCK_CONSTANT * SPEED_OF_LIGHT / 500e-9;
        assert!((energy.value() - expected).abs() / expected < 1e-9);
    }

    #[test]
    fn test_energy_to_wavelength() {
        // 1 eV should be about 1240 nm
        let energy = 1.0 * EV.clone();
        let wavelength = energy.to_equiv(&NM, spectral()).unwrap();

        // E = hc/λ => λ = hc/E
        let e_joules = 1.602176634e-19; // 1 eV in J
        let expected = PLANCK_CONSTANT * SPEED_OF_LIGHT / e_joules * 1e9;
        assert!((wavelength.value() - expected).abs() / expected < 1e-6);
    }
}
