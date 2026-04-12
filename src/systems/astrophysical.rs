//! Astrophysical units.
//!
//! This module provides units commonly used in astronomy and astrophysics,
//! including distance units (parsec, AU, light-year), solar units, and
//! spectroscopic units.
//!
//! # Examples
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! // Distance to Proxima Centauri
//! let distance = 1.3 * PARSEC;
//! let in_ly = distance.to(LIGHT_YEAR).unwrap();
//! assert!((in_ly.value() - 4.24).abs() < 0.01);
//!
//! // 1 parsec in AU
//! let one_pc = 1.0 * PARSEC;
//! let in_au = one_pc.to(AU).unwrap();
//! assert!((in_au.value() - 206_265.0).abs() / 206_265.0 < 0.001);
//! ```

use crate::dimension::{Dimension, Rational16};
use crate::unit::base::BaseUnit;

// Physical constants for unit definitions (2018 CODATA values)
const C_M_S: f64 = 299_792_458.0; // m/s (exact)
const AU_M: f64 = 1.495_978_707e11; // m (exact, IAU 2012)
const PC_M: f64 = 3.085_677_581_491_367e16; // m (IAU 2015)
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

// Shared dimension constants
const DIM_ENERGY: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
const DIM_FORCE: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH)
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
const DIM_POWER: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-3, 1)));
const DIM_MAGNETIC_FIELD: Dimension = Dimension::MASS
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
    .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1)));
const DIM_AREA: Dimension = Dimension::LENGTH.pow(Rational16::new(2, 1));
const DIM_SPECTRAL_FLUX: Dimension =
    Dimension::MASS.mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));

// =============================================================================
// Distance Units
// =============================================================================

/// Astronomical Unit - mean Earth-Sun distance
pub const AU: BaseUnit = BaseUnit::new("astronomical_unit", "AU", &["au"], Dimension::LENGTH, AU_M);

/// Parsec - distance at which 1 AU subtends 1 arcsecond
pub const PARSEC: BaseUnit = BaseUnit::new("parsec", "pc", &[], Dimension::LENGTH, PC_M);

/// Kiloparsec (10^3 pc)
pub const KPC: BaseUnit = BaseUnit::new("kiloparsec", "kpc", &[], Dimension::LENGTH, PC_M * 1e3);

/// Megaparsec (10^6 pc)
pub const MPC: BaseUnit = BaseUnit::new("megaparsec", "Mpc", &[], Dimension::LENGTH, PC_M * 1e6);

/// Gigaparsec (10^9 pc)
pub const GPC: BaseUnit = BaseUnit::new("gigaparsec", "Gpc", &[], Dimension::LENGTH, PC_M * 1e9);

/// Light-year - distance light travels in one Julian year
pub const LIGHT_YEAR: BaseUnit = BaseUnit::new(
    "light_year",
    "lyr",
    &["ly", "lightyear"],
    Dimension::LENGTH,
    LY_M,
);

/// Light-second
pub const LIGHT_SECOND: BaseUnit =
    BaseUnit::new("light_second", "ls", &[], Dimension::LENGTH, C_M_S);

// =============================================================================
// Solar Units
// =============================================================================

/// Solar mass
pub const SOLAR_MASS: BaseUnit = BaseUnit::new(
    "solar_mass",
    "M_sun",
    &["Msun", "solMass", "M_sol"],
    Dimension::MASS,
    MSUN_KG,
);

/// Solar radius (nominal)
pub const SOLAR_RADIUS: BaseUnit = BaseUnit::new(
    "solar_radius",
    "R_sun",
    &["Rsun", "solRad", "R_sol"],
    Dimension::LENGTH,
    RSUN_M,
);

/// Solar luminosity (nominal)
pub const SOLAR_LUMINOSITY: BaseUnit = BaseUnit::new(
    "solar_luminosity",
    "L_sun",
    &["Lsun", "solLum", "L_sol"],
    DIM_POWER,
    LSUN_W,
);

// =============================================================================
// Planetary Units
// =============================================================================

/// Jupiter mass
pub const JUPITER_MASS: BaseUnit = BaseUnit::new(
    "jupiter_mass",
    "M_jup",
    &["Mjup", "jupiterMass"],
    Dimension::MASS,
    MJUP_KG,
);

/// Jupiter radius (equatorial, nominal)
pub const JUPITER_RADIUS: BaseUnit = BaseUnit::new(
    "jupiter_radius",
    "R_jup",
    &["Rjup", "jupiterRad"],
    Dimension::LENGTH,
    RJUP_M,
);

/// Earth mass
pub const EARTH_MASS: BaseUnit = BaseUnit::new(
    "earth_mass",
    "M_earth",
    &["Mearth", "earthMass"],
    Dimension::MASS,
    MEARTH_KG,
);

/// Earth radius (equatorial, nominal)
pub const EARTH_RADIUS: BaseUnit = BaseUnit::new(
    "earth_radius",
    "R_earth",
    &["Rearth", "earthRad"],
    Dimension::LENGTH,
    REARTH_M,
);

// =============================================================================
// Spectroscopic Units
// =============================================================================

/// Angstrom (10^-10 m) - common wavelength unit
pub const ANGSTROM: BaseUnit = BaseUnit::new(
    "angstrom",
    "Angstrom",
    &["AA", "angstrom"],
    Dimension::LENGTH,
    1e-10,
);

/// Jansky - spectral flux density (10^-26 W/m^2/Hz)
pub const JANSKY: BaseUnit = BaseUnit::new("jansky", "Jy", &[], DIM_SPECTRAL_FLUX, 1e-26);

/// Millijansky (10^-3 Jy)
pub const MJY: BaseUnit = BaseUnit::new("millijansky", "mJy", &[], DIM_SPECTRAL_FLUX, 1e-29);

/// Microjansky (10^-6 Jy)
pub const UJY: BaseUnit = BaseUnit::new("microjansky", "uJy", &[], DIM_SPECTRAL_FLUX, 1e-32);

/// Rayleigh - unit of photon flux (10^10 photons/m^2/s/sr)
pub const RAYLEIGH: BaseUnit = BaseUnit::new(
    "rayleigh",
    "R",
    &[],
    Dimension::PHOTON
        .mul(&Dimension::LENGTH.pow(Rational16::new(-2, 1)))
        .mul(&Dimension::TIME.pow(Rational16::new(-1, 1)))
        .mul(&Dimension::SOLID_ANGLE.pow(Rational16::new(-1, 1))),
    1e10,
);

// =============================================================================
// Cross-section Units
// =============================================================================

/// Barn - nuclear cross section (10^-28 m^2)
pub const BARN: BaseUnit = BaseUnit::new("barn", "barn", &["b"], DIM_AREA, 1e-28);

/// Millibarn (10^-3 barn)
pub const MBARN: BaseUnit = BaseUnit::new("millibarn", "mbarn", &["mb"], DIM_AREA, 1e-31);

/// Microbarn (10^-6 barn)
pub const UBARN: BaseUnit = BaseUnit::new("microbarn", "ubarn", &["ub"], DIM_AREA, 1e-34);

// =============================================================================
// CGS units commonly used in astrophysics
// =============================================================================

/// Erg (CGS energy, 10^-7 J)
pub const ERG: BaseUnit = BaseUnit::new("erg", "erg", &[], DIM_ENERGY, 1e-7);

/// Dyne (CGS force, 10^-5 N)
pub const DYN: BaseUnit = BaseUnit::new("dyne", "dyn", &[], DIM_FORCE, 1e-5);

/// Gauss (CGS magnetic field, 10^-4 T)
pub const GAUSS: BaseUnit = BaseUnit::new("gauss", "G", &["Gauss"], DIM_MAGNETIC_FIELD, 1e-4);

// =============================================================================
// CGS Spectral Flux Density Units
// =============================================================================

/// CGS spectral flux density per frequency: erg/s/cm²/Hz
pub const FLAM_NU: BaseUnit = BaseUnit::new(
    "erg_per_s_cm2_Hz",
    "erg/(s cm^2 Hz)",
    &["erg/s/cm2/Hz", "cgs_fnu"],
    DIM_SPECTRAL_FLUX,
    1e-3,
);

/// CGS spectral flux density per wavelength: erg/s/cm²/Å
pub const FLAM: BaseUnit = BaseUnit::new(
    "erg_per_s_cm2_angstrom",
    "erg/(s cm^2 Angstrom)",
    &["erg/s/cm2/A", "FLAM", "flam"],
    Dimension::MASS
        .mul(&Dimension::LENGTH.pow(Rational16::new(-1, 1)))
        .mul(&Dimension::TIME.pow(Rational16::new(-3, 1))),
    1e7,
);

/// Photon flux per wavelength: photon/s/cm²/Å
pub const PHOTLAM: BaseUnit = BaseUnit::new(
    "photon_per_s_cm2_angstrom",
    "photon/(s cm^2 Angstrom)",
    &["PHOTLAM", "photlam"],
    Dimension::PHOTON
        .mul(&Dimension::LENGTH.pow(Rational16::new(-3, 1)))
        .mul(&Dimension::TIME.pow(Rational16::new(-1, 1))),
    1e14,
);

/// Photon flux per frequency: photon/s/cm²/Hz
pub const PHOTNU: BaseUnit = BaseUnit::new(
    "photon_per_s_cm2_Hz",
    "photon/(s cm^2 Hz)",
    &["PHOTNU", "photnu"],
    Dimension::PHOTON
        .mul(&Dimension::LENGTH.pow(Rational16::new(-2, 1)))
        .mul(&Dimension::TIME.pow(Rational16::new(-2, 1))),
    1e4,
);

// =============================================================================
// Photon/Count Units
// =============================================================================

/// Photon
pub const PHOTON: BaseUnit = BaseUnit::new("photon", "ph", &["photon"], Dimension::PHOTON, 1.0);

/// Count (generic counting unit, dimensionless)
pub const COUNT: BaseUnit = BaseUnit::new("count", "ct", &["count", "cts"], Dimension::PHOTON, 1.0);

/// Electron (for detector counts)
pub const ELECTRON: BaseUnit =
    BaseUnit::new("electron", "e-", &["electron"], Dimension::PHOTON, 1.0);

// =============================================================================
// Time Units (Astrophysical)
// =============================================================================

/// Sidereal day (23h 56m 4.0905s)
pub const SIDEREAL_DAY: BaseUnit =
    BaseUnit::new("sidereal_day", "sday", &[], Dimension::TIME, 86_164.090_5);

/// Tropical year (365.24219 days)
pub const TROPICAL_YEAR: BaseUnit = BaseUnit::new(
    "tropical_year",
    "tyr",
    &[],
    Dimension::TIME,
    365.24219 * 86400.0,
);

/// Sidereal year (365.25636 days)
pub const SIDEREAL_YEAR: BaseUnit = BaseUnit::new(
    "sidereal_year",
    "syr",
    &[],
    Dimension::TIME,
    365.25636 * 86400.0,
);

// =============================================================================
// Cosmological Units
// =============================================================================

/// Hubble time (1/H0 with H0 = 70 km/s/Mpc, approximately)
pub const HUBBLE_TIME: BaseUnit = BaseUnit::new("hubble_time", "t_H", &[], Dimension::TIME, 4.4e17);

/// Hubble distance (c/H0 with H0 = 70 km/s/Mpc, approximately)
pub const HUBBLE_DISTANCE: BaseUnit =
    BaseUnit::new("hubble_distance", "d_H", &[], Dimension::LENGTH, 1.32e26);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{J, KG, M, W};

    #[test]
    fn test_parsec_to_meters() {
        let q = 1.0 * PARSEC;
        let q_m = q.to(M).unwrap();
        assert!((q_m.value() - 3.0856775814913673e16).abs() / 3.0856775814913673e16 < 1e-9);
    }

    #[test]
    fn test_light_year_to_meters() {
        let q = 1.0 * LIGHT_YEAR;
        let q_m = q.to(M).unwrap();
        assert!((q_m.value() - 9.4607304725808e15).abs() / 9.4607304725808e15 < 1e-9);
    }

    #[test]
    fn test_au_to_meters() {
        let q = 1.0 * AU;
        let q_m = q.to(M).unwrap();
        assert!((q_m.value() - 1.495978707e11).abs() < 1.0);
    }

    #[test]
    fn test_solar_mass_to_kg() {
        let q = 1.0 * SOLAR_MASS;
        let q_kg = q.to(KG).unwrap();
        assert!((q_kg.value() - 1.98840987e30).abs() / 1.98840987e30 < 1e-6);
    }

    #[test]
    fn test_solar_luminosity_to_watts() {
        let q = 1.0 * SOLAR_LUMINOSITY;
        let q_w = q.to(W).unwrap();
        assert!((q_w.value() - 3.828e26).abs() / 3.828e26 < 1e-6);
    }

    #[test]
    fn test_angstrom_to_nm() {
        let q = 10.0 * ANGSTROM;
        let q_m = q.to(M).unwrap();
        assert!((q_m.value() - 1e-9).abs() < 1e-20);
    }

    #[test]
    fn test_erg_to_joule() {
        let q = 1e7 * ERG;
        let q_j = q.to(J).unwrap();
        assert!((q_j.value() - 1.0).abs() < 1e-10);
    }
}
