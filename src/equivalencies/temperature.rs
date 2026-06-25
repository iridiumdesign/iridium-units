//! Temperature equivalencies.
//!
//! This module provides:
//! - `temperature()` - Celsius ↔ Fahrenheit ↔ Kelvin conversions
//! - `temperature_energy()` - Kelvin ↔ electronvolt (via kT)

use super::{Converter, Equivalency};
use crate::constants::BOLTZMANN_CONSTANT;
use crate::dimension::{Dimension, Rational16};
use crate::unit::Unit;

/// Check if a unit has temperature dimension
fn is_temperature(unit: &Unit) -> bool {
    unit.dimension() == Dimension::TEMPERATURE
}

/// Check if a unit has energy dimension (M L² T⁻²)
fn is_energy(unit: &Unit) -> bool {
    let energy_dim = Dimension::MASS
        .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
        .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
    unit.dimension() == energy_dim
}

/// Temperature scale identification
#[derive(Debug, Clone, Copy, PartialEq)]
enum TempScale {
    Kelvin,
    Celsius,
    Fahrenheit,
}

fn identify_temp_scale(unit: &Unit) -> Option<TempScale> {
    if !is_temperature(unit) {
        return None;
    }

    // Use the offset field to identify offset scales
    if let Unit::Base(b) = unit {
        let offset = b.offset;
        // Celsius: offset ≈ 273.15 (K = °C + 273.15)
        if (offset - 273.15).abs() < 1e-6 {
            return Some(TempScale::Celsius);
        }
        // Fahrenheit: offset ≈ 459.67 (K = (°F + 459.67) × 5/9)
        if (offset - 459.67).abs() < 1e-6 {
            return Some(TempScale::Fahrenheit);
        }
    }

    // Absolute scales (Kelvin, Rankine, etc.)
    Some(TempScale::Kelvin)
}

/// Create a temperature equivalency for Celsius ↔ Fahrenheit ↔ Kelvin.
///
/// Note: Temperature scale conversions (K ↔ °C ↔ °F) are handled natively
/// by [`Quantity::to`](crate::Quantity::to) via the unit's offset field. This equivalency is
/// provided for explicit use but is not required for basic conversions.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
///
/// let temp = 100.0 * DEG_C;
/// let kelvin = temp.to(&K).unwrap(); // 373.15 K — no equivalency needed
/// ```
pub fn temperature() -> Equivalency {
    Equivalency::new("temperature", |from, to| {
        let from_scale = identify_temp_scale(from)?;
        let to_scale = identify_temp_scale(to)?;

        if from_scale == to_scale {
            return None;
        }

        let from_unit_scale = from.scale();
        let to_unit_scale = to.scale();

        Some(create_temp_converter(
            from_scale,
            to_scale,
            from_unit_scale,
            to_unit_scale,
        ))
    })
}

/// Create a converter between temperature scales.
///
/// The equivalency pipeline uses `Unit::to_si()` and `Unit::from_si()` to handle
/// offsets, so the converter receives and returns values in Kelvin (SI). For
/// same-dimension temperature conversions, this is a pass-through with validation.
fn create_temp_converter(
    from: TempScale,
    to: TempScale,
    _from_scale: f64,
    _to_scale: f64,
) -> Converter {
    // Both from and to are temperature units. The pipeline has already converted
    // the input to Kelvin via to_si() and will convert the output from Kelvin
    // via from_si(). We just validate the Kelvin value.
    let _ = (from, to);

    Converter::new(
        |k| {
            if k < 0.0 {
                return Err(format!(
                    "temperature cannot be negative in Kelvin, got {}",
                    k
                ));
            }
            Ok(k)
        },
        |k| {
            if k < 0.0 {
                return Err(format!(
                    "temperature cannot be negative in Kelvin, got {}",
                    k
                ));
            }
            Ok(k)
        },
    )
}

/// Create a temperature-energy equivalency via E = kT.
///
/// This allows conversion between temperature (in Kelvin) and energy,
/// using the Boltzmann constant.
///
/// # Example
///
/// ```
/// use iridium_units::prelude::*;
/// use iridium_units::systems::si::EV;
/// use iridium_units::equivalencies::temperature_energy;
///
/// // Room temperature in eV
/// let temp = 300.0 * K;
/// let energy = temp.to_equiv(&EV, temperature_energy()).unwrap();
/// // ~0.026 eV
/// ```
pub fn temperature_energy() -> Equivalency {
    Equivalency::new("temperature_energy", |from, to| {
        let (is_temp_to_energy, _from_scale, _to_scale) = if is_temperature(from) && is_energy(to) {
            (true, from.scale(), to.scale())
        } else if is_energy(from) && is_temperature(to) {
            (false, from.scale(), to.scale())
        } else {
            return None;
        };

        if is_temp_to_energy {
            // T → E: E = kT
            Some(Converter::new(
                |t_kelvin| {
                    if t_kelvin < 0.0 {
                        return Err(format!(
                            "Kelvin temperature cannot be negative, got {}",
                            t_kelvin
                        ));
                    }
                    Ok(BOLTZMANN_CONSTANT * t_kelvin)
                },
                |e_joule| {
                    if e_joule < 0.0 {
                        return Err(format!("energy cannot be negative, got {}", e_joule));
                    }
                    Ok(e_joule / BOLTZMANN_CONSTANT)
                },
            ))
        } else {
            // E → T: T = E/k
            Some(Converter::new(
                |e_joule| {
                    if e_joule < 0.0 {
                        return Err(format!("energy cannot be negative, got {}", e_joule));
                    }
                    Ok(e_joule / BOLTZMANN_CONSTANT)
                },
                |t_kelvin| {
                    if t_kelvin < 0.0 {
                        return Err(format!(
                            "Kelvin temperature cannot be negative, got {}",
                            t_kelvin
                        ));
                    }
                    Ok(BOLTZMANN_CONSTANT * t_kelvin)
                },
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{DEG_C, DEG_F, EV, J, K};

    #[test]
    fn test_temperature_energy_room_temp() {
        // Room temperature ~300K should be ~0.026 eV
        let temp = 300.0 * K;
        let energy = temp.to_equiv(EV, temperature_energy()).unwrap();

        // kT at 300K
        let expected = BOLTZMANN_CONSTANT * 300.0 / 1.602176634e-19;
        assert!((energy.value() - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_temperature_energy_1ev() {
        // 1 eV corresponds to about 11600 K
        let energy = 1.0 * EV;
        let temp = energy.to_equiv(K, temperature_energy()).unwrap();

        let expected = 1.602176634e-19 / BOLTZMANN_CONSTANT;
        assert!((temp.value() - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_temperature_energy_roundtrip() {
        let temp = 1000.0 * K;
        let energy = temp.to_equiv(J, temperature_energy()).unwrap();
        let temp_back = energy.to_equiv(K, temperature_energy()).unwrap();

        assert!((temp.value() - temp_back.value()).abs() / temp.value() < 1e-10);
    }

    #[test]
    fn test_negative_kelvin_fails() {
        let temp = -1.0 * K;
        let result = temp.to_equiv(J, temperature_energy());
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_energy_fails() {
        let energy = -1.0 * J;
        let result = energy.to_equiv(K, temperature_energy());
        assert!(result.is_err());
    }

    #[test]
    fn test_absolute_zero_ok() {
        // Absolute zero (0 K) should be valid
        let temp = 0.0 * K;
        let result = temp.to_equiv(J, temperature_energy());
        assert!(result.is_ok());
        assert!(result.unwrap().value().abs() < 1e-30);
    }

    // Temperature scale conversion tests (#18)
    // These use Quantity::to() directly — offset conversions are native, no equivalency needed.

    #[test]
    fn test_kelvin_to_celsius() {
        // 373.15 K = 100 °C (boiling point of water)
        let temp = 373.15 * K;
        let celsius = temp.to(DEG_C).unwrap();
        assert!((celsius.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        // 0 °C = 273.15 K (freezing point of water)
        let temp = 0.0 * DEG_C;
        let kelvin = temp.to(K).unwrap();
        assert!((kelvin.value() - 273.15).abs() < 1e-10);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        // 373.15 K = 212 °F (boiling point of water)
        let temp = 373.15 * K;
        let fahrenheit = temp.to(DEG_F).unwrap();
        assert!((fahrenheit.value() - 212.0).abs() < 1e-6);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        // 32 °F = 273.15 K (freezing point of water)
        let temp = 32.0 * DEG_F;
        let kelvin = temp.to(K).unwrap();
        assert!((kelvin.value() - 273.15).abs() < 1e-6);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        // 100 °C = 212 °F
        let temp = 100.0 * DEG_C;
        let fahrenheit = temp.to(DEG_F).unwrap();
        assert!((fahrenheit.value() - 212.0).abs() < 1e-6);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        // 32 °F = 0 °C
        let temp = 32.0 * DEG_F;
        let celsius = temp.to(DEG_C).unwrap();
        assert!(celsius.value().abs() < 1e-6);
    }

    #[test]
    fn test_celsius_roundtrip() {
        // °C → K → °C should be exact
        let temp = 37.0 * DEG_C;
        let kelvin = temp.to(K).unwrap();
        let back = kelvin.to(DEG_C).unwrap();
        assert!((back.value() - 37.0).abs() < 1e-10);
    }

    #[test]
    fn test_fahrenheit_roundtrip() {
        // °F → K → °F should be exact
        let temp = 98.6 * DEG_F;
        let kelvin = temp.to(K).unwrap();
        let back = kelvin.to(DEG_F).unwrap();
        assert!((back.value() - 98.6).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_fahrenheit_roundtrip() {
        // °C → °F → °C without going through K
        let temp = 37.0 * DEG_C;
        let f = temp.to(DEG_F).unwrap();
        let back = f.to(DEG_C).unwrap();
        assert!((back.value() - 37.0).abs() < 1e-10);
    }

    #[test]
    fn test_absolute_zero_across_scales() {
        // 0 K = -273.15 °C = -459.67 °F
        let zero_k = 0.0 * K;
        let celsius = zero_k.to(DEG_C).unwrap();
        let fahrenheit = zero_k.to(DEG_F).unwrap();
        assert!((celsius.value() - (-273.15)).abs() < 1e-10);
        assert!((fahrenheit.value() - (-459.67)).abs() < 1e-6);
    }

    #[test]
    fn test_minus_40_identity() {
        // -40 °C = -40 °F (well-known identity)
        let c = -40.0 * DEG_C;
        let f = c.to(DEG_F).unwrap();
        assert!((f.value() - (-40.0)).abs() < 1e-6);

        let f2 = -40.0 * DEG_F;
        let c2 = f2.to(DEG_C).unwrap();
        assert!((c2.value() - (-40.0)).abs() < 1e-6);
    }

    #[test]
    fn test_temperature_equality_across_scales() {
        // 100 °C == 212 °F == 373.15 K
        let c = 100.0 * DEG_C;
        let f = 212.0 * DEG_F;
        let k = 373.15 * K;
        assert_eq!(c, k);
        assert_eq!(c, f);
        assert_eq!(f, k);
    }

    #[test]
    fn test_celsius_decompose() {
        // 100 °C should decompose to 373.15 K (SI)
        let temp = 100.0 * DEG_C;
        let decomposed = temp.decompose();
        assert!((decomposed.value() - 373.15).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_arithmetic() {
        // 5 °C + 3 °C = 8 °C (interval addition)
        let a = 5.0 * DEG_C;
        let b = 3.0 * DEG_C;
        let sum = a + b;
        assert!((sum.value() - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_energy_from_celsius() {
        // temperature_energy equivalency should work with °C input
        // 0 °C = 273.15 K → kT = k * 273.15
        let temp = 0.0 * DEG_C;
        let energy = temp.to_equiv(J, temperature_energy()).unwrap();
        let expected = BOLTZMANN_CONSTANT * 273.15;
        assert!((energy.value() - expected).abs() / expected < 1e-6);
    }
}
