//! Astrophysical units.
//!
//! This module provides units commonly used in astronomy and astrophysics,
//! including distance units (parsec, AU, light-year), solar units, and
//! spectroscopic units.

use crate::dimension::{Dimension, Rational8};
use crate::unit::base::BaseUnit;
use crate::unit::Unit;
use lazy_static::lazy_static;

// Physical constants for unit definitions (2018 CODATA values)
const C_M_S: f64 = 299_792_458.0; // m/s (exact)
const AU_M: f64 = 1.495_978_707e11; // m (exact, IAU 2012)
const PC_M: f64 = 3.085_677_581_491_367_3e16; // m (IAU 2015)
const LY_M: f64 = 9.460_730_472_580_8e15; // m (Julian year)

// Solar units (IAU 2015 Resolution B3)
const MSUN_KG: f64 = 1.988_409_870_698_051e30; // kg
const RSUN_M: f64 = 6.957e8; // m (nominal)
const LSUN_W: f64 = 3.828e26; // W (nominal)

// Other astrophysical constants
const MJUP_KG: f64 = 1.898_13e27; // kg
const RJUP_M: f64 = 6.991_1e7; // m (equatorial, nominal)
const MEARTH_KG: f64 = 5.972_17e24; // kg
const REARTH_M: f64 = 6.378_1e6; // m (equatorial, nominal)

lazy_static! {
    // =============================================================================
    // Distance Units
    // =============================================================================

    /// Astronomical Unit - mean Earth-Sun distance
    pub static ref AU: Unit = Unit::Base(BaseUnit::new(
        "astronomical_unit", "AU", &["au"],
        Dimension::LENGTH,
        AU_M
    ));

    /// Parsec - distance at which 1 AU subtends 1 arcsecond
    pub static ref PARSEC: Unit = Unit::Base(BaseUnit::new(
        "parsec", "pc", &[],
        Dimension::LENGTH,
        PC_M
    ));

    /// Kiloparsec (10^3 pc)
    pub static ref KPC: Unit = Unit::Base(BaseUnit::new(
        "kiloparsec", "kpc", &[],
        Dimension::LENGTH,
        PC_M * 1e3
    ));

    /// Megaparsec (10^6 pc)
    pub static ref MPC: Unit = Unit::Base(BaseUnit::new(
        "megaparsec", "Mpc", &[],
        Dimension::LENGTH,
        PC_M * 1e6
    ));

    /// Gigaparsec (10^9 pc)
    pub static ref GPC: Unit = Unit::Base(BaseUnit::new(
        "gigaparsec", "Gpc", &[],
        Dimension::LENGTH,
        PC_M * 1e9
    ));

    /// Light-year - distance light travels in one Julian year
    pub static ref LIGHT_YEAR: Unit = Unit::Base(BaseUnit::new(
        "light_year", "lyr", &["ly", "lightyear"],
        Dimension::LENGTH,
        LY_M
    ));

    /// Light-second
    pub static ref LIGHT_SECOND: Unit = Unit::Base(BaseUnit::new(
        "light_second", "ls", &[],
        Dimension::LENGTH,
        C_M_S
    ));

    // =============================================================================
    // Solar Units
    // =============================================================================

    /// Solar mass
    pub static ref SOLAR_MASS: Unit = Unit::Base(BaseUnit::new(
        "solar_mass", "M_sun", &["Msun", "solMass", "M_sol"],
        Dimension::MASS,
        MSUN_KG
    ));

    /// Solar radius (nominal)
    pub static ref SOLAR_RADIUS: Unit = Unit::Base(BaseUnit::new(
        "solar_radius", "R_sun", &["Rsun", "solRad", "R_sol"],
        Dimension::LENGTH,
        RSUN_M
    ));

    /// Solar luminosity (nominal)
    pub static ref SOLAR_LUMINOSITY: Unit = Unit::Base(BaseUnit::new(
        "solar_luminosity", "L_sun", &["Lsun", "solLum", "L_sol"],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-3, 1))),
        LSUN_W
    ));

    // =============================================================================
    // Planetary Units
    // =============================================================================

    /// Jupiter mass
    pub static ref JUPITER_MASS: Unit = Unit::Base(BaseUnit::new(
        "jupiter_mass", "M_jup", &["Mjup", "jupiterMass"],
        Dimension::MASS,
        MJUP_KG
    ));

    /// Jupiter radius (equatorial, nominal)
    pub static ref JUPITER_RADIUS: Unit = Unit::Base(BaseUnit::new(
        "jupiter_radius", "R_jup", &["Rjup", "jupiterRad"],
        Dimension::LENGTH,
        RJUP_M
    ));

    /// Earth mass
    pub static ref EARTH_MASS: Unit = Unit::Base(BaseUnit::new(
        "earth_mass", "M_earth", &["Mearth", "earthMass"],
        Dimension::MASS,
        MEARTH_KG
    ));

    /// Earth radius (equatorial, nominal)
    pub static ref EARTH_RADIUS: Unit = Unit::Base(BaseUnit::new(
        "earth_radius", "R_earth", &["Rearth", "earthRad"],
        Dimension::LENGTH,
        REARTH_M
    ));

    // =============================================================================
    // Spectroscopic Units
    // =============================================================================

    /// Angstrom (10^-10 m) - common wavelength unit
    pub static ref ANGSTROM: Unit = Unit::Base(BaseUnit::new(
        "angstrom", "Angstrom", &["AA", "angstrom"],
        Dimension::LENGTH,
        1e-10
    ));

    /// Jansky - spectral flux density (10^-26 W/m^2/Hz)
    pub static ref JANSKY: Unit = Unit::Base(BaseUnit::new(
        "jansky", "Jy", &[],
        Dimension::MASS
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        1e-26
    ));

    /// Millijansky (10^-3 Jy)
    pub static ref MJY: Unit = Unit::Base(BaseUnit::new(
        "millijansky", "mJy", &[],
        Dimension::MASS
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        1e-29
    ));

    /// Microjansky (10^-6 Jy)
    pub static ref UJY: Unit = Unit::Base(BaseUnit::new(
        "microjansky", "uJy", &[],
        Dimension::MASS
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        1e-32
    ));

    /// Rayleigh - unit of photon flux (10^10 photons/m^2/s/sr)
    pub static ref RAYLEIGH: Unit = Unit::Base(BaseUnit::new(
        "rayleigh", "R", &[],
        Dimension::PHOTON
            .mul(&Dimension::LENGTH.pow(Rational8::new(-2, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-1, 1)))
            .mul(&Dimension::SOLID_ANGLE.pow(Rational8::new(-1, 1))),
        1e10
    ));

    // =============================================================================
    // Cross-section Units
    // =============================================================================

    /// Barn - nuclear cross section (10^-28 m^2)
    pub static ref BARN: Unit = Unit::Base(BaseUnit::new(
        "barn", "barn", &["b"],
        Dimension::LENGTH.pow(Rational8::new(2, 1)),
        1e-28
    ));

    /// Millibarn (10^-3 barn)
    pub static ref MBARN: Unit = Unit::Base(BaseUnit::new(
        "millibarn", "mbarn", &["mb"],
        Dimension::LENGTH.pow(Rational8::new(2, 1)),
        1e-31
    ));

    /// Microbarn (10^-6 barn)
    pub static ref UBARN: Unit = Unit::Base(BaseUnit::new(
        "microbarn", "ubarn", &["ub"],
        Dimension::LENGTH.pow(Rational8::new(2, 1)),
        1e-34
    ));

    // =============================================================================
    // CGS units commonly used in astrophysics
    // =============================================================================

    /// Erg (CGS energy, 10^-7 J)
    pub static ref ERG: Unit = Unit::Base(BaseUnit::new(
        "erg", "erg", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        1e-7
    ));

    /// Dyne (CGS force, 10^-5 N)
    pub static ref DYN: Unit = Unit::Base(BaseUnit::new(
        "dyne", "dyn", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH)
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        1e-5
    ));

    /// Gauss (CGS magnetic field, 10^-4 T)
    pub static ref GAUSS: Unit = Unit::Base(BaseUnit::new(
        "gauss", "G", &["Gauss"],
        Dimension::MASS
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1)))
            .mul(&Dimension::CURRENT.pow(Rational8::new(-1, 1))),
        1e-4
    ));

    // =============================================================================
    // Photon/Count Units
    // =============================================================================

    /// Photon
    pub static ref PHOTON: Unit = Unit::Base(BaseUnit::new(
        "photon", "ph", &["photon"],
        Dimension::PHOTON,
        1.0
    ));

    /// Count (generic counting unit, dimensionless)
    pub static ref COUNT: Unit = Unit::Base(BaseUnit::new(
        "count", "ct", &["count", "cts"],
        Dimension::PHOTON,
        1.0
    ));

    /// Electron (for detector counts)
    pub static ref ELECTRON: Unit = Unit::Base(BaseUnit::new(
        "electron", "e-", &["electron"],
        Dimension::PHOTON,
        1.0
    ));

    // =============================================================================
    // Time Units (Astrophysical)
    // =============================================================================

    /// Sidereal day (23h 56m 4.0905s)
    pub static ref SIDEREAL_DAY: Unit = Unit::Base(BaseUnit::new(
        "sidereal_day", "sday", &[],
        Dimension::TIME,
        86164.0905
    ));

    /// Tropical year (365.24219 days)
    pub static ref TROPICAL_YEAR: Unit = Unit::Base(BaseUnit::new(
        "tropical_year", "tyr", &[],
        Dimension::TIME,
        365.24219 * 86400.0
    ));

    /// Sidereal year (365.25636 days)
    pub static ref SIDEREAL_YEAR: Unit = Unit::Base(BaseUnit::new(
        "sidereal_year", "syr", &[],
        Dimension::TIME,
        365.25636 * 86400.0
    ));

    // =============================================================================
    // Cosmological Units
    // =============================================================================

    /// Hubble time (1/H0 with H0 = 70 km/s/Mpc, approximately)
    /// Note: This is approximate; actual value depends on cosmology
    pub static ref HUBBLE_TIME: Unit = Unit::Base(BaseUnit::new(
        "hubble_time", "t_H", &[],
        Dimension::TIME,
        4.4e17  // ~14 Gyr in seconds
    ));

    /// Hubble distance (c/H0 with H0 = 70 km/s/Mpc, approximately)
    pub static ref HUBBLE_DISTANCE: Unit = Unit::Base(BaseUnit::new(
        "hubble_distance", "d_H", &[],
        Dimension::LENGTH,
        1.32e26  // ~4.4 Gpc in meters
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{M, KG, W, J};

    #[test]
    fn test_parsec_to_meters() {
        let q = 1.0 * PARSEC.clone();
        let q_m = q.to(&M).unwrap();
        // 1 pc ≈ 3.086e16 m
        assert!((q_m.value() - 3.0856775814913673e16).abs() / 3.0856775814913673e16 < 1e-9);
    }

    #[test]
    fn test_light_year_to_meters() {
        let q = 1.0 * LIGHT_YEAR.clone();
        let q_m = q.to(&M).unwrap();
        // 1 lyr ≈ 9.461e15 m
        assert!((q_m.value() - 9.4607304725808e15).abs() / 9.4607304725808e15 < 1e-9);
    }

    #[test]
    fn test_au_to_meters() {
        let q = 1.0 * AU.clone();
        let q_m = q.to(&M).unwrap();
        // 1 AU = 1.495978707e11 m (exact)
        assert!((q_m.value() - 1.495978707e11).abs() < 1.0);
    }

    #[test]
    fn test_solar_mass_to_kg() {
        let q = 1.0 * SOLAR_MASS.clone();
        let q_kg = q.to(&KG).unwrap();
        assert!((q_kg.value() - 1.98840987e30).abs() / 1.98840987e30 < 1e-6);
    }

    #[test]
    fn test_solar_luminosity_to_watts() {
        let q = 1.0 * SOLAR_LUMINOSITY.clone();
        let q_w = q.to(&W).unwrap();
        assert!((q_w.value() - 3.828e26).abs() / 3.828e26 < 1e-6);
    }

    #[test]
    fn test_angstrom_to_nm() {
        let q = 10.0 * ANGSTROM.clone();
        let q_m = q.to(&M).unwrap();
        // 10 Angstrom = 1 nm = 1e-9 m
        assert!((q_m.value() - 1e-9).abs() < 1e-20);
    }

    #[test]
    fn test_erg_to_joule() {
        let q = 1e7 * ERG.clone();
        let q_j = q.to(&J).unwrap();
        assert!((q_j.value() - 1.0).abs() < 1e-10);
    }
}
