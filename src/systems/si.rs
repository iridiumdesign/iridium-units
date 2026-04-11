//! SI (International System of Units) units.
//!
//! This module provides the seven SI base units plus derived units and prefixed variants.
//!
//! All units are `const` values of type [`BaseUnit`], which is `Copy`. This means
//! you can use them directly without dereferencing:
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! let distance = 100.0 * M;
//! let speed = 10.0 * (M / S);
//! ```

use crate::dimension::{Dimension, Rational16};
use crate::unit::base::BaseUnit;

// =============================================================================
// SI Base Units (long-form names)
// =============================================================================

/// Meter - SI base unit of length
pub const METER: BaseUnit = BaseUnit::new("meter", "m", &["metre"], Dimension::LENGTH, 1.0);

/// Second - SI base unit of time
pub const SECOND: BaseUnit = BaseUnit::new("second", "s", &["sec"], Dimension::TIME, 1.0);

/// Kilogram - SI base unit of mass
pub const KILOGRAM: BaseUnit = BaseUnit::new("kilogram", "kg", &[], Dimension::MASS, 1.0);

/// Ampere - SI base unit of electric current
pub const AMPERE: BaseUnit = BaseUnit::new("ampere", "A", &["amp"], Dimension::CURRENT, 1.0);

/// Kelvin - SI base unit of temperature
pub const KELVIN: BaseUnit = BaseUnit::new("kelvin", "K", &[], Dimension::TEMPERATURE, 1.0);

/// Mole - SI base unit of amount of substance
pub const MOLE: BaseUnit = BaseUnit::new("mole", "mol", &[], Dimension::AMOUNT, 1.0);

/// Candela - SI base unit of luminous intensity
pub const CANDELA: BaseUnit = BaseUnit::new("candela", "cd", &[], Dimension::LUMINOUS_INTENSITY, 1.0);

/// Radian - SI unit of angle (dimensionless in SI, but we track it)
pub const RADIAN: BaseUnit = BaseUnit::new("radian", "rad", &[], Dimension::ANGLE, 1.0);

/// Steradian - SI unit of solid angle
pub const STERADIAN: BaseUnit = BaseUnit::new("steradian", "sr", &[], Dimension::SOLID_ANGLE, 1.0);

// =============================================================================
// Short-form base unit aliases
// =============================================================================

/// Meter (m)
pub const M: BaseUnit = METER;
/// Second (s)
pub const S: BaseUnit = SECOND;
/// Kilogram (kg)
pub const KG: BaseUnit = KILOGRAM;
/// Ampere (A)
pub const A: BaseUnit = AMPERE;
/// Kelvin (K)
pub const K: BaseUnit = KELVIN;
/// Mole (mol)
pub const MOL: BaseUnit = MOLE;
/// Candela (cd)
pub const CD: BaseUnit = CANDELA;
/// Radian (rad)
pub const RAD: BaseUnit = RADIAN;
/// Steradian (sr)
pub const SR: BaseUnit = STERADIAN;

// =============================================================================
// Temperature units (offset scales — conversions handled natively via unit offset)
// =============================================================================

/// Degree Celsius (offset from Kelvin by +273.15)
/// K = (°C + 273.15) × 1.0
pub const DEG_C: BaseUnit = BaseUnit::with_offset(
    "celsius", "°C", &["degC", "Celsius"],
    Dimension::TEMPERATURE,
    1.0,     // 1°C interval = 1K interval
    273.15,  // K = °C + 273.15
);

/// Degree Fahrenheit (offset from Rankine by +459.67)
/// K = (°F + 459.67) × 5/9
pub const DEG_F: BaseUnit = BaseUnit::with_offset(
    "fahrenheit", "°F", &["degF", "Fahrenheit"],
    Dimension::TEMPERATURE,
    5.0 / 9.0,  // 1°F interval = 5/9 K interval
    459.67,      // K = (°F + 459.67) × 5/9
);

// =============================================================================
// Length units with SI prefixes
// =============================================================================

/// Kilometer (10^3 m)
pub const KM: BaseUnit = BaseUnit::new("kilometer", "km", &[], Dimension::LENGTH, 1e3);

/// Centimeter (10^-2 m)
pub const CM: BaseUnit = BaseUnit::new("centimeter", "cm", &[], Dimension::LENGTH, 1e-2);

/// Millimeter (10^-3 m)
pub const MM: BaseUnit = BaseUnit::new("millimeter", "mm", &[], Dimension::LENGTH, 1e-3);

/// Micrometer (10^-6 m)
pub const UM: BaseUnit = BaseUnit::new("micrometer", "um", &["micron"], Dimension::LENGTH, 1e-6);

/// Nanometer (10^-9 m)
pub const NM: BaseUnit = BaseUnit::new("nanometer", "nm", &[], Dimension::LENGTH, 1e-9);

/// Picometer (10^-12 m)
pub const PM: BaseUnit = BaseUnit::new("picometer", "pm", &[], Dimension::LENGTH, 1e-12);

/// Femtometer (10^-15 m)
pub const FM: BaseUnit = BaseUnit::new("femtometer", "fm", &[], Dimension::LENGTH, 1e-15);

// =============================================================================
// Time units
// =============================================================================

/// Millisecond (10^-3 s)
pub const MS: BaseUnit = BaseUnit::new("millisecond", "ms", &[], Dimension::TIME, 1e-3);

/// Microsecond (10^-6 s)
pub const US: BaseUnit = BaseUnit::new("microsecond", "us", &[], Dimension::TIME, 1e-6);

/// Nanosecond (10^-9 s)
pub const NS: BaseUnit = BaseUnit::new("nanosecond", "ns", &[], Dimension::TIME, 1e-9);

/// Picosecond (10^-12 s)
pub const PS: BaseUnit = BaseUnit::new("picosecond", "ps", &[], Dimension::TIME, 1e-12);

/// Minute (60 s)
pub const MIN: BaseUnit = BaseUnit::new("minute", "min", &[], Dimension::TIME, 60.0);

/// Hour (3600 s)
pub const H: BaseUnit = BaseUnit::new("hour", "h", &["hr"], Dimension::TIME, 3600.0);

/// Day (86400 s)
pub const DAY: BaseUnit = BaseUnit::new("day", "d", &["day"], Dimension::TIME, 86400.0);

/// Julian year (365.25 days)
pub const YR: BaseUnit = BaseUnit::new("year", "yr", &["a", "year"], Dimension::TIME, 365.25 * 86400.0);

// =============================================================================
// Mass units
// =============================================================================

/// Gram (10^-3 kg)
pub const G: BaseUnit = BaseUnit::new("gram", "g", &[], Dimension::MASS, 1e-3);

/// Milligram (10^-6 kg)
pub const MG: BaseUnit = BaseUnit::new("milligram", "mg", &[], Dimension::MASS, 1e-6);

/// Microgram (10^-9 kg)
pub const UG: BaseUnit = BaseUnit::new("microgram", "ug", &[], Dimension::MASS, 1e-9);

/// Tonne / metric ton (10^3 kg)
pub const TONNE: BaseUnit = BaseUnit::new("tonne", "t", &["metric_ton"], Dimension::MASS, 1e3);

// =============================================================================
// Derived SI Units
// =============================================================================

/// Dimension: frequency (T^-1)
const DIM_FREQUENCY: Dimension = Dimension::TIME.inv();

/// Dimension: force (M L T^-2)
const DIM_FORCE: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH)
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));

/// Dimension: energy (M L^2 T^-2)
const DIM_ENERGY: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));

/// Dimension: power (M L^2 T^-3)
const DIM_POWER: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-3, 1)));

/// Dimension: pressure (M L^-1 T^-2)
const DIM_PRESSURE: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(-1, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));

/// Dimension: voltage (M L^2 T^-3 I^-1)
const DIM_VOLTAGE: Dimension = DIM_POWER
    .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1)));

/// Dimension: capacitance (I^2 T^4 M^-1 L^-2)
const DIM_CAPACITANCE: Dimension = Dimension::CURRENT.pow(Rational16::new(2, 1))
    .mul(&Dimension::TIME.pow(Rational16::new(4, 1)))
    .mul(&Dimension::MASS.pow(Rational16::new(-1, 1)))
    .mul(&Dimension::LENGTH.pow(Rational16::new(-2, 1)));

/// Dimension: resistance (M L^2 T^-3 I^-2)
const DIM_RESISTANCE: Dimension = DIM_POWER
    .mul(&Dimension::CURRENT.pow(Rational16::new(-2, 1)));

/// Dimension: conductance (I^2 T^3 M^-1 L^-2)
const DIM_CONDUCTANCE: Dimension = Dimension::CURRENT.pow(Rational16::new(2, 1))
    .mul(&Dimension::TIME.pow(Rational16::new(3, 1)))
    .mul(&Dimension::MASS.pow(Rational16::new(-1, 1)))
    .mul(&Dimension::LENGTH.pow(Rational16::new(-2, 1)));

/// Dimension: magnetic flux (M L^2 T^-2 I^-1)
const DIM_MAGNETIC_FLUX: Dimension = DIM_ENERGY
    .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1)));

/// Dimension: magnetic field (M T^-2 I^-1)
const DIM_MAGNETIC_FIELD: Dimension = Dimension::MASS
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)))
    .mul(&Dimension::CURRENT.pow(Rational16::new(-1, 1)));

/// Dimension: inductance (M L^2 T^-2 I^-2)
const DIM_INDUCTANCE: Dimension = DIM_ENERGY
    .mul(&Dimension::CURRENT.pow(Rational16::new(-2, 1)));

/// Dimension: luminous flux (J Ω)
const DIM_LUMINOUS_FLUX: Dimension = Dimension::LUMINOUS_INTENSITY
    .mul(&Dimension::SOLID_ANGLE);

/// Dimension: illuminance (J Ω L^-2)
const DIM_ILLUMINANCE: Dimension = DIM_LUMINOUS_FLUX
    .mul(&Dimension::LENGTH.pow(Rational16::new(-2, 1)));

/// Dimension: absorbed dose (L^2 T^-2)
const DIM_DOSE: Dimension = Dimension::LENGTH.pow(Rational16::new(2, 1))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));

/// Dimension: charge (I T)
const DIM_CHARGE: Dimension = Dimension::CURRENT.mul(&Dimension::TIME);

/// Hertz - frequency (1/s)
pub const HZ: BaseUnit = BaseUnit::new("hertz", "Hz", &[], DIM_FREQUENCY, 1.0);
/// Kilohertz (10^3 Hz)
pub const KHZ: BaseUnit = BaseUnit::new("kilohertz", "kHz", &[], DIM_FREQUENCY, 1e3);
/// Megahertz (10^6 Hz)
pub const MHZ: BaseUnit = BaseUnit::new("megahertz", "MHz", &[], DIM_FREQUENCY, 1e6);
/// Gigahertz (10^9 Hz)
pub const GHZ: BaseUnit = BaseUnit::new("gigahertz", "GHz", &[], DIM_FREQUENCY, 1e9);
/// Terahertz (10^12 Hz)
pub const THZ: BaseUnit = BaseUnit::new("terahertz", "THz", &[], DIM_FREQUENCY, 1e12);

/// Newton - force (kg m / s^2)
pub const N: BaseUnit = BaseUnit::new("newton", "N", &[], DIM_FORCE, 1.0);

/// Joule - energy (kg m^2 / s^2)
pub const J: BaseUnit = BaseUnit::new("joule", "J", &[], DIM_ENERGY, 1.0);

/// Electronvolt (1.602176634e-19 J)
pub const EV: BaseUnit = BaseUnit::new("electronvolt", "eV", &[], DIM_ENERGY, 1.602_176_634e-19);
/// Kiloelectronvolt (10^3 eV)
pub const KEV: BaseUnit = BaseUnit::new("kiloelectronvolt", "keV", &[], DIM_ENERGY, 1.602_176_634e-16);
/// Megaelectronvolt (10^6 eV)
pub const MEV: BaseUnit = BaseUnit::new("megaelectronvolt", "MeV", &[], DIM_ENERGY, 1.602_176_634e-13);
/// Gigaelectronvolt (10^9 eV)
pub const GEV: BaseUnit = BaseUnit::new("gigaelectronvolt", "GeV", &[], DIM_ENERGY, 1.602_176_634e-10);

/// Watt - power (kg m^2 / s^3)
pub const W: BaseUnit = BaseUnit::new("watt", "W", &[], DIM_POWER, 1.0);
/// Kilowatt (10^3 W)
pub const KW: BaseUnit = BaseUnit::new("kilowatt", "kW", &[], DIM_POWER, 1e3);
/// Megawatt (10^6 W)
pub const MW: BaseUnit = BaseUnit::new("megawatt", "MW", &[], DIM_POWER, 1e6);

/// Pascal - pressure (kg / m / s^2)
pub const PA: BaseUnit = BaseUnit::new("pascal", "Pa", &[], DIM_PRESSURE, 1.0);

/// Coulomb - electric charge (A s)
pub const C: BaseUnit = BaseUnit::new("coulomb", "C", &[], DIM_CHARGE, 1.0);

/// Volt - electric potential (kg m^2 / A / s^3)
pub const V: BaseUnit = BaseUnit::new("volt", "V", &[], DIM_VOLTAGE, 1.0);

/// Farad - capacitance (A^2 s^4 / kg / m^2)
pub const F: BaseUnit = BaseUnit::new("farad", "F", &[], DIM_CAPACITANCE, 1.0);

/// Ohm - electrical resistance (kg m^2 / A^2 / s^3)
pub const OHM: BaseUnit = BaseUnit::new("ohm", "Ohm", &["ohm"], DIM_RESISTANCE, 1.0);

/// Siemens - electrical conductance (A^2 s^3 / kg / m^2)
pub const SIEMENS: BaseUnit = BaseUnit::new("siemens", "S", &[], DIM_CONDUCTANCE, 1.0);

/// Weber - magnetic flux (kg m^2 / A / s^2)
pub const WB: BaseUnit = BaseUnit::new("weber", "Wb", &[], DIM_MAGNETIC_FLUX, 1.0);

/// Tesla - magnetic field (kg / A / s^2)
pub const T: BaseUnit = BaseUnit::new("tesla", "T", &[], DIM_MAGNETIC_FIELD, 1.0);

/// Henry - inductance (kg m^2 / A^2 / s^2)
pub const HENRY: BaseUnit = BaseUnit::new("henry", "H", &[], DIM_INDUCTANCE, 1.0);

/// Lumen - luminous flux (cd sr)
pub const LM: BaseUnit = BaseUnit::new("lumen", "lm", &[], DIM_LUMINOUS_FLUX, 1.0);

/// Lux - illuminance (cd sr / m^2)
pub const LX: BaseUnit = BaseUnit::new("lux", "lx", &[], DIM_ILLUMINANCE, 1.0);

/// Becquerel - radioactivity (1/s)
pub const BQ: BaseUnit = BaseUnit::new("becquerel", "Bq", &[], DIM_FREQUENCY, 1.0);

/// Gray - absorbed dose (m^2 / s^2)
pub const GY: BaseUnit = BaseUnit::new("gray", "Gy", &[], DIM_DOSE, 1.0);

/// Sievert - equivalent dose (m^2 / s^2)
pub const SV: BaseUnit = BaseUnit::new("sievert", "Sv", &[], DIM_DOSE, 1.0);

// =============================================================================
// Angle units
// =============================================================================

/// Degree (pi/180 rad)
pub const DEG: BaseUnit = BaseUnit::new(
    "degree", "deg", &["degree"], Dimension::ANGLE,
    std::f64::consts::PI / 180.0,
);

/// Arcminute (1/60 degree)
pub const ARCMIN: BaseUnit = BaseUnit::new(
    "arcminute", "arcmin", &["arcminute"], Dimension::ANGLE,
    std::f64::consts::PI / 180.0 / 60.0,
);

/// Arcsecond (1/3600 degree)
pub const ARCSEC: BaseUnit = BaseUnit::new(
    "arcsecond", "arcsec", &["arcsecond"], Dimension::ANGLE,
    std::f64::consts::PI / 180.0 / 3600.0,
);

/// Milliarcsecond (10^-3 arcsec)
pub const MAS: BaseUnit = BaseUnit::new(
    "milliarcsecond", "mas", &[], Dimension::ANGLE,
    std::f64::consts::PI / 180.0 / 3600.0 / 1000.0,
);

/// Microarcsecond (10^-6 arcsec)
pub const UAS: BaseUnit = BaseUnit::new(
    "microarcsecond", "uas", &[], Dimension::ANGLE,
    std::f64::consts::PI / 180.0 / 3600.0 / 1e6,
);

/// Hour angle
pub const HOURANGLE: BaseUnit = BaseUnit::new(
    "hourangle", "hourangle", &[], Dimension::ANGLE,
    std::f64::consts::PI / 12.0,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_km_to_m() {
        let q = 1.0 * KM;
        let q_m = q.to(M).unwrap();
        assert!((q_m.value() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_hour_to_second() {
        let q = 1.0 * H;
        let q_s = q.to(S).unwrap();
        assert!((q_s.value() - 3600.0).abs() < 1e-10);
    }

    #[test]
    fn test_degree_to_radian() {
        let q = 180.0 * DEG;
        let q_rad = q.to(RAD).unwrap();
        assert!((q_rad.value() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_energy_unit() {
        // 1 J = 1 kg m^2 / s^2
        let energy = 1.0 * J;
        let dim = energy.unit().dimension();
        assert_eq!(dim.mass, Rational16::ONE);
        assert_eq!(dim.length, Rational16::new(2, 1));
        assert_eq!(dim.time, Rational16::new(-2, 1));
    }
}
