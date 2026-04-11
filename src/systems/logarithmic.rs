//! Logarithmic units: magnitudes, decibels, and dex.
//!
//! Logarithmic units have special arithmetic properties. Unlike linear units,
//! adding two magnitude values does not produce a physically meaningful result
//! in the same way that adding two lengths does.
//!
//! # Magnitude Scale (Pogson)
//!
//! Astronomical magnitudes use the Pogson scale:
//! - `m = -2.5 * log10(F/F₀)` where F is flux and F₀ is a reference flux
//! - A difference of 1 magnitude = factor of 10^0.4 ≈ 2.512 in flux
//! - A difference of 5 magnitudes = factor of 100 in flux
//! - **Brighter objects have smaller (more negative) magnitudes**
//!
//! # Decibel Scale
//!
//! Decibels measure power ratios:
//! - `dB = 10 * log10(P/P₀)` for power
//! - `dB = 20 * log10(V/V₀)` for voltage/amplitude
//! - 3 dB ≈ factor of 2 in power
//! - 10 dB = factor of 10 in power
//!
//! # Dex (Order of Magnitude)
//!
//! Dex measures powers of 10:
//! - `dex = log10(x/x₀)`
//! - 1 dex = factor of 10
//! - 2 dex = factor of 100
//!
//! # Working with Logarithmic Quantities
//!
//! Because logarithmic arithmetic differs from linear arithmetic, you should
//! convert to linear space when combining fluxes:
//!
//! ```ignore
//! use iridium_units::prelude::*;
//! use iridium_units::systems::logarithmic::*;
//!
//! // Two stars with apparent magnitudes
//! let star1 = 5.0 * &*MAG;   // magnitude 5
//! let star2 = 3.0 * &*MAG;   // magnitude 3 (brighter)
//!
//! // To find combined brightness, convert to linear flux ratio
//! let flux1 = star1.mag_to_flux_ratio();  // 10^(-0.4 * 5) = 0.01
//! let flux2 = star2.mag_to_flux_ratio();  // 10^(-0.4 * 3) = 0.0631
//!
//! // Combine linearly
//! let combined_flux = flux1 + flux2;
//!
//! // Convert back to magnitude
//! let combined_mag = Quantity::flux_ratio_to_mag(combined_flux);
//! ```

use crate::dimension::Dimension;
use crate::unit::base::BaseUnit;
use crate::unit::Unit;
use lazy_static::lazy_static;

// =============================================================================
// Constants
// =============================================================================

/// Pogson ratio: 10^0.4 ≈ 2.5118864315
/// One magnitude difference corresponds to this flux ratio.
pub const POGSON_RATIO: f64 = 2.511_886_431_509_58;

/// Log base 10 of Pogson ratio = 0.4
pub const LOG10_POGSON: f64 = 0.4;

/// Magnitude zero-point factor: -2.5 (used in m = -2.5 * log10(F/F0))
pub const MAG_FACTOR: f64 = -2.5;

lazy_static! {
    // =============================================================================
    // Magnitude Units
    // =============================================================================

    /// Generic magnitude unit.
    ///
    /// Magnitudes are logarithmic measures of brightness. Lower values indicate
    /// brighter objects:
    /// - Sun: -26.74 mag (apparent)
    /// - Full Moon: -12.7 mag (apparent)
    /// - Sirius: -1.46 mag (apparent)
    /// - Faintest naked-eye stars: +6 mag (apparent)
    /// - Hubble deep field objects: +30 mag
    pub static ref MAG: Unit = Unit::Base(BaseUnit::new(
        "magnitude", "mag", &["magnitudes"],
        Dimension::MAGNITUDE,
        1.0
    ));

    /// Apparent magnitude - observed brightness from Earth.
    ///
    /// Apparent magnitude depends on:
    /// - Intrinsic luminosity of the object
    /// - Distance from observer
    /// - Interstellar extinction
    pub static ref APPARENT_MAG: Unit = Unit::Base(BaseUnit::new(
        "apparent_magnitude", "m_app", &["app_mag", "apparent_mag"],
        Dimension::MAGNITUDE,
        1.0
    ));

    /// Absolute magnitude - brightness at 10 parsecs distance.
    ///
    /// Absolute magnitude removes the distance dependence:
    /// - M = m - 5 * log10(d/10pc)
    /// - Sun: +4.83 (absolute visual)
    /// - Rigel: -7.84 (very luminous)
    /// - Proxima Centauri: +15.6 (dim red dwarf)
    pub static ref ABSOLUTE_MAG: Unit = Unit::Base(BaseUnit::new(
        "absolute_magnitude", "M_abs", &["abs_mag", "absolute_mag"],
        Dimension::MAGNITUDE,
        1.0
    ));

    // =============================================================================
    // Decibel Units
    // =============================================================================

    /// Decibel - logarithmic unit for power ratios.
    ///
    /// ```text
    /// dB = 10 * log10(P/P₀)
    /// ```
    ///
    /// Common reference levels:
    /// - 0 dB SPL = 20 µPa (threshold of hearing)
    /// - 0 dBm = 1 mW (RF power)
    /// - 0 dBW = 1 W (power)
    pub static ref DB: Unit = Unit::Base(BaseUnit::new(
        "decibel", "dB", &["decibels"],
        Dimension::MAGNITUDE,
        1.0
    ));

    /// Bel - base unit of decibels (1 B = 10 dB).
    ///
    /// Rarely used directly; decibels are more common.
    pub static ref BEL: Unit = Unit::Base(BaseUnit::new(
        "bel", "B", &["bels"],
        Dimension::MAGNITUDE,
        10.0  // 1 bel = 10 decibels in magnitude space
    ));

    // =============================================================================
    // Dex Unit
    // =============================================================================

    /// Dex - order of magnitude unit.
    ///
    /// ```text
    /// dex = log10(x/x₀)
    /// ```
    ///
    /// Used in astronomy for expressing ratios:
    /// - 1 dex = factor of 10
    /// - 0.3 dex ≈ factor of 2
    /// - 2 dex = factor of 100
    pub static ref DEX: Unit = Unit::Base(BaseUnit::new(
        "dex", "dex", &[],
        Dimension::MAGNITUDE,
        1.0
    ));

    // =============================================================================
    // Millimagnitude
    // =============================================================================

    /// Millimagnitude (10^-3 magnitude).
    ///
    /// Used for high-precision photometry where changes of 0.001 mag matter.
    pub static ref MILLIMAG: Unit = Unit::Base(BaseUnit::new(
        "millimagnitude", "mmag", &["millimag"],
        Dimension::MAGNITUDE,
        0.001
    ));
}

// =============================================================================
// Logarithmic Conversion Functions
// =============================================================================

/// Convert a magnitude value to a flux ratio.
///
/// Uses the Pogson formula: F/F₀ = 10^(-0.4 * m)
///
/// # Example
///
/// ```
/// use iridium_units::systems::logarithmic::mag_to_flux_ratio;
///
/// let flux_ratio = mag_to_flux_ratio(0.0);
/// assert!((flux_ratio - 1.0).abs() < 1e-10);  // 0 mag = flux ratio of 1
///
/// let flux_ratio_5 = mag_to_flux_ratio(5.0);
/// assert!((flux_ratio_5 - 0.01).abs() < 1e-10);  // 5 mag = 1/100 flux ratio
/// ```
#[inline]
pub fn mag_to_flux_ratio(mag: f64) -> f64 {
    10.0_f64.powf(-LOG10_POGSON * mag)
}

/// Convert a flux ratio to a magnitude.
///
/// Uses the Pogson formula: m = -2.5 * log10(F/F₀)
///
/// # Errors
///
/// Returns an error if flux_ratio is not positive (including NaN).
///
/// # Example
///
/// ```
/// use iridium_units::systems::logarithmic::flux_ratio_to_mag;
///
/// let mag = flux_ratio_to_mag(1.0).unwrap();
/// assert!((mag - 0.0).abs() < 1e-10);  // flux ratio of 1 = 0 mag
///
/// let mag_100 = flux_ratio_to_mag(100.0).unwrap();
/// assert!((mag_100 - (-5.0)).abs() < 1e-10);  // flux ratio of 100 = -5 mag
/// ```
#[inline]
pub fn flux_ratio_to_mag(flux_ratio: f64) -> Result<f64, crate::error::UnitError> {
    if !(flux_ratio > 0.0) {
        return Err(crate::error::UnitError::LogarithmicError(
            "flux ratio must be positive".to_string(),
        ));
    }
    Ok(MAG_FACTOR * flux_ratio.log10())
}

/// Convert a decibel value to a power ratio.
///
/// Uses the formula: P/P₀ = 10^(dB/10)
///
/// # Example
///
/// ```
/// use iridium_units::systems::logarithmic::db_to_power_ratio;
///
/// let ratio = db_to_power_ratio(10.0);
/// assert!((ratio - 10.0).abs() < 1e-10);  // 10 dB = power ratio of 10
///
/// let ratio_3db = db_to_power_ratio(3.0);
/// assert!((ratio_3db - 2.0).abs() < 0.01);  // 3 dB ≈ power ratio of 2
/// ```
#[inline]
pub fn db_to_power_ratio(db: f64) -> f64 {
    10.0_f64.powf(db / 10.0)
}

/// Convert a power ratio to decibels.
///
/// Uses the formula: dB = 10 * log10(P/P₀)
///
/// Returns an error if power_ratio is not positive.
///
/// # Example
///
/// ```
/// use iridium_units::systems::logarithmic::power_ratio_to_db;
///
/// let db = power_ratio_to_db(10.0).unwrap();
/// assert!((db - 10.0).abs() < 1e-10);  // power ratio of 10 = 10 dB
///
/// let db_2 = power_ratio_to_db(2.0).unwrap();
/// assert!((db_2 - 3.01).abs() < 0.01);  // power ratio of 2 ≈ 3 dB
/// ```
#[inline]
pub fn power_ratio_to_db(power_ratio: f64) -> Result<f64, crate::error::UnitError> {
    if !(power_ratio > 0.0) {
        return Err(crate::error::UnitError::LogarithmicError(
            "power ratio must be positive".to_string(),
        ));
    }
    Ok(10.0 * power_ratio.log10())
}

/// Convert a decibel value to an amplitude (voltage) ratio.
///
/// Uses the formula: V/V₀ = 10^(dB/20)
///
/// Note: Amplitude dB is used for voltage, current, and sound pressure.
#[inline]
pub fn db_to_amplitude_ratio(db: f64) -> f64 {
    10.0_f64.powf(db / 20.0)
}

/// Convert an amplitude (voltage) ratio to decibels.
///
/// Uses the formula: dB = 20 * log10(V/V₀)
///
/// Returns an error if amplitude_ratio is not positive.
#[inline]
pub fn amplitude_ratio_to_db(amplitude_ratio: f64) -> Result<f64, crate::error::UnitError> {
    if !(amplitude_ratio > 0.0) {
        return Err(crate::error::UnitError::LogarithmicError(
            "amplitude ratio must be positive".to_string(),
        ));
    }
    Ok(20.0 * amplitude_ratio.log10())
}

/// Convert a dex value to a linear ratio.
///
/// Uses the formula: x/x₀ = 10^dex
///
/// # Example
///
/// ```
/// use iridium_units::systems::logarithmic::dex_to_ratio;
///
/// let ratio = dex_to_ratio(1.0);
/// assert!((ratio - 10.0).abs() < 1e-10);  // 1 dex = ratio of 10
///
/// let ratio_2 = dex_to_ratio(2.0);
/// assert!((ratio_2 - 100.0).abs() < 1e-10);  // 2 dex = ratio of 100
/// ```
#[inline]
pub fn dex_to_ratio(dex: f64) -> f64 {
    10.0_f64.powf(dex)
}

/// Convert a linear ratio to dex.
///
/// Uses the formula: dex = log10(x/x₀)
///
/// # Errors
///
/// Returns an error if ratio is not positive (including NaN).
///
/// # Example
///
/// ```
/// use iridium_units::systems::logarithmic::ratio_to_dex;
///
/// let dex = ratio_to_dex(10.0).unwrap();
/// assert!((dex - 1.0).abs() < 1e-10);  // ratio of 10 = 1 dex
///
/// let dex_100 = ratio_to_dex(100.0).unwrap();
/// assert!((dex_100 - 2.0).abs() < 1e-10);  // ratio of 100 = 2 dex
/// ```
#[inline]
pub fn ratio_to_dex(ratio: f64) -> Result<f64, crate::error::UnitError> {
    if !(ratio > 0.0) {
        return Err(crate::error::UnitError::LogarithmicError(
            "ratio must be positive".to_string(),
        ));
    }
    Ok(ratio.log10())
}

// =============================================================================
// Magnitude Arithmetic Helpers
// =============================================================================

/// Combine two magnitude values by adding their fluxes.
///
/// This is the correct way to combine magnitudes when you want the total
/// brightness of two sources:
///
/// ```
/// use iridium_units::systems::logarithmic::combine_magnitudes;
///
/// // Two equal-brightness stars (e.g., both at mag 5.0)
/// let combined = combine_magnitudes(5.0, 5.0).unwrap();
/// // Combined flux is 2x, so magnitude decreases by 2.5*log10(2) ≈ 0.75
/// assert!((combined - 4.247).abs() < 0.001);
/// ```
#[inline]
pub fn combine_magnitudes(mag1: f64, mag2: f64) -> Result<f64, crate::error::UnitError> {
    let flux1 = mag_to_flux_ratio(mag1);
    let flux2 = mag_to_flux_ratio(mag2);
    flux_ratio_to_mag(flux1 + flux2)
}

/// Calculate the magnitude difference (contrast) between two magnitudes.
///
/// Returns the flux ratio as a magnitude difference.
#[inline]
pub fn magnitude_difference(mag_bright: f64, mag_faint: f64) -> f64 {
    mag_faint - mag_bright
}

/// Calculate distance modulus from apparent and absolute magnitudes.
///
/// Distance modulus: µ = m - M = 5 * log10(d/10pc)
///
/// # Example
///
/// ```
/// use iridium_units::systems::logarithmic::distance_modulus;
///
/// // Sirius: m = -1.46, M = 1.42
/// let mu = distance_modulus(-1.46, 1.42);
/// assert!((mu - (-2.88)).abs() < 0.01);
/// ```
#[inline]
pub fn distance_modulus(apparent_mag: f64, absolute_mag: f64) -> f64 {
    apparent_mag - absolute_mag
}

/// Calculate distance in parsecs from distance modulus.
///
/// d = 10^((µ + 5) / 5) parsecs
///
/// # Example
///
/// ```
/// use iridium_units::systems::logarithmic::distance_from_modulus;
///
/// // Distance modulus of 0 means d = 10 pc
/// let dist = distance_from_modulus(0.0);
/// assert!((dist - 10.0).abs() < 1e-10);
///
/// // Distance modulus of 5 means d = 100 pc
/// let dist_5 = distance_from_modulus(5.0);
/// assert!((dist_5 - 100.0).abs() < 1e-10);
/// ```
#[inline]
pub fn distance_from_modulus(distance_modulus: f64) -> f64 {
    10.0_f64.powf((distance_modulus + 5.0) / 5.0)
}

/// Calculate distance modulus from distance in parsecs.
///
/// µ = 5 * log10(d) - 5
///
/// Returns an error if distance_pc is not positive.
#[inline]
pub fn modulus_from_distance(distance_pc: f64) -> Result<f64, crate::error::UnitError> {
    if !(distance_pc > 0.0) {
        return Err(crate::error::UnitError::LogarithmicError(
            "distance must be positive".to_string(),
        ));
    }
    Ok(5.0 * distance_pc.log10() - 5.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mag_to_flux_roundtrip() {
        for mag in [-5.0, -1.0, 0.0, 1.0, 5.0, 10.0, 20.0] {
            let flux = mag_to_flux_ratio(mag);
            let back = flux_ratio_to_mag(flux).unwrap();
            assert!((back - mag).abs() < 1e-10, "roundtrip failed for mag={}", mag);
        }
    }

    #[test]
    fn test_mag_5_is_100x_fainter() {
        // 5 magnitudes = factor of 100 in flux
        let flux_0 = mag_to_flux_ratio(0.0);
        let flux_5 = mag_to_flux_ratio(5.0);
        assert!((flux_0 / flux_5 - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_db_to_power_roundtrip() {
        for db in [-20.0, -10.0, -3.0, 0.0, 3.0, 10.0, 20.0] {
            let power = db_to_power_ratio(db);
            let back = power_ratio_to_db(power).unwrap();
            assert!((back - db).abs() < 1e-10, "roundtrip failed for db={}", db);
        }
    }

    #[test]
    fn test_3db_is_2x_power() {
        let power_3db = db_to_power_ratio(3.0);
        // 3 dB ≈ 1.995 (close to 2)
        assert!((power_3db - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_10db_is_10x_power() {
        let power_10db = db_to_power_ratio(10.0);
        assert!((power_10db - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_dex_roundtrip() {
        for dex in [-2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0] {
            let ratio = dex_to_ratio(dex);
            let back = ratio_to_dex(ratio).unwrap();
            assert!((back - dex).abs() < 1e-10, "roundtrip failed for dex={}", dex);
        }
    }

    #[test]
    fn test_combine_equal_magnitudes() {
        // Two stars of equal brightness (mag 0) have combined mag = -0.752
        // (because 2x flux = -2.5*log10(2) ≈ -0.752 mag brighter)
        let combined = combine_magnitudes(0.0, 0.0).unwrap();
        let expected = flux_ratio_to_mag(2.0).unwrap();  // -0.752...
        assert!((combined - expected).abs() < 1e-10);
    }

    #[test]
    fn test_distance_modulus() {
        // At 10 pc, m = M (distance modulus = 0)
        let mu = modulus_from_distance(10.0).unwrap();
        assert!((mu - 0.0).abs() < 1e-10);

        // At 100 pc, distance modulus = 5
        let mu_100 = modulus_from_distance(100.0).unwrap();
        assert!((mu_100 - 5.0).abs() < 1e-10);

        // Roundtrip
        let dist_back = distance_from_modulus(mu_100);
        assert!((dist_back - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_error_on_non_positive_inputs() {
        // Zero
        assert!(flux_ratio_to_mag(0.0).is_err());
        assert!(power_ratio_to_db(0.0).is_err());
        assert!(amplitude_ratio_to_db(0.0).is_err());
        assert!(ratio_to_dex(0.0).is_err());
        assert!(modulus_from_distance(0.0).is_err());

        // Negative
        assert!(flux_ratio_to_mag(-1.0).is_err());
        assert!(power_ratio_to_db(-1.0).is_err());
        assert!(amplitude_ratio_to_db(-1.0).is_err());
        assert!(ratio_to_dex(-1.0).is_err());
        assert!(modulus_from_distance(-1.0).is_err());

        // NaN
        assert!(flux_ratio_to_mag(f64::NAN).is_err());
        assert!(power_ratio_to_db(f64::NAN).is_err());
        assert!(amplitude_ratio_to_db(f64::NAN).is_err());
        assert!(ratio_to_dex(f64::NAN).is_err());
        assert!(modulus_from_distance(f64::NAN).is_err());
    }

    #[test]
    fn test_unit_dimension() {
        assert_eq!(MAG.dimension(), Dimension::MAGNITUDE);
        assert_eq!(DB.dimension(), Dimension::MAGNITUDE);
        assert_eq!(DEX.dimension(), Dimension::MAGNITUDE);
        assert_eq!(APPARENT_MAG.dimension(), Dimension::MAGNITUDE);
        assert_eq!(ABSOLUTE_MAG.dimension(), Dimension::MAGNITUDE);
    }

    #[test]
    fn test_millimag_scale() {
        // 1000 mmag = 1 mag
        let q = 1000.0 * MILLIMAG.clone();
        let q_mag = q.to(&MAG).unwrap();
        assert!((q_mag.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bel_to_db() {
        // 1 bel = 10 dB
        let q = 1.0 * BEL.clone();
        let q_db = q.to(&DB).unwrap();
        assert!((q_db.value() - 10.0).abs() < 1e-10);
    }
}
