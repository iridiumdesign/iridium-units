//! Mass-energy equivalency via E = mc².
//!
//! This equivalency allows conversion between mass and energy using
//! Einstein's famous equation.

use super::{Converter, Equivalency};
use crate::constants::SPEED_OF_LIGHT;
use crate::dimension::{Dimension, Rational8};
use crate::unit::Unit;

/// Check if a unit has mass dimension
fn is_mass(unit: &Unit) -> bool {
    unit.dimension() == Dimension::MASS
}

/// Check if a unit has energy dimension (M L² T⁻²)
fn is_energy(unit: &Unit) -> bool {
    let energy_dim = Dimension::MASS
        .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
        .mul(&Dimension::TIME.pow(Rational8::new(-2, 1)));
    unit.dimension() == energy_dim
}

/// Create a mass-energy equivalency via E = mc².
///
/// # Example
///
/// ```ignore
/// use iridium_units::prelude::*;
/// use iridium_units::equivalencies::mass_energy;
///
/// // Rest mass energy of an electron
/// let electron_mass = 9.109e-31 * KG;
/// let rest_energy = electron_mass.to_equiv(&J, mass_energy()).unwrap();
/// // ~8.19e-14 J = 0.511 MeV
/// ```
pub fn mass_energy() -> Equivalency {
    let c_squared = SPEED_OF_LIGHT * SPEED_OF_LIGHT;

    Equivalency::new("mass_energy", move |from, to| {
        let is_mass_to_energy = if is_mass(from) && is_energy(to) {
            true
        } else if is_energy(from) && is_mass(to) {
            false
        } else {
            return None;
        };

        if is_mass_to_energy {
            // Mass → Energy: E = mc²
            Some(Converter::new(
                move |m_kg| {
                    if m_kg < 0.0 {
                        return Err(format!("mass cannot be negative, got {}", m_kg));
                    }
                    Ok(m_kg * c_squared)
                },
                move |e_j| {
                    if e_j < 0.0 {
                        return Err(format!("energy cannot be negative, got {}", e_j));
                    }
                    Ok(e_j / c_squared)
                },
            ))
        } else {
            // Energy → Mass: m = E/c²
            Some(Converter::new(
                move |e_j| {
                    if e_j < 0.0 {
                        return Err(format!("energy cannot be negative, got {}", e_j));
                    }
                    Ok(e_j / c_squared)
                },
                move |m_kg| {
                    if m_kg < 0.0 {
                        return Err(format!("mass cannot be negative, got {}", m_kg));
                    }
                    Ok(m_kg * c_squared)
                },
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{KG, J, MEV, EV};
    use crate::Quantity;

    #[test]
    fn test_mass_to_energy() {
        // 1 kg should give c² joules
        let mass = 1.0 * KG.clone();
        let energy = mass.to_equiv(&J, mass_energy()).unwrap();

        let expected = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
        assert!((energy.value() - expected).abs() / expected < 1e-10);
    }

    #[test]
    fn test_energy_to_mass() {
        // c² joules should give 1 kg
        let c_sq = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
        let energy = c_sq * J.clone();
        let mass = energy.to_equiv(&KG, mass_energy()).unwrap();

        assert!((mass.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_electron_rest_mass_energy() {
        // Electron mass: 9.1093837015e-31 kg
        // Should give ~0.511 MeV
        let electron_mass = 9.1093837015e-31 * KG.clone();
        let energy_mev = electron_mass.to_equiv(&MEV, mass_energy()).unwrap();

        // Expected: 0.51099895 MeV
        assert!((energy_mev.value() - 0.51099895).abs() / 0.51099895 < 1e-4);
    }

    #[test]
    fn test_mass_energy_roundtrip() {
        let mass = 1e-30 * KG.clone();
        let energy = mass.to_equiv(&J, mass_energy()).unwrap();
        let mass_back = energy.to_equiv(&KG, mass_energy()).unwrap();

        assert!((mass.value() - mass_back.value()).abs() / mass.value() < 1e-10);
    }

    #[test]
    fn test_negative_mass_fails() {
        let mass = -1.0 * KG.clone();
        let result = mass.to_equiv(&J, mass_energy());
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_energy_fails() {
        let energy = -1.0 * J.clone();
        let result = energy.to_equiv(&KG, mass_energy());
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_mass_ok() {
        let mass = 0.0 * KG.clone();
        let result = mass.to_equiv(&J, mass_energy());
        assert!(result.is_ok());
        assert!(result.unwrap().value().abs() < 1e-30);
    }
}
