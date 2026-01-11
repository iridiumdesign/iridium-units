//! Parallax equivalency for angle-distance conversion.
//!
//! The parallax equivalency converts between parallax angle and distance
//! using the definition of the parsec: 1 pc is the distance at which
//! 1 AU subtends an angle of 1 arcsecond.
//!
//! Distance (pc) = 1 / parallax (arcsec)

use super::{Converter, Equivalency};
use crate::dimension::Dimension;
use crate::unit::Unit;

// 1 parsec in meters
const PARSEC_M: f64 = 3.085_677_581_491_367_3e16;

// 1 arcsecond in radians
const ARCSEC_RAD: f64 = std::f64::consts::PI / 180.0 / 3600.0;

/// Check if a unit has angle dimension
fn is_angle(unit: &Unit) -> bool {
    unit.dimension() == Dimension::ANGLE
}

/// Check if a unit has length dimension
fn is_length(unit: &Unit) -> bool {
    unit.dimension() == Dimension::LENGTH
}

/// Create a parallax equivalency for angle ↔ distance conversion.
///
/// This uses the definition: distance (pc) = 1 / parallax (arcsec)
///
/// # Example
///
/// ```ignore
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::parallax;
///
/// // Proxima Centauri has a parallax of about 0.77 arcsec
/// let plx = 0.77 * ARCSEC;
/// let distance = plx.to_equiv(&PARSEC, parallax()).unwrap();
/// // ~1.3 pc
/// ```
pub fn parallax() -> Equivalency {
    Equivalency::new("parallax", |from, to| {
        let (is_angle_to_dist, from_scale, to_scale) = if is_angle(from) && is_length(to) {
            (true, from.scale(), to.scale())
        } else if is_length(from) && is_angle(to) {
            (false, from.scale(), to.scale())
        } else {
            return None;
        };

        if is_angle_to_dist {
            // Angle → Distance
            // parallax in radians → distance in meters
            // d = 1 AU / tan(parallax) ≈ 1 AU / parallax (for small angles)
            // Using the definition: d(pc) = 1 / p(arcsec)
            // d(m) = PARSEC_M / (p(rad) / ARCSEC_RAD)
            Some(Converter::new(
                move |p_rad| {
                    if p_rad <= 0.0 {
                        return Err(format!(
                            "parallax angle must be positive, got {}",
                            p_rad
                        ));
                    }
                    Ok(PARSEC_M * ARCSEC_RAD / p_rad)
                },
                move |d_m| {
                    if d_m <= 0.0 {
                        return Err(format!("distance must be positive, got {}", d_m));
                    }
                    Ok(PARSEC_M * ARCSEC_RAD / d_m)
                },
            ))
        } else {
            // Distance → Angle
            Some(Converter::new(
                move |d_m| {
                    if d_m <= 0.0 {
                        return Err(format!("distance must be positive, got {}", d_m));
                    }
                    Ok(PARSEC_M * ARCSEC_RAD / d_m)
                },
                move |p_rad| {
                    if p_rad <= 0.0 {
                        return Err(format!(
                            "parallax angle must be positive, got {}",
                            p_rad
                        ));
                    }
                    Ok(PARSEC_M * ARCSEC_RAD / p_rad)
                },
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{ARCSEC, MAS};
    use crate::systems::astrophysical::PARSEC;
    use crate::Quantity;

    #[test]
    fn test_parallax_1_arcsec() {
        // 1 arcsec parallax = 1 parsec distance (by definition)
        let plx = 1.0 * ARCSEC.clone();
        let dist = plx.to_equiv(&PARSEC, parallax()).unwrap();

        assert!((dist.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_parallax_proxima_centauri() {
        // Proxima Centauri: parallax ≈ 768.5 mas, distance ≈ 1.301 pc
        let plx = 768.5 * MAS.clone();
        let dist = plx.to_equiv(&PARSEC, parallax()).unwrap();

        let expected = 1.0 / 0.7685; // pc
        assert!((dist.value() - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_distance_to_parallax() {
        // 10 pc distance should give 0.1 arcsec parallax
        let dist = 10.0 * PARSEC.clone();
        let plx = dist.to_equiv(&ARCSEC, parallax()).unwrap();

        assert!((plx.value() - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_parallax_roundtrip() {
        let plx = 0.5 * ARCSEC.clone();
        let dist = plx.to_equiv(&PARSEC, parallax()).unwrap();
        let plx_back = dist.to_equiv(&ARCSEC, parallax()).unwrap();

        assert!((plx.value() - plx_back.value()).abs() / plx.value() < 1e-10);
    }

    #[test]
    fn test_zero_parallax_fails() {
        let plx = 0.0 * ARCSEC.clone();
        let result = plx.to_equiv(&PARSEC, parallax());
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_parallax_fails() {
        let plx = -1.0 * ARCSEC.clone();
        let result = plx.to_equiv(&PARSEC, parallax());
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_distance_fails() {
        let dist = 0.0 * PARSEC.clone();
        let result = dist.to_equiv(&ARCSEC, parallax());
        assert!(result.is_err());
    }
}
