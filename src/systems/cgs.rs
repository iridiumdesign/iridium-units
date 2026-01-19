//! CGS (Centimeter-Gram-Second) unit system.
//!
//! The CGS system was the first coherent metric system and is still
//! commonly used in astrophysics and electromagnetism.

use crate::dimension::{Dimension, Rational16};
use crate::unit::base::BaseUnit;
use crate::unit::Unit;
use lazy_static::lazy_static;

lazy_static! {
    // =============================================================================
    // Base CGS Units
    // =============================================================================

    /// Centimeter - CGS base unit of length (10^-2 m)
    pub static ref CENTIMETER: Unit = Unit::Base(BaseUnit::new(
        "centimeter", "cm", &[], Dimension::LENGTH, 1e-2
    ));

    /// Gram - CGS base unit of mass (10^-3 kg)
    pub static ref GRAM: Unit = Unit::Base(BaseUnit::new(
        "gram", "g", &[], Dimension::MASS, 1e-3
    ));

    // Second is the same as SI

    // =============================================================================
    // Derived CGS Units
    // =============================================================================

    /// Dyne - CGS unit of force (g cm / s^2 = 10^-5 N)
    pub static ref DYNE: Unit = Unit::Base(BaseUnit::new(
        "dyne", "dyn", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH)
            .mul(&Dimension::TIME.pow(Rational16::new(-2, 1))),
        1e-5
    ));

    /// Erg - CGS unit of energy (g cm^2 / s^2 = 10^-7 J)
    pub static ref ERG: Unit = Unit::Base(BaseUnit::new(
        "erg", "erg", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational16::new(-2, 1))),
        1e-7
    ));

    /// Barye - CGS unit of pressure (dyn/cm^2 = 0.1 Pa)
    pub static ref BARYE: Unit = Unit::Base(BaseUnit::new(
        "barye", "Ba", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational16::new(-1, 1)))
            .mul(&Dimension::TIME.pow(Rational16::new(-2, 1))),
        0.1
    ));

    /// Poise - CGS unit of dynamic viscosity (g / cm / s = 0.1 Pa s)
    pub static ref POISE: Unit = Unit::Base(BaseUnit::new(
        "poise", "P", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational16::new(-1, 1)))
            .mul(&Dimension::TIME.pow(Rational16::new(-1, 1))),
        0.1
    ));

    /// Stokes - CGS unit of kinematic viscosity (cm^2 / s = 10^-4 m^2/s)
    pub static ref STOKES: Unit = Unit::Base(BaseUnit::new(
        "stokes", "St", &[],
        Dimension::LENGTH.pow(Rational16::new(2, 1))
            .mul(&Dimension::TIME.pow(Rational16::new(-1, 1))),
        1e-4
    ));

    /// Kayser - CGS unit of wavenumber (1/cm)
    pub static ref KAYSER: Unit = Unit::Base(BaseUnit::new(
        "kayser", "kayser", &["cm^-1"],
        Dimension::LENGTH.pow(Rational16::new(-1, 1)),
        100.0  // 1/cm = 100 /m
    ));

    // =============================================================================
    // CGS Electromagnetic Units (Gaussian)
    // =============================================================================

    /// Gauss - CGS unit of magnetic field (10^-4 T)
    pub static ref GAUSS: Unit = Unit::Base(BaseUnit::new(
        "gauss", "G", &["Gauss"],
        Dimension::MASS
            .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
            .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1))),
        1e-4
    ));

    /// Maxwell - CGS unit of magnetic flux (10^-8 Wb)
    pub static ref MAXWELL: Unit = Unit::Base(BaseUnit::new(
        "maxwell", "Mx", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
            .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1))),
        1e-8
    ));

    /// Oersted - CGS unit of magnetic field strength
    /// Note: In Gaussian CGS, H and B have the same dimension
    pub static ref OERSTED: Unit = Unit::Base(BaseUnit::new(
        "oersted", "Oe", &[],
        Dimension::MASS
            .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
            .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1))),
        1e-4 / (4.0 * std::f64::consts::PI) * 1e3  // 1000/(4π) A/m in SI terms
    ));

    /// Statcoulomb - CGS-ESU unit of charge
    pub static ref STATCOULOMB: Unit = Unit::Base(BaseUnit::new(
        "statcoulomb", "statC", &["esu"],
        Dimension::CURRENT.mul(&Dimension::TIME),
        3.335641e-10
    ));

    /// Statampere - CGS-ESU unit of current
    pub static ref STATAMPERE: Unit = Unit::Base(BaseUnit::new(
        "statampere", "statA", &[],
        Dimension::CURRENT,
        3.335641e-10
    ));

    /// Statvolt - CGS-ESU unit of electric potential
    pub static ref STATVOLT: Unit = Unit::Base(BaseUnit::new(
        "statvolt", "statV", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational16::new(-3, 1)))
            .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1))),
        299.792458
    ));

    /// Franklin - same as statcoulomb
    pub static ref FRANKLIN: Unit = Unit::Base(BaseUnit::new(
        "franklin", "Fr", &[],
        Dimension::CURRENT.mul(&Dimension::TIME),
        3.335641e-10
    ));

    /// Biot - CGS-EMU unit of current (10 A)
    pub static ref BIOT: Unit = Unit::Base(BaseUnit::new(
        "biot", "Bi", &["abampere"],
        Dimension::CURRENT,
        10.0
    ));

    /// Abcoulomb - CGS-EMU unit of charge (10 C)
    pub static ref ABCOULOMB: Unit = Unit::Base(BaseUnit::new(
        "abcoulomb", "abC", &[],
        Dimension::CURRENT.mul(&Dimension::TIME),
        10.0
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{N, J, PA, T};

    #[test]
    fn test_dyne_to_newton() {
        let q = 1e5 * DYNE.clone();
        let q_n = q.to(&N).unwrap();
        assert!((q_n.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_erg_to_joule() {
        let q = 1e7 * ERG.clone();
        let q_j = q.to(&J).unwrap();
        assert!((q_j.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_to_tesla() {
        let q = 1e4 * GAUSS.clone();
        let q_t = q.to(&T).unwrap();
        assert!((q_t.value() - 1.0).abs() < 1e-10);
    }
}
