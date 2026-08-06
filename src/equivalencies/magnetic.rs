//! Magnetic flux density and field-strength equivalency.
//!
//! In vacuum, magnetic flux density `B` and magnetic field strength `H` are
//! related by `B = μ₀H`. This equivalency uses SI values internally, so it
//! supports Tesla and any unit with magnetic-field-strength dimension such as
//! `A/m`. Material permeability is not included; callers working with a
//! material must provide the appropriate constitutive relation themselves.

use super::{Converter, Equivalency};
use crate::constants::VACUUM_PERMEABILITY;
use crate::systems::si::{A, M, T};
use crate::unit::Unit;

/// Check if a unit represents magnetic flux density `B`.
fn is_magnetic_flux_density(unit: &Unit) -> bool {
    unit.dimension() == T.dimension()
}

/// Check if a unit represents magnetic field strength `H`.
fn is_magnetic_field_strength(unit: &Unit) -> bool {
    unit.dimension() == (A / M).dimension()
}

/// Create the vacuum magnetic equivalency `B = μ₀H`.
///
/// This converts between magnetic flux density (`B`, measured in tesla) and
/// magnetic field strength (`H`, measured in ampere per metre) in free space.
/// It deliberately does not model material permeability `μ = μᵣμ₀`.
///
/// # Example
///
/// ```
/// use iridium_units::equivalencies::magnetic_flux_density;
/// use iridium_units::prelude::*;
/// use iridium_units::systems::si::T;
///
/// let field_strength = 1.0 * &(A / M);
/// let flux_density = field_strength
///     .to_equiv(T, magnetic_flux_density())
///     .unwrap();
/// assert!((flux_density.value() - 1.256_637_062_12e-6).abs() < 1e-15);
/// ```
pub fn magnetic_flux_density() -> Equivalency {
    Equivalency::new("magnetic_flux_density", |from, to| {
        let h_to_b = if is_magnetic_field_strength(from) && is_magnetic_flux_density(to) {
            true
        } else if is_magnetic_flux_density(from) && is_magnetic_field_strength(to) {
            false
        } else {
            return None;
        };

        if h_to_b {
            Some(Converter::new_infallible(
                |h| h * VACUUM_PERMEABILITY,
                |b| b / VACUUM_PERMEABILITY,
            ))
        } else {
            Some(Converter::new_infallible(
                |b| b / VACUUM_PERMEABILITY,
                |h| h * VACUUM_PERMEABILITY,
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::VACUUM_PERMEABILITY;

    #[test]
    fn test_field_strength_to_flux_density() {
        let field_strength = 1.0 * &(A / M);
        let flux_density = field_strength.to_equiv(T, magnetic_flux_density()).unwrap();

        assert!((flux_density.value() - VACUUM_PERMEABILITY).abs() < 1e-15);
    }

    #[test]
    fn test_flux_density_to_field_strength() {
        let flux_density = VACUUM_PERMEABILITY * T;
        let field_strength = flux_density
            .to_equiv(A / M, magnetic_flux_density())
            .unwrap();

        assert!((field_strength.value() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_magnetic_roundtrip() {
        let original = 2.5 * &(A / M);
        let flux_density = original.to_equiv(T, magnetic_flux_density()).unwrap();
        let recovered = flux_density
            .to_equiv(A / M, magnetic_flux_density())
            .unwrap();

        assert!((recovered.value() - original.value()).abs() < 1e-12);
    }

    #[test]
    fn test_unrelated_dimensions_are_rejected() {
        let result = (1.0 * T).to_equiv(M, magnetic_flux_density());
        assert!(result.is_err());
    }
}
