//! Doppler equivalencies for frequency-velocity conversions.
//!
//! Three Doppler conventions are provided:
//!
//! - **Radio**: v = c(1 - ν/ν₀) — commonly used in radio astronomy
//! - **Optical**: v = c(ν₀/ν - 1) — commonly used in optical astronomy
//! - **Relativistic**: v = c(ν₀² - ν²)/(ν₀² + ν²) — exact relativistic formula
//!
//! Each requires a rest frequency to define the conversion.

use super::{Converter, Equivalency};
use crate::constants::SPEED_OF_LIGHT;
use crate::dimension::{Dimension, Rational16};
use crate::quantity::Quantity;
use crate::unit::Unit;

/// Check if a unit has velocity dimension (L/T)
fn is_velocity(unit: &Unit) -> bool {
    let velocity_dim = Dimension::LENGTH.mul(&Dimension::TIME.pow(Rational16::new(-1, 1)));
    unit.dimension() == velocity_dim
}

/// Check if a unit has frequency dimension (1/T)
fn is_frequency(unit: &Unit) -> bool {
    unit.dimension() == Dimension::TIME.inv()
}

/// Create a Doppler radio equivalency.
///
/// Radio convention: v = c(1 - ν/ν₀)
///
/// This is commonly used in radio astronomy because it produces a linear
/// relationship between velocity and frequency.
///
/// # Arguments
///
/// * `rest_freq` - The rest frequency of the spectral line
///
/// # Example
///
/// ```ignore
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::doppler_radio;
///
/// // CO(1-0) rest frequency
/// let rest_freq = 115.27120 * GHZ;
///
/// // Convert observed frequency to velocity
/// let observed = 115.0 * GHZ;
/// let velocity = observed.to_equiv(&(KM / S), doppler_radio(rest_freq)).unwrap();
/// ```
pub fn doppler_radio(rest_freq: Quantity) -> Equivalency {
    let nu0_hz = rest_freq.value() * rest_freq.unit().scale();

    Equivalency::new("doppler_radio", move |from, to| {
        let (is_freq_to_vel, _from_scale, _to_scale) = if is_frequency(from) && is_velocity(to) {
            (true, from.scale(), to.scale())
        } else if is_velocity(from) && is_frequency(to) {
            (false, from.scale(), to.scale())
        } else {
            return None;
        };

        let nu0 = nu0_hz;

        if is_freq_to_vel {
            // ν → v: v = c(1 - ν/ν₀)
            Some(Converter::new(
                move |nu_si| {
                    if nu_si <= 0.0 {
                        return Err(format!("frequency must be positive, got {}", nu_si));
                    }
                    Ok(SPEED_OF_LIGHT * (1.0 - nu_si / nu0))
                },
                move |v_si| {
                    if v_si.abs() >= SPEED_OF_LIGHT {
                        return Err(format!(
                            "velocity must be less than speed of light, got {} m/s",
                            v_si
                        ));
                    }
                    Ok(nu0 * (1.0 - v_si / SPEED_OF_LIGHT))
                },
            ))
        } else {
            // v → ν: ν = ν₀(1 - v/c)
            Some(Converter::new(
                move |v_si| {
                    if v_si.abs() >= SPEED_OF_LIGHT {
                        return Err(format!(
                            "velocity must be less than speed of light, got {} m/s",
                            v_si
                        ));
                    }
                    Ok(nu0 * (1.0 - v_si / SPEED_OF_LIGHT))
                },
                move |nu_si| {
                    if nu_si <= 0.0 {
                        return Err(format!("frequency must be positive, got {}", nu_si));
                    }
                    Ok(SPEED_OF_LIGHT * (1.0 - nu_si / nu0))
                },
            ))
        }
    })
}

/// Create a Doppler optical equivalency.
///
/// Optical convention: v = c(ν₀/ν - 1) = cz
///
/// This is commonly used in optical astronomy because it directly gives
/// the redshift z = Δλ/λ₀.
///
/// # Arguments
///
/// * `rest_freq` - The rest frequency of the spectral line
pub fn doppler_optical(rest_freq: Quantity) -> Equivalency {
    let nu0_hz = rest_freq.value() * rest_freq.unit().scale();

    Equivalency::new("doppler_optical", move |from, to| {
        let is_freq_to_vel = if is_frequency(from) && is_velocity(to) {
            true
        } else if is_velocity(from) && is_frequency(to) {
            false
        } else {
            return None;
        };

        let nu0 = nu0_hz;

        if is_freq_to_vel {
            // ν → v: v = c(ν₀/ν - 1)
            Some(Converter::new(
                move |nu_si| {
                    if nu_si <= 0.0 {
                        return Err(format!("frequency must be positive, got {}", nu_si));
                    }
                    Ok(SPEED_OF_LIGHT * (nu0 / nu_si - 1.0))
                },
                move |v_si| {
                    // In optical convention, v can exceed c for high redshifts
                    // but v < -c would give negative frequency
                    if v_si <= -SPEED_OF_LIGHT {
                        return Err(format!(
                            "velocity must be greater than -c, got {} m/s",
                            v_si
                        ));
                    }
                    Ok(nu0 / (1.0 + v_si / SPEED_OF_LIGHT))
                },
            ))
        } else {
            // v → ν: ν = ν₀/(1 + v/c)
            Some(Converter::new(
                move |v_si| {
                    if v_si <= -SPEED_OF_LIGHT {
                        return Err(format!(
                            "velocity must be greater than -c, got {} m/s",
                            v_si
                        ));
                    }
                    Ok(nu0 / (1.0 + v_si / SPEED_OF_LIGHT))
                },
                move |nu_si| {
                    if nu_si <= 0.0 {
                        return Err(format!("frequency must be positive, got {}", nu_si));
                    }
                    Ok(SPEED_OF_LIGHT * (nu0 / nu_si - 1.0))
                },
            ))
        }
    })
}

/// Create a Doppler relativistic equivalency.
///
/// Relativistic formula: v = c(ν₀² - ν²)/(ν₀² + ν²)
///
/// This is the exact relativistic Doppler formula, accurate at all velocities.
///
/// # Arguments
///
/// * `rest_freq` - The rest frequency of the spectral line
pub fn doppler_relativistic(rest_freq: Quantity) -> Equivalency {
    let nu0_hz = rest_freq.value() * rest_freq.unit().scale();

    Equivalency::new("doppler_relativistic", move |from, to| {
        let is_freq_to_vel = if is_frequency(from) && is_velocity(to) {
            true
        } else if is_velocity(from) && is_frequency(to) {
            false
        } else {
            return None;
        };

        let nu0 = nu0_hz;
        let nu0_sq = nu0 * nu0;

        if is_freq_to_vel {
            // ν → v: v = c(ν₀² - ν²)/(ν₀² + ν²)
            Some(Converter::new(
                move |nu_si| {
                    if nu_si <= 0.0 {
                        return Err(format!("frequency must be positive, got {}", nu_si));
                    }
                    let nu_sq = nu_si * nu_si;
                    Ok(SPEED_OF_LIGHT * (nu0_sq - nu_sq) / (nu0_sq + nu_sq))
                },
                move |v_si| {
                    // ν = ν₀ * sqrt((1 - v/c)/(1 + v/c))
                    if v_si.abs() >= SPEED_OF_LIGHT {
                        return Err(format!(
                            "velocity must be less than speed of light, got {} m/s",
                            v_si
                        ));
                    }
                    let beta = v_si / SPEED_OF_LIGHT;
                    Ok(nu0 * ((1.0 - beta) / (1.0 + beta)).sqrt())
                },
            ))
        } else {
            // v → ν: ν = ν₀ * sqrt((1 - v/c)/(1 + v/c))
            Some(Converter::new(
                move |v_si| {
                    if v_si.abs() >= SPEED_OF_LIGHT {
                        return Err(format!(
                            "velocity must be less than speed of light, got {} m/s",
                            v_si
                        ));
                    }
                    let beta = v_si / SPEED_OF_LIGHT;
                    Ok(nu0 * ((1.0 - beta) / (1.0 + beta)).sqrt())
                },
                move |nu_si| {
                    if nu_si <= 0.0 {
                        return Err(format!("frequency must be positive, got {}", nu_si));
                    }
                    let nu_sq = nu_si * nu_si;
                    Ok(SPEED_OF_LIGHT * (nu0_sq - nu_sq) / (nu0_sq + nu_sq))
                },
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{GHZ, HZ, M, S, KM};
    use crate::Quantity;

    fn km_per_s() -> Unit {
        &*KM / &*S
    }

    #[test]
    fn test_doppler_radio_redshift() {
        // Test with CO(1-0) at 115.27120 GHz
        let rest_freq = 115.27120 * GHZ.clone();

        // If observed frequency is lower, velocity should be positive (receding)
        let observed = 115.0 * GHZ.clone();
        let velocity = observed.to_equiv(&km_per_s(), doppler_radio(rest_freq.clone())).unwrap();

        // v = c(1 - ν/ν₀) = c(1 - 115/115.27120)
        let expected = SPEED_OF_LIGHT * (1.0 - 115.0 / 115.27120) / 1000.0; // km/s
        assert!((velocity.value() - expected).abs() / expected.abs() < 1e-6);
    }

    #[test]
    fn test_doppler_radio_blueshift() {
        let rest_freq = 115.27120 * GHZ.clone();

        // If observed frequency is higher, velocity should be negative (approaching)
        let observed = 116.0 * GHZ.clone();
        let velocity = observed.to_equiv(&km_per_s(), doppler_radio(rest_freq)).unwrap();

        assert!(velocity.value() < 0.0); // Approaching
    }

    #[test]
    fn test_doppler_optical() {
        let rest_freq = 1e15 * HZ.clone(); // Some UV frequency

        // z = 0.1 redshift means v/c = 0.1 in optical convention
        // ν = ν₀/(1 + z) = ν₀/1.1
        let observed = (1e15 / 1.1) * HZ.clone();
        let velocity = observed.to_equiv(&km_per_s(), doppler_optical(rest_freq)).unwrap();

        // v = c * z = c * 0.1
        let expected = SPEED_OF_LIGHT * 0.1 / 1000.0; // km/s
        assert!((velocity.value() - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_doppler_relativistic_low_velocity() {
        // At low velocities, all three conventions should agree
        let rest_freq = 1e9 * HZ.clone();

        // Small velocity: 1 km/s << c
        let velocity = 1.0 * km_per_s();

        let freq_radio = velocity.to_equiv(&HZ, doppler_radio(rest_freq.clone())).unwrap();
        let freq_optical = velocity.to_equiv(&HZ, doppler_optical(rest_freq.clone())).unwrap();
        let freq_rel = velocity.to_equiv(&HZ, doppler_relativistic(rest_freq.clone())).unwrap();

        // All should be very close for small velocities
        assert!((freq_radio.value() - freq_optical.value()).abs() / freq_radio.value() < 1e-6);
        assert!((freq_radio.value() - freq_rel.value()).abs() / freq_radio.value() < 1e-6);
    }

    fn m_per_s() -> Unit {
        &*M / &*S
    }

    #[test]
    fn test_superluminal_radio_fails() {
        let rest_freq = 1e9 * HZ.clone();
        // Velocity at speed of light
        let velocity = SPEED_OF_LIGHT * m_per_s();
        let result = velocity.to_equiv(&HZ, doppler_radio(rest_freq));
        assert!(result.is_err());
    }

    #[test]
    fn test_superluminal_relativistic_fails() {
        let rest_freq = 1e9 * HZ.clone();
        // Velocity at speed of light
        let velocity = SPEED_OF_LIGHT * m_per_s();
        let result = velocity.to_equiv(&HZ, doppler_relativistic(rest_freq));
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_frequency_fails() {
        let rest_freq = 1e9 * HZ.clone();
        let frequency = 0.0 * HZ.clone();
        let result = frequency.to_equiv(&km_per_s(), doppler_radio(rest_freq));
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_frequency_fails() {
        let rest_freq = 1e9 * HZ.clone();
        let frequency = -1e9 * HZ.clone();
        let result = frequency.to_equiv(&km_per_s(), doppler_relativistic(rest_freq));
        assert!(result.is_err());
    }
}
