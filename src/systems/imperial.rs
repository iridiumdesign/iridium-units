//! Imperial/US customary units.
//!
//! This module provides common Imperial and US customary units.
//!
//! # Examples
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! // Marathon distance
//! let marathon = 42.195 * KM;
//! let in_miles = marathon.to(MILE).unwrap();
//! assert!((in_miles.value() - 26.219).abs() < 0.001);
//!
//! // Weight conversion
//! let weight = 150.0 * POUND;
//! let in_kg = weight.to(KG).unwrap();
//! assert!((in_kg.value() - 68.04).abs() < 0.01);
//! ```

use crate::dimension::{Dimension, Rational16};
use crate::unit::base::BaseUnit;

// Conversion factors
const INCH_M: f64 = 0.0254; // m (exact)
const FOOT_M: f64 = 0.3048; // m (exact)
const YARD_M: f64 = 0.9144; // m (exact)
const MILE_M: f64 = 1609.344; // m (exact)
const NAUTICAL_MILE_M: f64 = 1852.0; // m (exact)

const POUND_KG: f64 = 0.45359237; // kg (exact)
const OUNCE_KG: f64 = 0.028349523125; // kg (exact)
const TON_KG: f64 = 907.18474; // kg (short ton)
const LONG_TON_KG: f64 = 1016.0469088; // kg (long ton)

// Volume
const GALLON_US_M3: f64 = 0.003785411784; // m^3 (exact)
const GALLON_IMP_M3: f64 = 0.00454609; // m^3 (exact)
const FLUID_OUNCE_US_M3: f64 = 2.9573529562e-5; // m^3
const PINT_US_M3: f64 = 4.73176473e-4; // m^3
const QUART_US_M3: f64 = 9.46352946e-4; // m^3

// Shared dimension constants
const DIM_VOLUME: Dimension = Dimension::LENGTH.pow(Rational16::new(3, 1));
const DIM_FORCE: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH)
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
const DIM_PRESSURE: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(-1, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
const DIM_ENERGY: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));
const DIM_POWER: Dimension = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-3, 1)));
const DIM_SPEED: Dimension = Dimension::LENGTH
    .mul(&Dimension::TIME.pow(Rational16::new(-1, 1)));

// =============================================================================
// Length Units
// =============================================================================

/// Inch (2.54 cm exactly)
pub const INCH: BaseUnit = BaseUnit::new(
    "inch", "in", &["inch", "inches"], Dimension::LENGTH, INCH_M,
);

/// Foot (12 inches)
pub const FOOT: BaseUnit = BaseUnit::new(
    "foot", "ft", &["foot", "feet"], Dimension::LENGTH, FOOT_M,
);

/// Yard (3 feet)
pub const YARD: BaseUnit = BaseUnit::new(
    "yard", "yd", &["yard", "yards"], Dimension::LENGTH, YARD_M,
);

/// Mile (5280 feet)
pub const MILE: BaseUnit = BaseUnit::new(
    "mile", "mi", &["mile", "miles"], Dimension::LENGTH, MILE_M,
);

/// Nautical mile (1852 m exactly)
pub const NAUTICAL_MILE: BaseUnit = BaseUnit::new(
    "nautical_mile", "nmi", &["NM"], Dimension::LENGTH, NAUTICAL_MILE_M,
);

/// Fathom (6 feet)
pub const FATHOM: BaseUnit = BaseUnit::new(
    "fathom", "fathom", &["fathoms"], Dimension::LENGTH, 6.0 * FOOT_M,
);

/// Furlong (660 feet)
pub const FURLONG: BaseUnit = BaseUnit::new(
    "furlong", "fur", &["furlong", "furlongs"], Dimension::LENGTH, 660.0 * FOOT_M,
);

/// Thou / mil (0.001 inch)
pub const THOU: BaseUnit = BaseUnit::new(
    "thou", "thou", &["mil"], Dimension::LENGTH, INCH_M / 1000.0,
);

// =============================================================================
// Mass Units
// =============================================================================

/// Pound (avoirdupois)
pub const POUND: BaseUnit = BaseUnit::new(
    "pound", "lb", &["lbm", "pound", "pounds"], Dimension::MASS, POUND_KG,
);

/// Ounce (avoirdupois)
pub const OUNCE: BaseUnit = BaseUnit::new(
    "ounce", "oz", &["ounce", "ounces"], Dimension::MASS, OUNCE_KG,
);

/// Short ton (2000 lb)
pub const TON: BaseUnit = BaseUnit::new(
    "ton", "ton", &["short_ton"], Dimension::MASS, TON_KG,
);

/// Long ton (2240 lb)
pub const LONG_TON: BaseUnit = BaseUnit::new(
    "long_ton", "long_ton", &["imperial_ton"], Dimension::MASS, LONG_TON_KG,
);

/// Grain (1/7000 lb)
pub const GRAIN: BaseUnit = BaseUnit::new(
    "grain", "gr", &["grain", "grains"], Dimension::MASS, POUND_KG / 7000.0,
);

/// Stone (14 lb)
pub const STONE: BaseUnit = BaseUnit::new(
    "stone", "st", &["stone"], Dimension::MASS, 14.0 * POUND_KG,
);

// =============================================================================
// Volume Units (US)
// =============================================================================

/// US gallon
pub const GALLON: BaseUnit = BaseUnit::new(
    "gallon", "gal", &["gallon", "gallons"], DIM_VOLUME, GALLON_US_M3,
);

/// Imperial gallon
pub const IMPERIAL_GALLON: BaseUnit = BaseUnit::new(
    "imperial_gallon", "imp_gal", &[], DIM_VOLUME, GALLON_IMP_M3,
);

/// US fluid ounce
pub const FLUID_OUNCE: BaseUnit = BaseUnit::new(
    "fluid_ounce", "fl_oz", &["floz"], DIM_VOLUME, FLUID_OUNCE_US_M3,
);

/// US pint
pub const PINT: BaseUnit = BaseUnit::new(
    "pint", "pt", &["pint", "pints"], DIM_VOLUME, PINT_US_M3,
);

/// US quart
pub const QUART: BaseUnit = BaseUnit::new(
    "quart", "qt", &["quart", "quarts"], DIM_VOLUME, QUART_US_M3,
);

/// Cubic inch
pub const CUBIC_INCH: BaseUnit = BaseUnit::new(
    "cubic_inch", "in^3", &["cu_in"], DIM_VOLUME, INCH_M * INCH_M * INCH_M,
);

/// Cubic foot
pub const CUBIC_FOOT: BaseUnit = BaseUnit::new(
    "cubic_foot", "ft^3", &["cu_ft"], DIM_VOLUME, FOOT_M * FOOT_M * FOOT_M,
);

// =============================================================================
// Force and Pressure
// =============================================================================

/// Pound-force
pub const POUND_FORCE: BaseUnit = BaseUnit::new(
    "pound_force", "lbf", &[], DIM_FORCE, POUND_KG * 9.80665,
);

/// Pounds per square inch
pub const PSI: BaseUnit = BaseUnit::new(
    "psi", "psi", &[], DIM_PRESSURE, POUND_KG * 9.80665 / (INCH_M * INCH_M),
);

// =============================================================================
// Temperature (offsets handled in equivalencies)
// =============================================================================

/// Rankine (absolute temperature in Fahrenheit scale)
/// ΔR = ΔK * 9/5
pub const RANKINE: BaseUnit = BaseUnit::new(
    "rankine", "R", &["degR"], Dimension::TEMPERATURE, 5.0 / 9.0,
);

// =============================================================================
// Energy
// =============================================================================

/// British thermal unit (IT)
pub const BTU: BaseUnit = BaseUnit::new(
    "btu", "BTU", &["Btu"], DIM_ENERGY, 1055.05585,
);

/// Therm (100,000 BTU)
pub const THERM: BaseUnit = BaseUnit::new(
    "therm", "therm", &[], DIM_ENERGY, 1.055e8,
);

/// Foot-pound
pub const FOOT_POUND: BaseUnit = BaseUnit::new(
    "foot_pound", "ft_lbf", &["ft_lb"], DIM_ENERGY, 1.355818,
);

// =============================================================================
// Power
// =============================================================================

/// Horsepower (mechanical)
pub const HORSEPOWER: BaseUnit = BaseUnit::new(
    "horsepower", "hp", &[], DIM_POWER, 745.69987,
);

// =============================================================================
// Speed
// =============================================================================

/// Miles per hour
pub const MPH: BaseUnit = BaseUnit::new(
    "mph", "mph", &["mi/h"], DIM_SPEED, MILE_M / 3600.0,
);

/// Knot (nautical mile per hour)
pub const KNOT: BaseUnit = BaseUnit::new(
    "knot", "kn", &["kt", "knot", "knots"], DIM_SPEED, NAUTICAL_MILE_M / 3600.0,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{M, KG, CM};

    #[test]
    fn test_foot_to_meters() {
        let q = 1.0 * FOOT;
        let q_m = q.to(M).unwrap();
        assert!((q_m.value() - 0.3048).abs() < 1e-10);
    }

    #[test]
    fn test_inch_to_cm() {
        let q = 1.0 * INCH;
        let q_cm = q.to(CM).unwrap();
        assert!((q_cm.value() - 2.54).abs() < 1e-10);
    }

    #[test]
    fn test_pound_to_kg() {
        let q = 1.0 * POUND;
        let q_kg = q.to(KG).unwrap();
        assert!((q_kg.value() - 0.45359237).abs() < 1e-10);
    }

    #[test]
    fn test_mile_to_km() {
        let q = 1.0 * MILE;
        let q_m = q.to(M).unwrap();
        assert!((q_m.value() - 1609.344).abs() < 1e-10);
    }
}
