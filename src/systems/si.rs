//! SI (International System of Units) units.
//!
//! This module provides the seven SI base units plus derived units and prefixed variants.

use crate::dimension::Dimension;
use crate::unit::base::BaseUnit;
use crate::unit::Unit;
use lazy_static::lazy_static;

// =============================================================================
// SI Base Units
// =============================================================================

/// Meter - SI base unit of length
pub static METER: BaseUnit = BaseUnit::new("meter", "m", &["metre"], Dimension::LENGTH, 1.0);

/// Second - SI base unit of time
pub static SECOND: BaseUnit = BaseUnit::new("second", "s", &["sec"], Dimension::TIME, 1.0);

/// Kilogram - SI base unit of mass
pub static KILOGRAM: BaseUnit = BaseUnit::new("kilogram", "kg", &[], Dimension::MASS, 1.0);

/// Ampere - SI base unit of electric current
pub static AMPERE: BaseUnit = BaseUnit::new("ampere", "A", &["amp"], Dimension::CURRENT, 1.0);

/// Kelvin - SI base unit of temperature
pub static KELVIN: BaseUnit = BaseUnit::new("kelvin", "K", &[], Dimension::TEMPERATURE, 1.0);

/// Mole - SI base unit of amount of substance
pub static MOLE: BaseUnit = BaseUnit::new("mole", "mol", &[], Dimension::AMOUNT, 1.0);

/// Candela - SI base unit of luminous intensity
pub static CANDELA: BaseUnit = BaseUnit::new("candela", "cd", &[], Dimension::LUMINOUS_INTENSITY, 1.0);

/// Radian - SI unit of angle (dimensionless in SI, but we track it)
pub static RADIAN: BaseUnit = BaseUnit::new("radian", "rad", &[], Dimension::ANGLE, 1.0);

/// Steradian - SI unit of solid angle
pub static STERADIAN: BaseUnit = BaseUnit::new("steradian", "sr", &[], Dimension::SOLID_ANGLE, 1.0);

// =============================================================================
// Convenient Unit Constants (uppercase for easy access)
// =============================================================================

lazy_static! {
    // Base units
    pub static ref M: Unit = Unit::Base(METER.clone());
    pub static ref S: Unit = Unit::Base(SECOND.clone());
    pub static ref KG: Unit = Unit::Base(KILOGRAM.clone());
    pub static ref A: Unit = Unit::Base(AMPERE.clone());
    pub static ref K: Unit = Unit::Base(KELVIN.clone());
    pub static ref MOL: Unit = Unit::Base(MOLE.clone());
    pub static ref CD: Unit = Unit::Base(CANDELA.clone());
    pub static ref RAD: Unit = Unit::Base(RADIAN.clone());
    pub static ref SR: Unit = Unit::Base(STERADIAN.clone());

    // =============================================================================
    // Length units with SI prefixes
    // =============================================================================

    /// Kilometer (10^3 m)
    pub static ref KM: Unit = Unit::Base(BaseUnit::new(
        "kilometer", "km", &[], Dimension::LENGTH, 1e3
    ));

    /// Centimeter (10^-2 m)
    pub static ref CM: Unit = Unit::Base(BaseUnit::new(
        "centimeter", "cm", &[], Dimension::LENGTH, 1e-2
    ));

    /// Millimeter (10^-3 m)
    pub static ref MM: Unit = Unit::Base(BaseUnit::new(
        "millimeter", "mm", &[], Dimension::LENGTH, 1e-3
    ));

    /// Micrometer (10^-6 m)
    pub static ref UM: Unit = Unit::Base(BaseUnit::new(
        "micrometer", "um", &["micron"], Dimension::LENGTH, 1e-6
    ));

    /// Nanometer (10^-9 m)
    pub static ref NM: Unit = Unit::Base(BaseUnit::new(
        "nanometer", "nm", &[], Dimension::LENGTH, 1e-9
    ));

    /// Picometer (10^-12 m)
    pub static ref PM: Unit = Unit::Base(BaseUnit::new(
        "picometer", "pm", &[], Dimension::LENGTH, 1e-12
    ));

    /// Femtometer (10^-15 m)
    pub static ref FM: Unit = Unit::Base(BaseUnit::new(
        "femtometer", "fm", &[], Dimension::LENGTH, 1e-15
    ));

    // =============================================================================
    // Time units
    // =============================================================================

    /// Millisecond (10^-3 s)
    pub static ref MS: Unit = Unit::Base(BaseUnit::new(
        "millisecond", "ms", &[], Dimension::TIME, 1e-3
    ));

    /// Microsecond (10^-6 s)
    pub static ref US: Unit = Unit::Base(BaseUnit::new(
        "microsecond", "us", &[], Dimension::TIME, 1e-6
    ));

    /// Nanosecond (10^-9 s)
    pub static ref NS: Unit = Unit::Base(BaseUnit::new(
        "nanosecond", "ns", &[], Dimension::TIME, 1e-9
    ));

    /// Picosecond (10^-12 s)
    pub static ref PS: Unit = Unit::Base(BaseUnit::new(
        "picosecond", "ps", &[], Dimension::TIME, 1e-12
    ));

    /// Minute (60 s)
    pub static ref MIN: Unit = Unit::Base(BaseUnit::new(
        "minute", "min", &[], Dimension::TIME, 60.0
    ));

    /// Hour (3600 s)
    pub static ref H: Unit = Unit::Base(BaseUnit::new(
        "hour", "h", &["hr"], Dimension::TIME, 3600.0
    ));

    /// Day (86400 s)
    pub static ref DAY: Unit = Unit::Base(BaseUnit::new(
        "day", "d", &["day"], Dimension::TIME, 86400.0
    ));

    /// Julian year (365.25 days)
    pub static ref YR: Unit = Unit::Base(BaseUnit::new(
        "year", "yr", &["a", "year"], Dimension::TIME, 365.25 * 86400.0
    ));

    // =============================================================================
    // Mass units
    // =============================================================================

    /// Gram (10^-3 kg)
    pub static ref G: Unit = Unit::Base(BaseUnit::new(
        "gram", "g", &[], Dimension::MASS, 1e-3
    ));

    /// Milligram (10^-6 kg)
    pub static ref MG: Unit = Unit::Base(BaseUnit::new(
        "milligram", "mg", &[], Dimension::MASS, 1e-6
    ));

    /// Microgram (10^-9 kg)
    pub static ref UG: Unit = Unit::Base(BaseUnit::new(
        "microgram", "ug", &[], Dimension::MASS, 1e-9
    ));

    /// Tonne / metric ton (10^3 kg)
    pub static ref TONNE: Unit = Unit::Base(BaseUnit::new(
        "tonne", "t", &["metric_ton"], Dimension::MASS, 1e3
    ));

    // =============================================================================
    // Derived SI Units
    // =============================================================================

    /// Hertz - frequency (1/s)
    pub static ref HZ: Unit = Unit::Base(BaseUnit::new(
        "hertz", "Hz", &[], Dimension::TIME.inv(), 1.0
    ));

    /// Kilohertz (10^3 Hz)
    pub static ref KHZ: Unit = Unit::Base(BaseUnit::new(
        "kilohertz", "kHz", &[], Dimension::TIME.inv(), 1e3
    ));

    /// Megahertz (10^6 Hz)
    pub static ref MHZ: Unit = Unit::Base(BaseUnit::new(
        "megahertz", "MHz", &[], Dimension::TIME.inv(), 1e6
    ));

    /// Gigahertz (10^9 Hz)
    pub static ref GHZ: Unit = Unit::Base(BaseUnit::new(
        "gigahertz", "GHz", &[], Dimension::TIME.inv(), 1e9
    ));

    /// Terahertz (10^12 Hz)
    pub static ref THZ: Unit = Unit::Base(BaseUnit::new(
        "terahertz", "THz", &[], Dimension::TIME.inv(), 1e12
    ));

    /// Newton - force (kg m / s^2)
    pub static ref N: Unit = Unit::Base(BaseUnit::new(
        "newton", "N", &[],
        Dimension::MASS.mul(&Dimension::LENGTH).mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Joule - energy (kg m^2 / s^2)
    pub static ref J: Unit = Unit::Base(BaseUnit::new(
        "joule", "J", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Electronvolt (1.602176634e-19 J)
    pub static ref EV: Unit = Unit::Base(BaseUnit::new(
        "electronvolt", "eV", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.602176634e-19
    ));

    /// Kiloelectronvolt (10^3 eV)
    pub static ref KEV: Unit = Unit::Base(BaseUnit::new(
        "kiloelectronvolt", "keV", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.602176634e-16
    ));

    /// Megaelectronvolt (10^6 eV)
    pub static ref MEV: Unit = Unit::Base(BaseUnit::new(
        "megaelectronvolt", "MeV", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.602176634e-13
    ));

    /// Gigaelectronvolt (10^9 eV)
    pub static ref GEV: Unit = Unit::Base(BaseUnit::new(
        "gigaelectronvolt", "GeV", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.602176634e-10
    ));

    /// Watt - power (kg m^2 / s^3)
    pub static ref W: Unit = Unit::Base(BaseUnit::new(
        "watt", "W", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-3, 1))),
        1.0
    ));

    /// Kilowatt (10^3 W)
    pub static ref KW: Unit = Unit::Base(BaseUnit::new(
        "kilowatt", "kW", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-3, 1))),
        1e3
    ));

    /// Megawatt (10^6 W)
    pub static ref MW: Unit = Unit::Base(BaseUnit::new(
        "megawatt", "MW", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-3, 1))),
        1e6
    ));

    /// Pascal - pressure (kg / m / s^2)
    pub static ref PA: Unit = Unit::Base(BaseUnit::new(
        "pascal", "Pa", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(-1, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Coulomb - electric charge (A s)
    pub static ref C: Unit = Unit::Base(BaseUnit::new(
        "coulomb", "C", &[],
        Dimension::CURRENT.mul(&Dimension::TIME),
        1.0
    ));

    /// Volt - electric potential (kg m^2 / A / s^3)
    pub static ref V: Unit = Unit::Base(BaseUnit::new(
        "volt", "V", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-3, 1)))
            .mul(&Dimension::CURRENT.pow(crate::dimension::Rational16::new(-1, 1))),
        1.0
    ));

    /// Farad - capacitance (A^2 s^4 / kg / m^2)
    pub static ref F: Unit = Unit::Base(BaseUnit::new(
        "farad", "F", &[],
        Dimension::CURRENT.pow(crate::dimension::Rational16::new(2, 1))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(4, 1)))
            .mul(&Dimension::MASS.pow(crate::dimension::Rational16::new(-1, 1)))
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Ohm - electrical resistance (kg m^2 / A^2 / s^3)
    pub static ref OHM: Unit = Unit::Base(BaseUnit::new(
        "ohm", "Ohm", &["ohm"],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-3, 1)))
            .mul(&Dimension::CURRENT.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Siemens - electrical conductance (A^2 s^3 / kg / m^2)
    pub static ref SIEMENS: Unit = Unit::Base(BaseUnit::new(
        "siemens", "S", &[],
        Dimension::CURRENT.pow(crate::dimension::Rational16::new(2, 1))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(3, 1)))
            .mul(&Dimension::MASS.pow(crate::dimension::Rational16::new(-1, 1)))
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Weber - magnetic flux (kg m^2 / A / s^2)
    pub static ref WB: Unit = Unit::Base(BaseUnit::new(
        "weber", "Wb", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1)))
            .mul(&Dimension::CURRENT.pow(crate::dimension::Rational16::new(-1, 1))),
        1.0
    ));

    /// Tesla - magnetic field (kg / A / s^2)
    pub static ref T: Unit = Unit::Base(BaseUnit::new(
        "tesla", "T", &[],
        Dimension::MASS
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1)))
            .mul(&Dimension::CURRENT.pow(crate::dimension::Rational16::new(-1, 1))),
        1.0
    ));

    /// Henry - inductance (kg m^2 / A^2 / s^2)
    pub static ref HENRY: Unit = Unit::Base(BaseUnit::new(
        "henry", "H", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1)))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1)))
            .mul(&Dimension::CURRENT.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Lumen - luminous flux (cd sr)
    pub static ref LM: Unit = Unit::Base(BaseUnit::new(
        "lumen", "lm", &[],
        Dimension::LUMINOUS_INTENSITY.mul(&Dimension::SOLID_ANGLE),
        1.0
    ));

    /// Lux - illuminance (cd sr / m^2)
    pub static ref LX: Unit = Unit::Base(BaseUnit::new(
        "lux", "lx", &[],
        Dimension::LUMINOUS_INTENSITY
            .mul(&Dimension::SOLID_ANGLE)
            .mul(&Dimension::LENGTH.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Becquerel - radioactivity (1/s)
    pub static ref BQ: Unit = Unit::Base(BaseUnit::new(
        "becquerel", "Bq", &[],
        Dimension::TIME.inv(),
        1.0
    ));

    /// Gray - absorbed dose (m^2 / s^2)
    pub static ref GY: Unit = Unit::Base(BaseUnit::new(
        "gray", "Gy", &[],
        Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    /// Sievert - equivalent dose (m^2 / s^2)
    pub static ref SV: Unit = Unit::Base(BaseUnit::new(
        "sievert", "Sv", &[],
        Dimension::LENGTH.pow(crate::dimension::Rational16::new(2, 1))
            .mul(&Dimension::TIME.pow(crate::dimension::Rational16::new(-2, 1))),
        1.0
    ));

    // =============================================================================
    // Angle units
    // =============================================================================

    /// Degree (pi/180 rad)
    pub static ref DEG: Unit = Unit::Base(BaseUnit::new(
        "degree", "deg", &["degree"],
        Dimension::ANGLE,
        std::f64::consts::PI / 180.0
    ));

    /// Arcminute (1/60 degree)
    pub static ref ARCMIN: Unit = Unit::Base(BaseUnit::new(
        "arcminute", "arcmin", &["arcminute"],
        Dimension::ANGLE,
        std::f64::consts::PI / 180.0 / 60.0
    ));

    /// Arcsecond (1/3600 degree)
    pub static ref ARCSEC: Unit = Unit::Base(BaseUnit::new(
        "arcsecond", "arcsec", &["arcsecond"],
        Dimension::ANGLE,
        std::f64::consts::PI / 180.0 / 3600.0
    ));

    /// Milliarcsecond (10^-3 arcsec)
    pub static ref MAS: Unit = Unit::Base(BaseUnit::new(
        "milliarcsecond", "mas", &[],
        Dimension::ANGLE,
        std::f64::consts::PI / 180.0 / 3600.0 / 1000.0
    ));

    /// Microarcsecond (10^-6 arcsec)
    pub static ref UAS: Unit = Unit::Base(BaseUnit::new(
        "microarcsecond", "uas", &[],
        Dimension::ANGLE,
        std::f64::consts::PI / 180.0 / 3600.0 / 1e6
    ));

    /// Hour angle
    pub static ref HOURANGLE: Unit = Unit::Base(BaseUnit::new(
        "hourangle", "hourangle", &[],
        Dimension::ANGLE,
        std::f64::consts::PI / 12.0
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_km_to_m() {
        let q = 1.0 * KM.clone();
        let q_m = q.to(&M).unwrap();
        assert!((q_m.value() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_hour_to_second() {
        let q = 1.0 * H.clone();
        let q_s = q.to(&S).unwrap();
        assert!((q_s.value() - 3600.0).abs() < 1e-10);
    }

    #[test]
    fn test_degree_to_radian() {
        let q = 180.0 * DEG.clone();
        let q_rad = q.to(&RAD).unwrap();
        assert!((q_rad.value() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_energy_unit() {
        // 1 J = 1 kg m^2 / s^2
        let energy = 1.0 * J.clone();
        let dim = energy.unit().dimension();
        assert_eq!(dim.mass, crate::dimension::Rational16::ONE);
        assert_eq!(dim.length, crate::dimension::Rational16::new(2, 1));
        assert_eq!(dim.time, crate::dimension::Rational16::new(-2, 1));
    }
}
