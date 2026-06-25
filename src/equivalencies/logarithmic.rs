//! Logarithmic equivalencies for converting between log and linear quantities.
//!
//! This module provides equivalencies for converting between:
//! - Stellar magnitudes and flux ratios
//! - Decibels and power/amplitude ratios
//! - Dex and linear ratios
//!
//! # Example
//!
//! ```
//! use iridium_units::prelude::*;
//! use iridium_units::equivalencies::logarithmic::magnitude_flux;
//! use iridium_units::systems::logarithmic::MAG;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // A star at magnitude 5
//!     let mag = 5.0 * MAG;
//!
//!     // Convert to flux ratio (dimensionless)
//!     let flux_ratio = mag.to_equiv(&Unit::dimensionless(), magnitude_flux())?;
//!     // flux_ratio.value() ≈ 0.01 (5 mag = 1/100 flux ratio)
//!     Ok(())
//! }
//! ```

use crate::dimension::Dimension;
use crate::equivalencies::{Converter, Equivalency};
use crate::systems::logarithmic::{
    amplitude_ratio_to_db, db_to_amplitude_ratio, db_to_power_ratio, dex_to_ratio,
    flux_ratio_to_mag, mag_to_flux_ratio, power_ratio_to_db, ratio_to_dex,
};

/// Equivalency between stellar magnitudes and flux ratios.
///
/// Converts between magnitude units (which have the MAGNITUDE dimension)
/// and dimensionless flux ratios using the Pogson scale:
///
/// - Forward (mag → flux ratio): `F/F₀ = 10^(-0.4 * m)`
/// - Backward (flux ratio → mag): `m = -2.5 * log10(F/F₀)`
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::logarithmic::magnitude_flux;
/// use iridium_units::systems::logarithmic::MAG;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let star = 5.0 * MAG;  // 5th magnitude star
///     let flux = star.to_equiv(&Unit::dimensionless(), magnitude_flux())?;
///     assert!((flux.value() - 0.01).abs() < 1e-10);  // 1/100 of reference flux
///     Ok(())
/// }
/// ```
pub fn magnitude_flux() -> Equivalency {
    Equivalency::new("magnitude_flux", |from, to| {
        let from_dim = from.dimension();
        let to_dim = to.dimension();

        // Magnitude → Dimensionless (flux ratio)
        // Note: to_equiv already handles scaling, so we work with SI values
        if from_dim == Dimension::MAGNITUDE && to_dim.is_dimensionless() {
            return Some(Converter::new(
                move |mag| {
                    // mag is already in SI magnitude units (scaled by to_equiv)
                    Ok(mag_to_flux_ratio(mag))
                },
                move |flux| flux_ratio_to_mag(flux).map_err(|e| e.to_string()),
            ));
        }

        // Dimensionless (flux ratio) → Magnitude
        if from_dim.is_dimensionless() && to_dim == Dimension::MAGNITUDE {
            return Some(Converter::new(
                move |flux| flux_ratio_to_mag(flux).map_err(|e| e.to_string()),
                move |mag| Ok(mag_to_flux_ratio(mag)),
            ));
        }

        None
    })
}

/// Equivalency between decibels (power) and power ratios.
///
/// Converts using the power formula:
/// - Forward (dB → power ratio): `P/P₀ = 10^(dB/10)`
/// - Backward (power ratio → dB): `dB = 10 * log10(P/P₀)`
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::logarithmic::db_power;
/// use iridium_units::systems::logarithmic::DB;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let signal = 10.0 * DB;  // 10 dB
///     let power = signal.to_equiv(&Unit::dimensionless(), db_power())?;
///     assert!((power.value() - 10.0).abs() < 1e-10);  // 10x power ratio
///     Ok(())
/// }
/// ```
pub fn db_power() -> Equivalency {
    Equivalency::new("db_power", |from, to| {
        let from_dim = from.dimension();
        let to_dim = to.dimension();

        // dB → Dimensionless (power ratio)
        // Note: to_equiv already handles scaling, so we work with SI values
        if from_dim == Dimension::MAGNITUDE && to_dim.is_dimensionless() {
            return Some(Converter::new(
                move |db| Ok(db_to_power_ratio(db)),
                move |power| power_ratio_to_db(power).map_err(|e| e.to_string()),
            ));
        }

        // Dimensionless (power ratio) → dB
        if from_dim.is_dimensionless() && to_dim == Dimension::MAGNITUDE {
            return Some(Converter::new(
                move |power| power_ratio_to_db(power).map_err(|e| e.to_string()),
                move |db| Ok(db_to_power_ratio(db)),
            ));
        }

        None
    })
}

/// Equivalency between decibels (amplitude) and amplitude ratios.
///
/// Uses the amplitude/voltage formula (20 dB per decade):
/// - Forward (dB → amplitude ratio): `V/V₀ = 10^(dB/20)`
/// - Backward (amplitude ratio → dB): `dB = 20 * log10(V/V₀)`
///
/// Use this for voltage, current, and sound pressure measurements.
pub fn db_amplitude() -> Equivalency {
    Equivalency::new("db_amplitude", |from, to| {
        let from_dim = from.dimension();
        let to_dim = to.dimension();

        // dB → Dimensionless (amplitude ratio)
        // Note: to_equiv already handles scaling, so we work with SI values
        if from_dim == Dimension::MAGNITUDE && to_dim.is_dimensionless() {
            return Some(Converter::new(
                move |db| Ok(db_to_amplitude_ratio(db)),
                move |amplitude| amplitude_ratio_to_db(amplitude).map_err(|e| e.to_string()),
            ));
        }

        // Dimensionless (amplitude ratio) → dB
        if from_dim.is_dimensionless() && to_dim == Dimension::MAGNITUDE {
            return Some(Converter::new(
                move |amplitude| amplitude_ratio_to_db(amplitude).map_err(|e| e.to_string()),
                move |db| Ok(db_to_amplitude_ratio(db)),
            ));
        }

        None
    })
}

/// Equivalency between dex and linear ratios.
///
/// Converts using the dex formula:
/// - Forward (dex → ratio): `x/x₀ = 10^dex`
/// - Backward (ratio → dex): `dex = log10(x/x₀)`
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::logarithmic::dex_ratio;
/// use iridium_units::systems::logarithmic::DEX;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let order = 2.0 * DEX;  // 2 orders of magnitude
///     let ratio = order.to_equiv(&Unit::dimensionless(), dex_ratio())?;
///     assert!((ratio.value() - 100.0).abs() < 1e-10);  // factor of 100
///     Ok(())
/// }
/// ```
pub fn dex_ratio() -> Equivalency {
    Equivalency::new("dex_ratio", |from, to| {
        let from_dim = from.dimension();
        let to_dim = to.dimension();

        // Dex → Dimensionless (ratio)
        // Note: to_equiv already handles scaling, so we work with SI values
        if from_dim == Dimension::MAGNITUDE && to_dim.is_dimensionless() {
            return Some(Converter::new(
                move |dex| Ok(dex_to_ratio(dex)),
                move |ratio| ratio_to_dex(ratio).map_err(|e| e.to_string()),
            ));
        }

        // Dimensionless (ratio) → Dex
        if from_dim.is_dimensionless() && to_dim == Dimension::MAGNITUDE {
            return Some(Converter::new(
                move |ratio| ratio_to_dex(ratio).map_err(|e| e.to_string()),
                move |dex| Ok(dex_to_ratio(dex)),
            ));
        }

        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::logarithmic::{DB, DEX, MAG, MILLIMAG};
    use crate::unit::Unit;

    #[test]
    fn test_magnitude_to_flux() {
        let mag5 = 5.0 * MAG;
        let flux = mag5
            .to_equiv(Unit::dimensionless(), magnitude_flux())
            .unwrap();
        // 5 mag = 10^(-0.4*5) = 0.01 flux ratio
        assert!((flux.value() - 0.01).abs() < 1e-10);
    }

    #[test]
    fn test_flux_to_magnitude() {
        let flux = 0.01 * Unit::dimensionless();
        let mag = flux.to_equiv(MAG, magnitude_flux()).unwrap();
        // 0.01 flux ratio = 5 mag
        assert!((mag.value() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_magnitude() {
        let mag0 = 0.0 * MAG;
        let flux = mag0
            .to_equiv(Unit::dimensionless(), magnitude_flux())
            .unwrap();
        // 0 mag = flux ratio of 1
        assert!((flux.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_negative_magnitude() {
        // Negative magnitudes mean brighter than reference
        let mag_neg5 = -5.0 * MAG;
        let flux = mag_neg5
            .to_equiv(Unit::dimensionless(), magnitude_flux())
            .unwrap();
        // -5 mag = 100x flux ratio
        assert!((flux.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_millimag_conversion() {
        // 5000 mmag = 5 mag
        let mmag = 5000.0 * MILLIMAG;
        let flux = mmag
            .to_equiv(Unit::dimensionless(), magnitude_flux())
            .unwrap();
        assert!((flux.value() - 0.01).abs() < 1e-10);
    }

    #[test]
    fn test_db_power_conversion() {
        let db10 = 10.0 * DB;
        let power = db10.to_equiv(Unit::dimensionless(), db_power()).unwrap();
        // 10 dB = 10x power
        assert!((power.value() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_db_3db_is_2x() {
        let db3 = 3.0 * DB;
        let power = db3.to_equiv(Unit::dimensionless(), db_power()).unwrap();
        // 3 dB ≈ 2x power
        assert!((power.value() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_power_to_db() {
        let power = 100.0 * Unit::dimensionless();
        let db = power.to_equiv(DB, db_power()).unwrap();
        // 100x = 20 dB
        assert!((db.value() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_db_amplitude_6db() {
        // 6 dB = 2x amplitude (20 dB per decade)
        let db6 = 6.0 * DB;
        let amp = db6
            .to_equiv(Unit::dimensionless(), db_amplitude())
            .unwrap();
        // 6 dB ≈ 1.995x amplitude
        assert!((amp.value() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_dex_conversion() {
        let dex2 = 2.0 * DEX;
        let ratio = dex2.to_equiv(Unit::dimensionless(), dex_ratio()).unwrap();
        // 2 dex = 100x
        assert!((ratio.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_ratio_to_dex() {
        let ratio = 1000.0 * Unit::dimensionless();
        let dex = ratio.to_equiv(DEX, dex_ratio()).unwrap();
        // 1000x = 3 dex
        assert!((dex.value() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_flux_conversion() {
        // Negative flux ratio should fail
        let flux = -1.0 * Unit::dimensionless();
        let result = flux.to_equiv(MAG, magnitude_flux());
        assert!(result.is_err());
    }
}
