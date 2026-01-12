//! String parsing for units and quantities.
//!
//! This module provides parsing of unit strings like "m", "km/s", "m/s^2"
//! and quantity strings like "5.0 km" or "100 m/s".
//!
//! # Examples
//!
//! ```
//! use iridium_units::prelude::*;
//! use std::str::FromStr;
//!
//! // Parse a simple unit
//! let meter = Unit::from_str("m").unwrap();
//! let kilometer = Unit::from_str("km").unwrap();
//!
//! // Parse composite units
//! let velocity = Unit::from_str("m/s").unwrap();
//! let acceleration = Unit::from_str("m/s^2").unwrap();
//!
//! // Parse quantities
//! let distance = Quantity::from_str("100 km").unwrap();
//! let speed = Quantity::from_str("9.8 m/s^2").unwrap();
//! ```
//!
//! # Supported Syntax
//!
//! ## Unit Strings
//!
//! - Simple units: `"m"`, `"meter"`, `"meters"`, `"km"`, `"kg"`
//! - Powers: `"m^2"`, `"s^-1"`, `"m^3"`
//! - Multiplication: `"kg m"`, `"kg*m"`, `"kg.m"`
//! - Division: `"m/s"`, `"kg/m^3"`
//! - Combined: `"kg m^2 / s^2"`, `"m/s^2"`
//!
//! ## Quantity Strings
//!
//! - `"5.0 km"`, `"100 m/s"`, `"-3.14 rad"`
//! - Scientific notation: `"1.5e8 m"`, `"6.67e-11 m^3/kg/s^2"`

use crate::dimension::Rational8;
use crate::error::{UnitError, UnitResult};
use crate::quantity::Quantity;
use crate::unit::Unit;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::RwLock;

/// A registry entry for a unit.
#[derive(Clone)]
struct UnitEntry {
    /// Function to get the unit (since lazy_static units need dereferencing)
    unit: Unit,
}

lazy_static! {
    /// Global unit registry mapping strings to units.
    static ref UNIT_REGISTRY: RwLock<HashMap<String, UnitEntry>> = {
        let mut map = HashMap::new();
        register_builtin_units(&mut map);
        RwLock::new(map)
    };
}

/// Register all built-in units with the registry.
fn register_builtin_units(map: &mut HashMap<String, UnitEntry>) {
    use crate::systems::si::*;
    use crate::systems::astrophysical::{
        AU, PARSEC, KPC, MPC, GPC, LIGHT_YEAR, LIGHT_SECOND,
        SOLAR_MASS, SOLAR_RADIUS, SOLAR_LUMINOSITY,
        JUPITER_MASS, JUPITER_RADIUS, EARTH_MASS, EARTH_RADIUS,
        ANGSTROM, JANSKY, MJY, UJY, BARN, MBARN, UBARN,
        ERG, DYN, GAUSS, PHOTON, COUNT, ELECTRON,
        SIDEREAL_DAY, TROPICAL_YEAR, SIDEREAL_YEAR,
    };
    use crate::systems::imperial::*;
    use crate::systems::cgs::{CENTIMETER, GRAM, DYNE as DYNE_CGS, ERG as ERG_CGS};

    // Helper to register a unit with multiple names
    macro_rules! register {
        ($map:expr, $unit:expr, $($name:expr),+) => {
            let entry = UnitEntry { unit: $unit.clone() };
            $(
                $map.insert($name.to_lowercase(), entry.clone());
            )+
        };
    }

    // SI Base Units
    register!(map, *M, "m", "meter", "meters", "metre", "metres");
    register!(map, *S, "s", "sec", "second", "seconds");
    register!(map, *KG, "kg", "kilogram", "kilograms");
    register!(map, *A, "a", "amp", "ampere", "amperes");
    register!(map, *K, "k", "kelvin");
    register!(map, *MOL, "mol", "mole", "moles");
    register!(map, *CD, "cd", "candela");
    register!(map, *RAD, "rad", "radian", "radians");
    register!(map, *SR, "sr", "steradian", "steradians");

    // SI Length
    register!(map, *KM, "km", "kilometer", "kilometers", "kilometre", "kilometres");
    register!(map, *CM, "cm", "centimeter", "centimeters", "centimetre", "centimetres");
    register!(map, *MM, "mm", "millimeter", "millimeters", "millimetre", "millimetres");
    register!(map, *UM, "um", "micrometer", "micrometers", "micron", "microns");
    register!(map, *NM, "nm", "nanometer", "nanometers");
    register!(map, *PM, "pm", "picometer", "picometers");
    register!(map, *FM, "fm", "femtometer", "femtometers");

    // SI Time
    register!(map, *MS, "ms", "millisecond", "milliseconds");
    register!(map, *US, "us", "microsecond", "microseconds");
    register!(map, *NS, "ns", "nanosecond", "nanoseconds");
    register!(map, *PS, "ps", "picosecond", "picoseconds");
    register!(map, *MIN, "min", "minute", "minutes");
    register!(map, *H, "h", "hr", "hour", "hours");
    register!(map, *DAY, "d", "day", "days");
    register!(map, *YR, "yr", "year", "years", "julian_year");

    // SI Mass
    register!(map, *G, "g", "gram", "grams");
    register!(map, *MG, "mg", "milligram", "milligrams");
    register!(map, *UG, "ug", "microgram", "micrograms");
    register!(map, *TONNE, "t", "tonne", "tonnes", "metric_ton");

    // SI Derived - Frequency
    register!(map, *HZ, "hz", "hertz");
    register!(map, *KHZ, "khz", "kilohertz");
    register!(map, *MHZ, "mhz", "megahertz");
    register!(map, *GHZ, "ghz", "gigahertz");
    register!(map, *THZ, "thz", "terahertz");

    // SI Derived - Mechanics
    register!(map, *N, "n", "newton", "newtons");
    register!(map, *J, "j", "joule", "joules");
    register!(map, *W, "w", "watt", "watts");
    register!(map, *KW, "kw", "kilowatt", "kilowatts");
    register!(map, *MW, "mw", "megawatt", "megawatts");
    register!(map, *PA, "pa", "pascal", "pascals");

    // SI Derived - Electrical
    register!(map, *C, "c", "coulomb", "coulombs");
    register!(map, *V, "v", "volt", "volts");
    register!(map, *F, "f", "farad", "farads");
    register!(map, *OHM, "ohm", "ohms");

    // SI Derived - Energy
    register!(map, *EV, "ev", "electronvolt", "electronvolts");
    register!(map, *KEV, "kev", "kiloelectronvolt");
    register!(map, *MEV, "mev", "megaelectronvolt");
    register!(map, *GEV, "gev", "gigaelectronvolt");

    // SI Angles
    register!(map, *DEG, "deg", "degree", "degrees");
    register!(map, *ARCMIN, "arcmin", "arcminute", "arcminutes");
    register!(map, *ARCSEC, "arcsec", "arcsecond", "arcseconds");
    register!(map, *MAS, "mas", "milliarcsecond", "milliarcseconds");
    register!(map, *UAS, "uas", "microarcsecond", "microarcseconds");

    // Astrophysical - Distance
    register!(map, *AU, "au", "astronomical_unit");
    register!(map, *PARSEC, "pc", "parsec", "parsecs");
    register!(map, *KPC, "kpc", "kiloparsec", "kiloparsecs");
    register!(map, *MPC, "mpc", "megaparsec", "megaparsecs");
    register!(map, *GPC, "gpc", "gigaparsec", "gigaparsecs");
    register!(map, *LIGHT_YEAR, "ly", "lyr", "lightyear", "lightyears", "light_year", "light_years");

    // Astrophysical - Solar
    register!(map, *SOLAR_MASS, "m_sun", "msun", "solmass", "solar_mass");
    register!(map, *SOLAR_RADIUS, "r_sun", "rsun", "solrad", "solar_radius");
    register!(map, *SOLAR_LUMINOSITY, "l_sun", "lsun", "sollum", "solar_luminosity");

    // Astrophysical - Planetary
    register!(map, *JUPITER_MASS, "m_jup", "mjup", "jupiter_mass");
    register!(map, *JUPITER_RADIUS, "r_jup", "rjup", "jupiter_radius");
    register!(map, *EARTH_MASS, "m_earth", "mearth", "earth_mass");
    register!(map, *EARTH_RADIUS, "r_earth", "rearth", "earth_radius");

    // Astrophysical - Spectroscopic
    register!(map, *ANGSTROM, "angstrom", "aa");
    register!(map, *JANSKY, "jy", "jansky");
    register!(map, *MJY, "mjy", "millijansky");
    register!(map, *UJY, "ujy", "microjansky");
    register!(map, *BARN, "barn", "barns");

    // Astrophysical - CGS commonly used
    register!(map, *ERG, "erg", "ergs");
    register!(map, *DYN, "dyn", "dyne", "dynes");
    register!(map, *GAUSS, "gauss");

    // Imperial - Length
    register!(map, *INCH, "in", "inch", "inches");
    register!(map, *FOOT, "ft", "foot", "feet");
    register!(map, *YARD, "yd", "yard", "yards");
    register!(map, *MILE, "mi", "mile", "miles");
    register!(map, *NAUTICAL_MILE, "nmi", "nautical_mile");

    // Imperial - Mass
    register!(map, *POUND, "lb", "lbm", "pound", "pounds");
    register!(map, *OUNCE, "oz", "ounce", "ounces");
    register!(map, *TON, "ton", "tons", "short_ton");

    // Imperial - Volume
    register!(map, *GALLON, "gal", "gallon", "gallons");
    register!(map, *PINT, "pt", "pint", "pints");
    register!(map, *QUART, "qt", "quart", "quarts");

    // Imperial - Other
    register!(map, *PSI, "psi");
    register!(map, *MPH, "mph");
    register!(map, *KNOT, "kn", "kt", "knot", "knots");
    register!(map, *HORSEPOWER, "hp", "horsepower");
    register!(map, *BTU, "btu");

    // CGS
    register!(map, *CENTIMETER, "centimeter_cgs");
    register!(map, *GRAM, "gram_cgs");
}

/// Look up a simple unit by name.
pub fn lookup_unit(name: &str) -> Option<Unit> {
    let registry = UNIT_REGISTRY.read().ok()?;
    registry.get(&name.to_lowercase()).map(|e| e.unit.clone())
}

/// Register a custom unit with the registry.
///
/// This allows adding user-defined units that can be parsed from strings.
pub fn register_unit(names: &[&str], unit: Unit) {
    if let Ok(mut registry) = UNIT_REGISTRY.write() {
        let entry = UnitEntry { unit };
        for name in names {
            registry.insert(name.to_lowercase(), entry.clone());
        }
    }
}

/// Parse a unit string into a Unit.
///
/// Supports:
/// - Simple units: "m", "kg", "s"
/// - Powers: "m^2", "s^-1", "m^1/2"
/// - Multiplication: "kg m", "kg*m"
/// - Division: "m/s", "kg/m^3"
/// - Combined: "kg m^2 / s^2"
pub fn parse_unit(s: &str) -> UnitResult<Unit> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(Unit::dimensionless());
    }

    // Split by division, but be careful not to split inside exponents
    // Exponents like "m^1/2" should not be split at the "/"
    let parts = split_unit_by_division(s);

    match parts.len() {
        1 => parse_unit_product(&parts[0]),
        2 => {
            let numerator = parse_unit_product(&parts[0])?;
            let denominator = parse_unit_product(&parts[1])?;
            Ok(&numerator / &denominator)
        }
        _ => {
            // Multiple divisions: a/b/c = a / (b * c)
            let numerator = parse_unit_product(&parts[0])?;
            let mut denominator = parse_unit_product(&parts[1])?;
            for part in &parts[2..] {
                let next = parse_unit_product(part)?;
                denominator = &denominator * &next;
            }
            Ok(&numerator / &denominator)
        }
    }
}

/// Split a unit string by division, respecting exponent notation.
///
/// This handles cases like "m^1/2" where the "/" is part of the exponent,
/// not a division between units.
fn split_unit_by_division(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_exponent = false;

    for c in s.chars() {
        if c == '^' {
            in_exponent = true;
            current.push(c);
        } else if c == '/' && !in_exponent {
            // This is a unit division
            parts.push(current.trim().to_string());
            current = String::new();
        } else if c.is_whitespace() && in_exponent {
            // Exponent ends at whitespace
            in_exponent = false;
            current.push(c);
        } else if !c.is_ascii_digit() && c != '-' && c != '+' && c != '/' && in_exponent {
            // Exponent ends at non-numeric character (except / for fractions)
            in_exponent = false;
            current.push(c);
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    if parts.is_empty() {
        parts.push(String::new());
    }

    parts
}

/// Parse a product of units (e.g., "kg m^2")
fn parse_unit_product(s: &str) -> UnitResult<Unit> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(Unit::dimensionless());
    }

    // Split by whitespace, *, or . (multiplication separators)
    let tokens: Vec<&str> = s
        .split(|c: char| c.is_whitespace() || c == '*' || c == '.')
        .filter(|t| !t.is_empty())
        .collect();

    if tokens.is_empty() {
        return Ok(Unit::dimensionless());
    }

    let mut result = parse_unit_with_power(tokens[0])?;
    for token in &tokens[1..] {
        let next = parse_unit_with_power(token)?;
        result = &result * &next;
    }

    Ok(result)
}

/// Parse a single unit with optional power (e.g., "m^2", "s^-1")
fn parse_unit_with_power(s: &str) -> UnitResult<Unit> {
    let s = s.trim();

    // Check for power notation
    if let Some(idx) = s.find('^') {
        let (name, power_str) = s.split_at(idx);
        let power_str = &power_str[1..]; // Skip the '^'

        let power = parse_power(power_str)?;
        let base_unit = lookup_simple_unit(name)?;
        Ok(base_unit.pow(power))
    } else if let Some(idx) = s.find("**") {
        // Python-style power notation
        let (name, power_str) = s.split_at(idx);
        let power_str = &power_str[2..]; // Skip the '**'

        let power = parse_power(power_str)?;
        let base_unit = lookup_simple_unit(name)?;
        Ok(base_unit.pow(power))
    } else {
        lookup_simple_unit(s)
    }
}

/// Parse a power exponent (integer or fraction)
fn parse_power(s: &str) -> UnitResult<Rational8> {
    let s = s.trim();

    // Check for fraction notation (e.g., "1/2")
    if let Some(idx) = s.find('/') {
        let (num_str, den_str) = s.split_at(idx);
        let den_str = &den_str[1..];

        let num: i8 = num_str.trim().parse().map_err(|_| {
            UnitError::ParseError(format!("invalid power numerator: {}", num_str))
        })?;
        let den: i8 = den_str.trim().parse().map_err(|_| {
            UnitError::ParseError(format!("invalid power denominator: {}", den_str))
        })?;

        if den == 0 {
            return Err(UnitError::ParseError("power denominator cannot be zero".into()));
        }

        Ok(Rational8::new(num, den))
    } else {
        // Simple integer power
        let exp: i8 = s.parse().map_err(|_| {
            UnitError::ParseError(format!("invalid power: {}", s))
        })?;
        Ok(Rational8::new(exp, 1))
    }
}

/// Look up a simple unit name (no powers or operations)
fn lookup_simple_unit(name: &str) -> UnitResult<Unit> {
    let name = name.trim();
    lookup_unit(name).ok_or_else(|| UnitError::UnknownUnit(name.to_string()))
}

/// Parse a quantity string (value + unit).
///
/// Format: `<value> <unit>`
///
/// Examples:
/// - "5.0 km"
/// - "100 m/s"
/// - "9.8 m/s^2"
/// - "1.5e8 m"
/// - "-3.14 rad"
pub fn parse_quantity(s: &str) -> UnitResult<Quantity> {
    let s = s.trim();

    // Find where the number ends and the unit begins
    // Numbers can contain: digits, '.', 'e', 'E', '+', '-'
    let mut unit_start = 0;
    let mut in_exponent = false;

    for (i, c) in s.char_indices() {
        if c == 'e' || c == 'E' {
            in_exponent = true;
            continue;
        }

        if in_exponent && (c == '+' || c == '-') {
            in_exponent = false;
            continue;
        }

        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' {
            continue;
        }

        // Found a non-number character
        if c.is_whitespace() {
            unit_start = i;
            break;
        } else {
            // Unit starts immediately after number (e.g., "5km")
            unit_start = i;
            break;
        }
    }

    if unit_start == 0 {
        // No unit found, try parsing whole string as number
        return Err(UnitError::ParseError(format!(
            "cannot parse quantity: no unit found in '{}'", s
        )));
    }

    let (value_str, unit_str) = s.split_at(unit_start);
    let value_str = value_str.trim();
    let unit_str = unit_str.trim();

    let value: f64 = value_str.parse().map_err(|_| {
        UnitError::ParseError(format!("invalid number: '{}'", value_str))
    })?;

    let unit = parse_unit(unit_str)?;

    Ok(Quantity::new(value, unit))
}

// Implement FromStr for Unit
impl FromStr for Unit {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_unit(s)
    }
}

// Implement FromStr for Quantity
impl FromStr for Quantity {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_quantity(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::si::{M, S, KG, KM, H};

    #[test]
    fn test_lookup_simple_unit() {
        let m = lookup_unit("m").unwrap();
        assert_eq!(m.symbol(), "m");

        let meter = lookup_unit("meter").unwrap();
        assert_eq!(meter.symbol(), "m");

        let meters = lookup_unit("meters").unwrap();
        assert_eq!(meters.symbol(), "m");
    }

    #[test]
    fn test_lookup_case_insensitive() {
        let m1 = lookup_unit("M").unwrap();
        let m2 = lookup_unit("m").unwrap();
        let m3 = lookup_unit("METER").unwrap();

        assert_eq!(m1.dimension(), m2.dimension());
        assert_eq!(m2.dimension(), m3.dimension());
    }

    #[test]
    fn test_parse_simple_unit() {
        let m = parse_unit("m").unwrap();
        assert_eq!(m.dimension(), M.dimension());

        let km = parse_unit("km").unwrap();
        assert_eq!(km.dimension(), KM.dimension());
    }

    #[test]
    fn test_parse_unit_with_power() {
        let m2 = parse_unit("m^2").unwrap();
        let dim = m2.dimension();
        assert_eq!(dim.length, Rational8::new(2, 1));

        let s_inv = parse_unit("s^-1").unwrap();
        let dim = s_inv.dimension();
        assert_eq!(dim.time, Rational8::new(-1, 1));
    }

    #[test]
    fn test_parse_unit_division() {
        let velocity = parse_unit("m/s").unwrap();
        let dim = velocity.dimension();
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-1, 1));
    }

    #[test]
    fn test_parse_unit_product() {
        let momentum = parse_unit("kg m").unwrap();
        let dim = momentum.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.length, Rational8::ONE);

        // With asterisk
        let momentum2 = parse_unit("kg*m").unwrap();
        assert_eq!(momentum2.dimension(), momentum.dimension());
    }

    #[test]
    fn test_parse_complex_unit() {
        // Energy: kg m^2 / s^2
        let energy = parse_unit("kg m^2 / s^2").unwrap();
        let dim = energy.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.length, Rational8::new(2, 1));
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_parse_acceleration() {
        let accel = parse_unit("m/s^2").unwrap();
        let dim = accel.dimension();
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_parse_quantity_simple() {
        let q = parse_quantity("100 km").unwrap();
        assert!((q.value() - 100.0).abs() < 1e-10);
        assert_eq!(q.unit().dimension(), KM.dimension());
    }

    #[test]
    fn test_parse_quantity_velocity() {
        let q = parse_quantity("10 m/s").unwrap();
        assert!((q.value() - 10.0).abs() < 1e-10);
        let dim = q.unit().dimension();
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-1, 1));
    }

    #[test]
    fn test_parse_quantity_scientific() {
        let q = parse_quantity("1.5e8 m").unwrap();
        assert!((q.value() - 1.5e8).abs() < 1.0);
    }

    #[test]
    fn test_parse_quantity_negative() {
        let q = parse_quantity("-3.14 rad").unwrap();
        assert!((q.value() - (-3.14)).abs() < 1e-10);
    }

    #[test]
    fn test_unit_from_str() {
        let m: Unit = "m".parse().unwrap();
        assert_eq!(m.dimension(), M.dimension());

        let velocity: Unit = "km/h".parse().unwrap();
        let expected_dim = (&*KM / &*H).dimension();
        assert_eq!(velocity.dimension(), expected_dim);
    }

    #[test]
    fn test_quantity_from_str() {
        let q: Quantity = "100 km".parse().unwrap();
        assert!((q.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_unknown_unit_error() {
        let result = parse_unit("foo");
        assert!(matches!(result, Err(UnitError::UnknownUnit(_))));
    }

    #[test]
    fn test_astrophysical_units() {
        let pc = parse_unit("pc").unwrap();
        let au = parse_unit("AU").unwrap();
        let ly = parse_unit("ly").unwrap();

        // All are length units
        assert_eq!(pc.dimension(), M.dimension());
        assert_eq!(au.dimension(), M.dimension());
        assert_eq!(ly.dimension(), M.dimension());
    }

    #[test]
    fn test_imperial_units() {
        let ft = parse_unit("ft").unwrap();
        let mi = parse_unit("mi").unwrap();
        let lb = parse_unit("lb").unwrap();

        assert_eq!(ft.dimension(), M.dimension());
        assert_eq!(mi.dimension(), M.dimension());
        assert_eq!(lb.dimension(), KG.dimension());
    }

    #[test]
    fn test_dimensionless() {
        let d = parse_unit("").unwrap();
        assert!(d.is_dimensionless());
    }

    #[test]
    fn test_fractional_power() {
        let sqrt_m = parse_unit("m^1/2").unwrap();
        let dim = sqrt_m.dimension();
        assert_eq!(dim.length, Rational8::new(1, 2));
    }
}
