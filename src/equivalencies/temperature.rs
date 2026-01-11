//! Temperature equivalencies.
//!
//! This module provides:
//! - `temperature()` - Celsius ↔ Fahrenheit ↔ Kelvin conversions
//! - `temperature_energy()` - Kelvin ↔ electronvolt (via kT)

use super::{Converter, Equivalency};
use crate::constants::BOLTZMANN_CONSTANT;
use crate::dimension::{Dimension, Rational8};
use crate::unit::Unit;

/// Check if a unit has temperature dimension
fn is_temperature(unit: &Unit) -> bool {
    unit.dimension() == Dimension::TEMPERATURE
}

/// Check if a unit has energy dimension (M L² T⁻²)
fn is_energy(unit: &Unit) -> bool {
    let energy_dim = Dimension::MASS
        .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
        .mul(&Dimension::TIME.pow(Rational8::new(-2, 1)));
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

    let symbol = unit.symbol().to_lowercase();

    // Kelvin is the SI base unit with scale 1.0
    if unit.scale() == 1.0 {
        return Some(TempScale::Kelvin);
    }

    // Check by symbol
    if symbol.contains("deg") && symbol.contains("c") || symbol == "celsius" {
        return Some(TempScale::Celsius);
    }

    if symbol.contains("deg") && symbol.contains("f") || symbol == "fahrenheit" {
        return Some(TempScale::Fahrenheit);
    }

    // Rankine has scale 5/9 relative to Kelvin (same intervals as Fahrenheit)
    if (unit.scale() - 5.0 / 9.0).abs() < 1e-10 {
        // Rankine is an absolute scale, treat like Kelvin for intervals
        return Some(TempScale::Kelvin);
    }

    // Default to Kelvin for unknown temperature units
    Some(TempScale::Kelvin)
}

/// Create a temperature equivalency for Celsius ↔ Fahrenheit ↔ Kelvin.
///
/// Note: This handles the offset conversions between temperature scales.
/// For interval/difference conversions (e.g., ΔT), use direct unit conversion.
///
/// # Example
///
/// ```ignore
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::temperature;
///
/// // This would require special handling for offset temperature scales
/// // Currently, the library treats all temperatures as intervals from absolute zero
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

fn create_temp_converter(
    from: TempScale,
    to: TempScale,
    _from_scale: f64,
    _to_scale: f64,
) -> Converter {
    use TempScale::*;

    match (from, to) {
        (Kelvin, Celsius) => Converter::new(|k| k - 273.15, |c| c + 273.15),

        (Celsius, Kelvin) => Converter::new(|c| c + 273.15, |k| k - 273.15),

        (Kelvin, Fahrenheit) => {
            Converter::new(|k| (k - 273.15) * 9.0 / 5.0 + 32.0, |f| (f - 32.0) * 5.0 / 9.0 + 273.15)
        }

        (Fahrenheit, Kelvin) => {
            Converter::new(|f| (f - 32.0) * 5.0 / 9.0 + 273.15, |k| (k - 273.15) * 9.0 / 5.0 + 32.0)
        }

        (Celsius, Fahrenheit) => {
            Converter::new(|c| c * 9.0 / 5.0 + 32.0, |f| (f - 32.0) * 5.0 / 9.0)
        }

        (Fahrenheit, Celsius) => {
            Converter::new(|f| (f - 32.0) * 5.0 / 9.0, |c| c * 9.0 / 5.0 + 32.0)
        }

        _ => Converter::new(|x| x, |x| x),
    }
}

/// Create a temperature-energy equivalency via E = kT.
///
/// This allows conversion between temperature (in Kelvin) and energy,
/// using the Boltzmann constant.
///
/// # Example
///
/// ```ignore
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::temperature_energy;
///
/// // Room temperature in eV
/// let temp = 300.0 * K;
/// let energy = temp.to_equiv(&EV, temperature_energy()).unwrap();
/// // ~0.026 eV
/// ```
pub fn temperature_energy() -> Equivalency {
    Equivalency::new("temperature_energy", |from, to| {
        let (is_temp_to_energy, from_scale, to_scale) =
            if is_temperature(from) && is_energy(to) {
                (true, from.scale(), to.scale())
            } else if is_energy(from) && is_temperature(to) {
                (false, from.scale(), to.scale())
            } else {
                return None;
            };

        if is_temp_to_energy {
            // T → E: E = kT
            Some(Converter::new(
                |t_kelvin| BOLTZMANN_CONSTANT * t_kelvin,
                |e_joule| e_joule / BOLTZMANN_CONSTANT,
            ))
        } else {
            // E → T: T = E/k
            Some(Converter::new(
                |e_joule| e_joule / BOLTZMANN_CONSTANT,
                |t_kelvin| BOLTZMANN_CONSTANT * t_kelvin,
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{K, EV, J};
    use crate::Quantity;

    #[test]
    fn test_temperature_energy_room_temp() {
        // Room temperature ~300K should be ~0.026 eV
        let temp = 300.0 * K.clone();
        let energy = temp.to_equiv(&EV, temperature_energy()).unwrap();

        // kT at 300K
        let expected = BOLTZMANN_CONSTANT * 300.0 / 1.602176634e-19;
        assert!((energy.value() - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_temperature_energy_1ev() {
        // 1 eV corresponds to about 11600 K
        let energy = 1.0 * EV.clone();
        let temp = energy.to_equiv(&K, temperature_energy()).unwrap();

        let expected = 1.602176634e-19 / BOLTZMANN_CONSTANT;
        assert!((temp.value() - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_temperature_energy_roundtrip() {
        let temp = 1000.0 * K.clone();
        let energy = temp.to_equiv(&J, temperature_energy()).unwrap();
        let temp_back = energy.to_equiv(&K, temperature_energy()).unwrap();

        assert!((temp.value() - temp_back.value()).abs() / temp.value() < 1e-10);
    }
}
