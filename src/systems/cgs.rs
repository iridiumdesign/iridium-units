//! CGS (Centimeter-Gram-Second) unit system.
//!
//! The CGS system was the first coherent metric system and is still
//! commonly used in astrophysics and electromagnetism.
//!
//! # Examples
//!
//! ```
//! use iridium_units::prelude::*;
//! use iridium_units::systems::cgs::{DYNE, ERG};
//!
//! // Convert SI to CGS
//! let force = 1.0 * N;
//! let in_dyne = force.to(DYNE).unwrap();
//! assert!((in_dyne.value() - 1e5).abs() < 1e-5);
//!
//! let energy = 1.0 * J;
//! let in_erg = energy.to(ERG).unwrap();
//! assert!((in_erg.value() - 1e7).abs() < 1e-3);
//! ```

use crate::dimension::{Dimension, Rational16};
use crate::unit::base::BaseUnit;

// Shared dimension constants
const DIM_FORCE: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH)
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
const DIM_ENERGY: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
const DIM_PRESSURE: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(-1, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
const DIM_VISCOSITY: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(-1, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-1, 1)));
const DIM_MAGNETIC_FIELD: Dimension = Dimension::MASS
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
    .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1)));
const DIM_MAGNETIC_FLUX: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
    .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1)));
const DIM_VOLTAGE: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-3, 1)))
    .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1)));
const DIM_CHARGE: Dimension = Dimension::CURRENT.mul(&Dimension::TIME);

// =============================================================================
// Base CGS Units
// =============================================================================

/// Centimeter - CGS base unit of length (10^-2 m)
pub const CENTIMETER: BaseUnit = BaseUnit::new(
    "centimeter", "cm", &[], Dimension::LENGTH, 1e-2,
);

/// Gram - CGS base unit of mass (10^-3 kg)
pub const GRAM: BaseUnit = BaseUnit::new(
    "gram", "g", &[], Dimension::MASS, 1e-3,
);

// Second is the same as SI

// =============================================================================
// Derived CGS Units
// =============================================================================

/// Dyne - CGS unit of force (g cm / s^2 = 10^-5 N)
pub const DYNE: BaseUnit = BaseUnit::new("dyne", "dyn", &[], DIM_FORCE, 1e-5);

/// Erg - CGS unit of energy (g cm^2 / s^2 = 10^-7 J)
pub const ERG: BaseUnit = BaseUnit::new("erg", "erg", &[], DIM_ENERGY, 1e-7);

/// Barye - CGS unit of pressure (dyn/cm^2 = 0.1 Pa)
pub const BARYE: BaseUnit = BaseUnit::new("barye", "Ba", &[], DIM_PRESSURE, 0.1);

/// Poise - CGS unit of dynamic viscosity (g / cm / s = 0.1 Pa s)
pub const POISE: BaseUnit = BaseUnit::new("poise", "P", &[], DIM_VISCOSITY, 0.1);

/// Stokes - CGS unit of kinematic viscosity (cm^2 / s = 10^-4 m^2/s)
pub const STOKES: BaseUnit = BaseUnit::new(
    "stokes", "St", &[],
    Dimension::LENGTH.pow(Rational16::new(2, 1))
        .mul(&Dimension::TIME.pow(Rational16::new(-1, 1))),
    1e-4,
);

/// Kayser - CGS unit of wavenumber (1/cm)
pub const KAYSER: BaseUnit = BaseUnit::new(
    "kayser", "kayser", &["cm^-1"],
    Dimension::LENGTH.pow(Rational16::new(-1, 1)),
    100.0,
);

// =============================================================================
// CGS Electromagnetic Units (Gaussian)
// =============================================================================

/// Gauss - CGS unit of magnetic field (10^-4 T)
pub const GAUSS: BaseUnit = BaseUnit::new("gauss", "G", &["Gauss"], DIM_MAGNETIC_FIELD, 1e-4);

/// Maxwell - CGS unit of magnetic flux (10^-8 Wb)
pub const MAXWELL: BaseUnit = BaseUnit::new("maxwell", "Mx", &[], DIM_MAGNETIC_FLUX, 1e-8);

/// Oersted - CGS unit of magnetic field strength
pub const OERSTED: BaseUnit = BaseUnit::new(
    "oersted", "Oe", &[], DIM_MAGNETIC_FIELD,
    1e-4 / (4.0 * std::f64::consts::PI) * 1e3,
);

/// Statcoulomb - CGS-ESU unit of charge
pub const STATCOULOMB: BaseUnit = BaseUnit::new("statcoulomb", "statC", &["esu"], DIM_CHARGE, 3.335641e-10);

/// Statampere - CGS-ESU unit of current
pub const STATAMPERE: BaseUnit = BaseUnit::new("statampere", "statA", &[], Dimension::CURRENT, 3.335641e-10);

/// Statvolt - CGS-ESU unit of electric potential
pub const STATVOLT: BaseUnit = BaseUnit::new("statvolt", "statV", &[], DIM_VOLTAGE, 299.792458);

/// Franklin - same as statcoulomb
pub const FRANKLIN: BaseUnit = BaseUnit::new("franklin", "Fr", &[], DIM_CHARGE, 3.335641e-10);

/// Biot - CGS-EMU unit of current (10 A)
pub const BIOT: BaseUnit = BaseUnit::new("biot", "Bi", &["abampere"], Dimension::CURRENT, 10.0);

/// Abcoulomb - CGS-EMU unit of charge (10 C)
pub const ABCOULOMB: BaseUnit = BaseUnit::new("abcoulomb", "abC", &[], DIM_CHARGE, 10.0);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{N, J, T};

    #[test]
    fn test_dyne_to_newton() {
        let q = 1e5 * DYNE;
        let q_n = q.to(N).unwrap();
        assert!((q_n.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_erg_to_joule() {
        let q = 1e7 * ERG;
        let q_j = q.to(J).unwrap();
        assert!((q_j.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_to_tesla() {
        let q = 1e4 * GAUSS;
        let q_t = q.to(T).unwrap();
        assert!((q_t.value() - 1.0).abs() < 1e-10);
    }
}
