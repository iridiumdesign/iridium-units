//! Imperial/US customary units.
//!
//! This module provides common Imperial and US customary units.

use crate::dimension::{Dimension, Rational8};
use crate::unit::base::BaseUnit;
use crate::unit::Unit;
use lazy_static::lazy_static;

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

lazy_static! {
    // =============================================================================
    // Length Units
    // =============================================================================

    /// Inch (2.54 cm exactly)
    pub static ref INCH: Unit = Unit::Base(BaseUnit::new(
        "inch", "in", &["inch", "inches"],
        Dimension::LENGTH,
        INCH_M
    ));

    /// Foot (12 inches)
    pub static ref FOOT: Unit = Unit::Base(BaseUnit::new(
        "foot", "ft", &["foot", "feet"],
        Dimension::LENGTH,
        FOOT_M
    ));

    /// Yard (3 feet)
    pub static ref YARD: Unit = Unit::Base(BaseUnit::new(
        "yard", "yd", &["yard", "yards"],
        Dimension::LENGTH,
        YARD_M
    ));

    /// Mile (5280 feet)
    pub static ref MILE: Unit = Unit::Base(BaseUnit::new(
        "mile", "mi", &["mile", "miles"],
        Dimension::LENGTH,
        MILE_M
    ));

    /// Nautical mile (1852 m exactly)
    pub static ref NAUTICAL_MILE: Unit = Unit::Base(BaseUnit::new(
        "nautical_mile", "nmi", &["NM"],
        Dimension::LENGTH,
        NAUTICAL_MILE_M
    ));

    /// Fathom (6 feet)
    pub static ref FATHOM: Unit = Unit::Base(BaseUnit::new(
        "fathom", "fathom", &["fathoms"],
        Dimension::LENGTH,
        6.0 * FOOT_M
    ));

    /// Furlong (660 feet)
    pub static ref FURLONG: Unit = Unit::Base(BaseUnit::new(
        "furlong", "fur", &["furlong", "furlongs"],
        Dimension::LENGTH,
        660.0 * FOOT_M
    ));

    /// Thou / mil (0.001 inch)
    pub static ref THOU: Unit = Unit::Base(BaseUnit::new(
        "thou", "thou", &["mil"],
        Dimension::LENGTH,
        INCH_M / 1000.0
    ));

    // =============================================================================
    // Mass Units
    // =============================================================================

    /// Pound (avoirdupois)
    pub static ref POUND: Unit = Unit::Base(BaseUnit::new(
        "pound", "lb", &["lbm", "pound", "pounds"],
        Dimension::MASS,
        POUND_KG
    ));

    /// Ounce (avoirdupois)
    pub static ref OUNCE: Unit = Unit::Base(BaseUnit::new(
        "ounce", "oz", &["ounce", "ounces"],
        Dimension::MASS,
        OUNCE_KG
    ));

    /// Short ton (2000 lb)
    pub static ref TON: Unit = Unit::Base(BaseUnit::new(
        "ton", "ton", &["short_ton"],
        Dimension::MASS,
        TON_KG
    ));

    /// Long ton (2240 lb)
    pub static ref LONG_TON: Unit = Unit::Base(BaseUnit::new(
        "long_ton", "long_ton", &["imperial_ton"],
        Dimension::MASS,
        LONG_TON_KG
    ));

    /// Grain (1/7000 lb)
    pub static ref GRAIN: Unit = Unit::Base(BaseUnit::new(
        "grain", "gr", &["grain", "grains"],
        Dimension::MASS,
        POUND_KG / 7000.0
    ));

    /// Stone (14 lb)
    pub static ref STONE: Unit = Unit::Base(BaseUnit::new(
        "stone", "st", &["stone"],
        Dimension::MASS,
        14.0 * POUND_KG
    ));

    // =============================================================================
    // Volume Units (US)
    // =============================================================================

    /// US gallon
    pub static ref GALLON: Unit = Unit::Base(BaseUnit::new(
        "gallon", "gal", &["gallon", "gallons"],
        Dimension::LENGTH.pow(Rational8::new(3, 1)),
        GALLON_US_M3
    ));

    /// Imperial gallon
    pub static ref IMPERIAL_GALLON: Unit = Unit::Base(BaseUnit::new(
        "imperial_gallon", "imp_gal", &[],
        Dimension::LENGTH.pow(Rational8::new(3, 1)),
        GALLON_IMP_M3
    ));

    /// US fluid ounce
    pub static ref FLUID_OUNCE: Unit = Unit::Base(BaseUnit::new(
        "fluid_ounce", "fl_oz", &["floz"],
        Dimension::LENGTH.pow(Rational8::new(3, 1)),
        FLUID_OUNCE_US_M3
    ));

    /// US pint
    pub static ref PINT: Unit = Unit::Base(BaseUnit::new(
        "pint", "pt", &["pint", "pints"],
        Dimension::LENGTH.pow(Rational8::new(3, 1)),
        PINT_US_M3
    ));

    /// US quart
    pub static ref QUART: Unit = Unit::Base(BaseUnit::new(
        "quart", "qt", &["quart", "quarts"],
        Dimension::LENGTH.pow(Rational8::new(3, 1)),
        QUART_US_M3
    ));

    /// Cubic inch
    pub static ref CUBIC_INCH: Unit = Unit::Base(BaseUnit::new(
        "cubic_inch", "in^3", &["cu_in"],
        Dimension::LENGTH.pow(Rational8::new(3, 1)),
        INCH_M * INCH_M * INCH_M
    ));

    /// Cubic foot
    pub static ref CUBIC_FOOT: Unit = Unit::Base(BaseUnit::new(
        "cubic_foot", "ft^3", &["cu_ft"],
        Dimension::LENGTH.pow(Rational8::new(3, 1)),
        FOOT_M * FOOT_M * FOOT_M
    ));

    // =============================================================================
    // Force and Pressure
    // =============================================================================

    /// Pound-force
    pub static ref POUND_FORCE: Unit = Unit::Base(BaseUnit::new(
        "pound_force", "lbf", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH)
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        POUND_KG * 9.80665  // Using standard gravity
    ));

    /// Pounds per square inch
    pub static ref PSI: Unit = Unit::Base(BaseUnit::new(
        "psi", "psi", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational8::new(-1, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        POUND_KG * 9.80665 / (INCH_M * INCH_M)
    ));

    // =============================================================================
    // Temperature (offsets handled in equivalencies)
    // =============================================================================

    /// Rankine (absolute temperature in Fahrenheit scale)
    /// ΔR = ΔK * 9/5
    pub static ref RANKINE: Unit = Unit::Base(BaseUnit::new(
        "rankine", "R", &["degR"],
        Dimension::TEMPERATURE,
        5.0 / 9.0  // 1 R = 5/9 K
    ));

    // =============================================================================
    // Energy
    // =============================================================================

    /// British thermal unit (IT)
    pub static ref BTU: Unit = Unit::Base(BaseUnit::new(
        "btu", "BTU", &["Btu"],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        1055.05585  // J
    ));

    /// Therm (100,000 BTU)
    pub static ref THERM: Unit = Unit::Base(BaseUnit::new(
        "therm", "therm", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        1.055e8  // J
    ));

    /// Foot-pound
    pub static ref FOOT_POUND: Unit = Unit::Base(BaseUnit::new(
        "foot_pound", "ft_lbf", &["ft_lb"],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-2, 1))),
        1.355818  // J
    ));

    // =============================================================================
    // Power
    // =============================================================================

    /// Horsepower (mechanical)
    pub static ref HORSEPOWER: Unit = Unit::Base(BaseUnit::new(
        "horsepower", "hp", &[],
        Dimension::MASS
            .mul(&Dimension::LENGTH.pow(Rational8::new(2, 1)))
            .mul(&Dimension::TIME.pow(Rational8::new(-3, 1))),
        745.69987  // W
    ));

    // =============================================================================
    // Speed
    // =============================================================================

    /// Miles per hour
    pub static ref MPH: Unit = Unit::Base(BaseUnit::new(
        "mph", "mph", &["mi/h"],
        Dimension::LENGTH.mul(&Dimension::TIME.pow(Rational8::new(-1, 1))),
        MILE_M / 3600.0
    ));

    /// Knot (nautical mile per hour)
    pub static ref KNOT: Unit = Unit::Base(BaseUnit::new(
        "knot", "kn", &["kt", "knot", "knots"],
        Dimension::LENGTH.mul(&Dimension::TIME.pow(Rational8::new(-1, 1))),
        NAUTICAL_MILE_M / 3600.0
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{M, KG, CM};

    #[test]
    fn test_foot_to_meters() {
        let q = 1.0 * FOOT.clone();
        let q_m = q.to(&M).unwrap();
        assert!((q_m.value() - 0.3048).abs() < 1e-10);
    }

    #[test]
    fn test_inch_to_cm() {
        let q = 1.0 * INCH.clone();
        let q_cm = q.to(&CM).unwrap();
        assert!((q_cm.value() - 2.54).abs() < 1e-10);
    }

    #[test]
    fn test_pound_to_kg() {
        let q = 1.0 * POUND.clone();
        let q_kg = q.to(&KG).unwrap();
        assert!((q_kg.value() - 0.45359237).abs() < 1e-10);
    }

    #[test]
    fn test_mile_to_km() {
        let q = 1.0 * MILE.clone();
        let q_m = q.to(&M).unwrap();
        assert!((q_m.value() - 1609.344).abs() < 1e-10);
    }
}
