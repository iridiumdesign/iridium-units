//! String parsing for units and quantities.
//!
//! This module provides flexible parsing of unit strings from various formats,
//! including technical documents, academic papers, and user input. It supports
//! Unicode symbols, LaTeX notation, natural language ("per"), and parenthesized
//! expressions.
//!
//! # Quick Start
//!
//! ```
//! use iridium_units::prelude::*;
//! use std::str::FromStr;
//!
//! // Parse simple units
//! let meter = Unit::from_str("m").unwrap();
//! let velocity = Unit::from_str("m/s").unwrap();
//!
//! // Parse quantities (value + unit)
//! let distance = Quantity::from_str("100 km").unwrap();
//! let speed = Quantity::from_str("9.8 m/s^2").unwrap();
//!
//! // Unicode and technical formats work too
//! let area = parse_unit("m²").unwrap();           // Unicode superscript
//! let wavelength = parse_unit("µm").unwrap();     // Unicode micro
//! let flux = parse_unit("erg/cm²/s").unwrap();    // Mixed formats
//! ```
//!
//! # Supported Syntax
//!
//! ## Basic Unit Strings
//!
//! | Format | Examples |
//! |--------|----------|
//! | Simple units | `"m"`, `"meter"`, `"meters"`, `"km"`, `"kg"` |
//! | Powers | `"m^2"`, `"s^-1"`, `"m^1/2"` (fractional) |
//! | Multiplication | `"kg m"`, `"kg*m"`, `"kg.m"` |
//! | Division | `"m/s"`, `"kg/m^3"`, `"erg/cm^2/s"` |
//! | Combined | `"kg m^2 / s^2"`, `"m/s^2"` |
//!
//! ## Unicode Support
//!
//! The parser automatically normalizes Unicode characters:
//!
//! | Unicode | Normalized | Description |
//! |---------|------------|-------------|
//! | `µ`, `μ` | `u` | Micro sign, Greek mu |
//! | `Ω` | `ohm` | Ohm sign, Greek Omega |
//! | `Å` | `angstrom` | Angstrom sign |
//! | `°` | `deg` | Degree symbol |
//! | `′` | `arcmin` | Prime (arc minute) |
//! | `″` | `arcsec` | Double prime (arc second) |
//! | `²`, `³`, etc. | `^2`, `^3` | Superscript digits |
//! | `⁻¹` | `^-1` | Superscript negative |
//! | `·`, `×` | `*` | Multiplication signs |
//! | `÷` | `/` | Division sign |
//!
//! ```
//! # use iridium_units::prelude::*;
//! // All of these work:
//! let _ = parse_unit("m²").unwrap();      // Unicode superscript
//! let _ = parse_unit("s⁻¹").unwrap();     // Negative superscript
//! let _ = parse_unit("µm").unwrap();      // Micro symbol
//! let _ = parse_unit("kg·m").unwrap();    // Middle dot multiplication
//! let _ = parse_unit("Ω").unwrap();       // Ohm symbol
//! ```
//!
//! ## LaTeX Notation
//!
//! Common LaTeX patterns are recognized:
//!
//! | LaTeX | Normalized | Description |
//! |-------|------------|-------------|
//! | `m^{2}` | `m^2` | Braced exponents |
//! | `\cdot` | `*` | Multiplication |
//! | `\times` | `*` | Multiplication |
//! | `\mu` | `u` | Micro prefix |
//! | `\Omega` | `ohm` | Ohm |
//!
//! ```
//! # use iridium_units::prelude::*;
//! let energy = parse_unit("kg m^{2} / s^{2}").unwrap();
//! let product = parse_unit(r"kg \cdot m").unwrap();
//! ```
//!
//! ## Natural Language ("per" notation)
//!
//! The word "per" is converted to division:
//!
//! ```
//! # use iridium_units::prelude::*;
//! let speed = parse_unit("km per hour").unwrap();
//! let rate = parse_unit("m per s").unwrap();
//! ```
//!
//! ## Astrophysical Subscript Notation
//!
//! Common astrophysical subscript patterns are recognized:
//!
//! | Input | Recognized as |
//! |-------|---------------|
//! | `M_sun`, `M_⊙` | Solar mass |
//! | `R_sun`, `R_⊙` | Solar radius |
//! | `L_sun`, `L_⊙` | Solar luminosity |
//! | `M_jup` | Jupiter mass |
//! | `R_jup` | Jupiter radius |
//! | `M_earth`, `M_⊕` | Earth mass |
//! | `R_earth`, `R_⊕` | Earth radius |
//!
//! ```
//! # use iridium_units::prelude::*;
//! let stellar_mass = parse_unit("M_sun").unwrap();
//! let planet_radius = parse_unit("R_jup").unwrap();
//! ```
//!
//! ## Parentheses
//!
//! Parentheses can be used for grouping:
//!
//! ```
//! # use iridium_units::prelude::*;
//! let force = parse_unit("(kg m)/s^2").unwrap();
//! let accel = parse_unit("m/(s^2)").unwrap();
//! let squared_vel = parse_unit("(m/s)^2").unwrap();
//! ```
//!
//! ## Quantity Strings
//!
//! Quantities combine a numeric value with a unit:
//!
//! ```
//! # use iridium_units::prelude::*;
//! let distance = parse_quantity("100 km").unwrap();
//! let speed = parse_quantity("9.8 m/s^2").unwrap();
//! let wavelength = parse_quantity("500 nm").unwrap();
//! let scientific = parse_quantity("1.5e8 m").unwrap();
//! let negative = parse_quantity("-3.14 rad").unwrap();
//! ```
//!
//! # Error Handling and Suggestions
//!
//! When a unit name is not recognized, the parser suggests similar unit names:
//!
//! ```
//! # use iridium_units::prelude::*;
//! let result = parse_unit("metrs");
//! match result {
//!     Err(UnitError::UnknownUnit { name, suggestions }) => {
//!         println!("Unknown unit '{}', did you mean: {:?}?", name, suggestions);
//!         // Output: Unknown unit 'metrs', did you mean: ["meters", "meter"]?
//!     }
//!     _ => {}
//! }
//! ```
//!
//! # Custom Unit Registry
//!
//! For applications that need custom units (e.g., from a database), use [`UnitRegistry`]:
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! // Create a registry with all built-in units
//! let registry = UnitRegistry::with_builtins();
//!
//! // Parse using the registry
//! let unit = registry.parse_unit("m/s").unwrap();
//! let quantity = registry.parse_quantity("100 km").unwrap();
//!
//! // Create a custom registry with additional units
//! let custom_registry = UnitRegistry::with_builtins()
//!     .with_alias(&["custom_length", "cl"], M.clone());
//!
//! let custom = custom_registry.parse_unit("custom_length").unwrap();
//! ```
//!
//! ## Registry API
//!
//! | Method | Description |
//! |--------|-------------|
//! | `new()` | Create empty registry |
//! | `with_builtins()` | Create with all standard units |
//! | `register(&mut self, names, unit)` | Add a unit with aliases |
//! | `with_unit(self, names, unit)` | Builder: add unit |
//! | `with_alias(self, names, unit)` | Builder: add alias |
//! | `lookup(name)` | Look up unit by name |
//! | `parse_unit(s)` | Parse unit string |
//! | `parse_quantity(s)` | Parse quantity string |
//! | `merge(&mut self, other)` | Merge another registry |
//!
//! # Available Unit Names
//!
//! ## SI Units
//!
//! - **Base**: `m`, `s`, `kg`, `a` (ampere), `k` (kelvin), `mol`, `cd`, `rad`, `sr`
//! - **Length**: `km`, `cm`, `mm`, `um`/`µm`, `nm`, `pm`, `fm`
//! - **Time**: `ms`, `us`/`µs`, `ns`, `ps`, `min`, `h`/`hr`, `d`/`day`, `yr`
//! - **Mass**: `g`, `mg`, `ug`/`µg`, `t`/`tonne`
//! - **Frequency**: `hz`, `khz`, `mhz`, `ghz`, `thz`
//! - **Derived**: `n` (newton), `j` (joule), `w` (watt), `pa`, `c` (coulomb), `v`, `f` (farad), `ohm`/`Ω`
//! - **Energy**: `ev`, `kev`, `mev`, `gev`
//! - **Angle**: `deg`/`°`, `arcmin`/`′`, `arcsec`/`″`, `mas`, `uas`
//!
//! ## Astrophysical Units
//!
//! - **Distance**: `au`, `pc`, `kpc`, `mpc`, `gpc`, `ly`/`lightyear`
//! - **Solar**: `msun`/`M_sun`/`solar_mass`, `rsun`/`R_sun`, `lsun`/`L_sun`
//! - **Planetary**: `mjup`/`M_jup`, `rjup`/`R_jup`, `mearth`/`M_earth`, `rearth`/`R_earth`
//! - **Spectroscopic**: `angstrom`/`Å`, `jy` (jansky), `mjy`, `ujy`, `barn`
//! - **CGS**: `erg`, `dyn`, `gauss`
//!
//! ## Imperial Units
//!
//! - **Length**: `in`, `ft`, `yd`, `mi`, `nmi`
//! - **Mass**: `lb`, `oz`, `ton`
//! - **Other**: `psi`, `mph`, `hp`, `btu`, `gal`, `pt`, `qt`

use crate::dimension::Rational8;
use crate::error::{UnitError, UnitResult};
use crate::quantity::Quantity;
use crate::unit::Unit;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::RwLock;

// ============================================================================
// Levenshtein Distance and Suggestions
// ============================================================================

/// Calculate the Levenshtein distance between two strings.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev_row: Vec<usize> = (0..=b_len).collect();
    let mut curr_row: Vec<usize> = vec![0; b_len + 1];

    for i in 1..=a_len {
        curr_row[0] = i;
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[b_len]
}

/// Find similar unit names for suggestions.
fn find_similar_units(name: &str, registry: &HashMap<String, UnitEntry>, max_suggestions: usize) -> Vec<String> {
    let name_lower = name.to_lowercase();
    let threshold = (name_lower.len() / 2).clamp(2, 3);

    let mut candidates: Vec<(String, usize)> = registry
        .keys()
        .filter_map(|key| {
            let dist = levenshtein_distance(&name_lower, key);
            if dist <= threshold {
                Some((key.clone(), dist))
            } else {
                None
            }
        })
        .collect();

    candidates.sort_by_key(|(_, dist)| *dist);
    candidates.truncate(max_suggestions);
    candidates.into_iter().map(|(name, _)| name).collect()
}

// ============================================================================
// Unicode Normalization
// ============================================================================

/// Normalize Unicode characters to ASCII equivalents for parsing.
fn normalize_unicode(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 2);
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            // Greek letters
            'µ' | 'μ' => result.push('u'),  // micro sign (U+00B5) and Greek mu (U+03BC)
            '\u{2126}' | '\u{03A9}' => result.push_str("ohm"),  // Ohm sign (U+2126) and Greek Omega (U+03A9)
            'α' => result.push_str("alpha"),
            'β' => result.push_str("beta"),
            'γ' => result.push_str("gamma"),
            'δ' => result.push_str("delta"),
            'λ' => result.push_str("lambda"),
            'π' => result.push_str("pi"),

            // Symbols
            '\u{212B}' | '\u{00C5}' => result.push_str("angstrom"),  // Angstrom sign (U+212B) and A with ring (U+00C5)
            '°' => result.push_str("deg"),
            '′' => result.push_str("arcmin"),
            '″' => result.push_str("arcsec"),
            '℃' => result.push_str("degC"),
            '℉' => result.push_str("degF"),

            // Superscripts for powers
            '⁰' => result.push_str("^0"),
            '¹' => result.push_str("^1"),
            '²' => result.push_str("^2"),
            '³' => result.push_str("^3"),
            '⁴' => result.push_str("^4"),
            '⁵' => result.push_str("^5"),
            '⁶' => result.push_str("^6"),
            '⁷' => result.push_str("^7"),
            '⁸' => result.push_str("^8"),
            '⁹' => result.push_str("^9"),
            '⁻' => result.push_str("^-"),
            '⁺' => result.push_str("^+"),

            // Fractions
            '½' => result.push_str("^1/2"),
            '⅓' => result.push_str("^1/3"),
            '¼' => result.push_str("^1/4"),
            '⅔' => result.push_str("^2/3"),
            '¾' => result.push_str("^3/4"),

            // Multiplication signs
            '·' | '×' | '∙' | '⋅' => result.push('*'),

            // Division
            '÷' => result.push('/'),

            _ => result.push(c),
        }
        i += 1;
    }

    // Post-process to merge consecutive power operators
    // e.g., "m^-^2" -> "m^-2" and "m^^2" -> "m^2"
    result
        .replace("^^", "^")
        .replace("^-^", "^-")
        .replace("^+^", "^")
}

// ============================================================================
// LaTeX/Technical Format Support
// ============================================================================

/// Normalize LaTeX-style notation to standard format.
fn normalize_latex(s: &str) -> String {
    let mut result = s.to_string();

    // Handle LaTeX braces in exponents: m^{2} -> m^2
    while let Some(start) = result.find("^{") {
        if let Some(end) = result[start..].find('}') {
            let inner = &result[start + 2..start + end];
            result = format!("{}{}{}", &result[..start + 1], inner, &result[start + end + 1..]);
        } else {
            break;
        }
    }

    // Handle LaTeX commands
    result = result.replace(r"\cdot", "*");
    result = result.replace(r"\times", "*");
    result = result.replace(r"\mu", "u");
    result = result.replace(r"\alpha", "alpha");
    result = result.replace(r"\beta", "beta");
    result = result.replace(r"\gamma", "gamma");
    result = result.replace(r"\Omega", "ohm");
    result = result.replace(r"\AA", "angstrom");
    result = result.replace(r"\deg", "deg");
    result = result.replace(r"\prime", "arcmin");
    result = result.replace(r"\arcsec", "arcsec");
    result = result.replace(r"\arcmin", "arcmin");
    result = result.replace(r"\frac{1}{2}", "^1/2");

    result
}

/// Normalize "per" notation to division.
fn normalize_per_notation(s: &str) -> String {
    // Handle various "per" patterns (case insensitive)
    let result = s.to_string();

    // Use regex-like replacement for "per" patterns
    let words: Vec<&str> = result.split_whitespace().collect();
    let mut new_words = Vec::new();
    let mut i = 0;

    while i < words.len() {
        if words[i].eq_ignore_ascii_case("per") && i + 1 < words.len() {
            // Replace "per X" with "/ X"
            new_words.push("/");
            new_words.push(words[i + 1]);
            i += 2;
        } else {
            new_words.push(words[i]);
            i += 1;
        }
    }

    new_words.join(" ")
}

/// Normalize subscript notation for astrophysical units.
fn normalize_subscripts(s: &str) -> String {
    let mut result = s.to_string();

    // Common astrophysical subscript patterns
    let subscript_mappings = [
        ("M_sun", "msun"), ("m_sun", "msun"), ("M_⊙", "msun"),
        ("R_sun", "rsun"), ("r_sun", "rsun"), ("R_⊙", "rsun"),
        ("L_sun", "lsun"), ("l_sun", "lsun"), ("L_⊙", "lsun"),
        ("M_jup", "mjup"), ("m_jup", "mjup"),
        ("R_jup", "rjup"), ("r_jup", "rjup"),
        ("M_earth", "mearth"), ("m_earth", "mearth"), ("M_⊕", "mearth"),
        ("R_earth", "rearth"), ("r_earth", "rearth"), ("R_⊕", "rearth"),
        ("sol_mass", "msun"), ("solar_mass", "msun"),
        ("sol_rad", "rsun"), ("solar_rad", "rsun"),
        ("sol_lum", "lsun"), ("solar_lum", "lsun"),
        ("jup_mass", "mjup"), ("jupiter_mass", "mjup"),
        ("jup_rad", "rjup"), ("jupiter_rad", "rjup"),
        ("earth_mass", "mearth"),
        ("earth_rad", "rearth"),
    ];

    for (pattern, replacement) in &subscript_mappings {
        // Case-insensitive replacement
        let pattern_lower = pattern.to_lowercase();
        let result_lower = result.to_lowercase();
        if let Some(pos) = result_lower.find(&pattern_lower) {
            let before = &result[..pos];
            let after = &result[pos + pattern.len()..];
            result = format!("{}{}{}", before, replacement, after);
        }
    }

    result
}

// ============================================================================
// Parentheses Support
// ============================================================================

/// Check if parentheses in a string are balanced.
fn check_balanced_parens(s: &str) -> UnitResult<()> {
    let mut depth = 0i32;
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(UnitError::ParseError("unbalanced parentheses: unexpected ')'".into()));
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return Err(UnitError::ParseError("unbalanced parentheses: missing ')'".into()));
    }
    Ok(())
}

/// Find the position of a top-level division operator (respecting parentheses).
fn find_top_level_division(s: &str) -> Option<usize> {
    let mut depth = 0;
    let mut in_exponent = false;

    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '^' => in_exponent = true,
            '/' if depth == 0 && !in_exponent => return Some(i),
            _ if c.is_whitespace() && in_exponent => in_exponent = false,
            _ if !c.is_ascii_digit() && c != '-' && c != '+' && c != '/' && in_exponent => {
                in_exponent = false;
            }
            _ => {}
        }
    }
    None
}

/// Parse an expression that may contain parentheses.
fn parse_with_parens(s: &str, registry: &HashMap<String, UnitEntry>) -> UnitResult<Unit> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(Unit::dimensionless());
    }

    check_balanced_parens(s)?;

    // Check for top-level division first
    if let Some(div_pos) = find_top_level_division(s) {
        let numerator = parse_with_parens(&s[..div_pos], registry)?;
        let denominator = parse_with_parens(&s[div_pos + 1..], registry)?;
        return Ok(&numerator / &denominator);
    }

    // Check for parenthesized expression with optional power
    if s.starts_with('(') {
        if let Some(close_pos) = find_matching_paren(s, 0) {
            let inner = &s[1..close_pos];
            let after = &s[close_pos + 1..];

            let inner_unit = parse_with_parens(inner, registry)?;

            // Check for power after closing paren
            if let Some(power_str) = after.strip_prefix('^') {
                let power = parse_power(power_str)?;
                let result = inner_unit.pow(power);
                return Ok(result);
            } else if after.is_empty() {
                return Ok(inner_unit);
            } else {
                // There's more to parse - multiply
                let rest = parse_with_parens(after.trim_start_matches(['*', ' ']), registry)?;
                return Ok(&inner_unit * &rest);
            }
        }
    }

    // No parentheses at this level - parse as product
    parse_unit_product_with_registry(s, registry)
}

/// Find the position of the matching closing parenthesis.
fn find_matching_paren(s: &str, open_pos: usize) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    if chars.get(open_pos) != Some(&'(') {
        return None;
    }

    let mut depth = 1;
    for (i, &c) in chars.iter().enumerate().skip(open_pos + 1) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

// ============================================================================
// UnitRegistry Struct
// ============================================================================

/// A registry mapping unit names to Unit values.
///
/// The registry provides a way to look up units by name, with support for
/// custom units beyond the built-in set. This is useful for applications
/// that need to populate unit definitions from external sources (e.g., databases).
///
/// # Examples
///
/// ```
/// use iridium_units::prelude::*;
///
/// // Create a registry with built-in units
/// let registry = UnitRegistry::with_builtins();
/// let meter = registry.lookup("m").unwrap();
///
/// // Parse units using the registry
/// let velocity = registry.parse_unit("m/s").unwrap();
/// ```
#[derive(Clone)]
pub struct UnitRegistry {
    entries: HashMap<String, UnitEntry>,
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        UnitRegistry {
            entries: HashMap::new(),
        }
    }

    /// Create a new registry with all built-in units registered.
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        register_builtin_units(&mut registry.entries);
        register_extended_aliases(&mut registry.entries);
        registry
    }

    /// Register a unit with one or more names.
    pub fn register(&mut self, names: &[&str], unit: Unit) {
        let entry = UnitEntry { unit };
        for name in names {
            self.entries.insert(name.to_lowercase(), entry.clone());
        }
    }

    /// Builder method to register a unit.
    pub fn with_unit(mut self, names: &[&str], unit: Unit) -> Self {
        self.register(names, unit);
        self
    }

    /// Builder method to register an alias (same as with_unit).
    pub fn with_alias(mut self, names: &[&str], unit: Unit) -> Self {
        self.register(names, unit);
        self
    }

    /// Register multiple units at once.
    pub fn register_many(&mut self, units: Vec<(&[&str], Unit)>) {
        for (names, unit) in units {
            self.register(names, unit);
        }
    }

    /// Look up a unit by name.
    pub fn lookup(&self, name: &str) -> Option<Unit> {
        self.entries.get(&name.to_lowercase()).map(|e| e.unit.clone())
    }

    /// Parse a unit string using this registry.
    pub fn parse_unit(&self, s: &str) -> UnitResult<Unit> {
        parse_unit_with_registry(s, &self.entries)
    }

    /// Parse a quantity string using this registry.
    pub fn parse_quantity(&self, s: &str) -> UnitResult<Quantity> {
        parse_quantity_with_registry(s, &self.entries)
    }

    /// Merge another registry into this one.
    ///
    /// Entries from the other registry will overwrite existing entries
    /// with the same name.
    pub fn merge(&mut self, other: &UnitRegistry) {
        for (name, entry) in &other.entries {
            self.entries.insert(name.clone(), entry.clone());
        }
    }

    /// Get the number of registered unit names.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get all registered unit names.
    pub fn names(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }
}

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
        register_extended_aliases(&mut map);
        RwLock::new(map)
    };
}

/// Register all built-in units with the registry.
fn register_builtin_units(map: &mut HashMap<String, UnitEntry>) {
    use crate::systems::si::*;
    use crate::systems::astrophysical::{
        AU, PARSEC, KPC, MPC, GPC, LIGHT_YEAR,
        SOLAR_MASS, SOLAR_RADIUS, SOLAR_LUMINOSITY,
        JUPITER_MASS, JUPITER_RADIUS, EARTH_MASS, EARTH_RADIUS,
        ANGSTROM, JANSKY, MJY, UJY, BARN,
        ERG, DYN, GAUSS,
    };
    use crate::systems::imperial::*;
    use crate::systems::cgs::{CENTIMETER, GRAM};

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

/// Register extended Unicode and academic aliases.
fn register_extended_aliases(map: &mut HashMap<String, UnitEntry>) {
    use crate::systems::si::*;
    use crate::systems::astrophysical::{
        SOLAR_MASS, SOLAR_RADIUS, SOLAR_LUMINOSITY,
        JUPITER_MASS, JUPITER_RADIUS, EARTH_MASS, EARTH_RADIUS,
        ANGSTROM,
    };

    macro_rules! register {
        ($map:expr, $unit:expr, $($name:expr),+) => {
            let entry = UnitEntry { unit: $unit.clone() };
            $(
                $map.insert($name.to_lowercase(), entry.clone());
            )+
        };
    }

    // Unicode micro symbol aliases
    register!(map, *UM, "µm");
    register!(map, *US, "µs");
    register!(map, *UG, "µg");

    // Ohm with Unicode
    register!(map, *OHM, "ω");

    // Angstrom with Unicode
    register!(map, *ANGSTROM, "å");

    // Degree symbol
    register!(map, *DEG, "°");

    // Arc minute/second with Unicode
    register!(map, *ARCMIN, "′");
    register!(map, *ARCSEC, "″");

    // Extended astrophysical aliases
    register!(map, *SOLAR_MASS, "m⊙", "solmass", "sol_mass");
    register!(map, *SOLAR_RADIUS, "r⊙", "solrad", "sol_rad", "solarradius");
    register!(map, *SOLAR_LUMINOSITY, "l⊙", "sollum", "sol_lum", "solarluminosity");
    register!(map, *JUPITER_MASS, "m_jupiter", "jupitermass");
    register!(map, *JUPITER_RADIUS, "r_jupiter", "jupiterradius");
    register!(map, *EARTH_MASS, "m⊕", "earthmass");
    register!(map, *EARTH_RADIUS, "r⊕", "earthradius");
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
/// - Unicode: "m²", "µm", "Ω", "°", "′", "″"
/// - LaTeX: "m^{2}", "\cdot", "\mu"
/// - Per notation: "km per hour"
/// - Subscripts: "M_sun", "R_jup"
/// - Parentheses: "(kg m)/s^2"
pub fn parse_unit(s: &str) -> UnitResult<Unit> {
    let registry = UNIT_REGISTRY.read()
        .map_err(|_| UnitError::ParseError("failed to acquire registry lock".into()))?;
    parse_unit_with_registry(s, &registry)
}

/// Parse a unit string using a specific registry.
fn parse_unit_with_registry(s: &str, registry: &HashMap<String, UnitEntry>) -> UnitResult<Unit> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(Unit::dimensionless());
    }

    // Apply normalizations in order
    let normalized = normalize_unicode(s);
    let normalized = normalize_latex(&normalized);
    let normalized = normalize_per_notation(&normalized);
    let normalized = normalize_subscripts(&normalized);

    // Check for parentheses - use the parentheses-aware parser
    if normalized.contains('(') || normalized.contains(')') {
        return parse_with_parens(&normalized, registry);
    }

    // Split by division, but be careful not to split inside exponents
    // Exponents like "m^1/2" should not be split at the "/"
    let parts = split_unit_by_division(&normalized);

    match parts.len() {
        1 => parse_unit_product_with_registry(&parts[0], registry),
        2 => {
            let numerator = parse_unit_product_with_registry(&parts[0], registry)?;
            let denominator = parse_unit_product_with_registry(&parts[1], registry)?;
            Ok(&numerator / &denominator)
        }
        _ => {
            // Multiple divisions: a/b/c = a / (b * c)
            let numerator = parse_unit_product_with_registry(&parts[0], registry)?;
            let mut denominator = parse_unit_product_with_registry(&parts[1], registry)?;
            for part in &parts[2..] {
                let next = parse_unit_product_with_registry(part, registry)?;
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

    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c == '^' {
            in_exponent = true;
            current.push(c);
        } else if c == '/' && !in_exponent {
            // This is a unit division
            parts.push(current.trim().to_string());
            current = String::new();
        } else if c == '/' && in_exponent {
            // Check if this is a fractional power or a unit division
            // It's a fractional power only if followed by a digit (possibly with sign)
            let next_idx = i + 1;
            let is_fraction = if next_idx < chars.len() {
                let next = chars[next_idx];
                next.is_ascii_digit() || (next == '-' && next_idx + 1 < chars.len() && chars[next_idx + 1].is_ascii_digit())
            } else {
                false
            };

            if is_fraction {
                // This is part of a fractional exponent like ^1/2
                current.push(c);
            } else {
                // Exponent has ended, this is a unit division
                in_exponent = false;
                parts.push(current.trim().to_string());
                current = String::new();
            }
        } else if c.is_whitespace() && in_exponent {
            // Exponent ends at whitespace
            in_exponent = false;
            current.push(c);
        } else if !c.is_ascii_digit() && c != '-' && c != '+' && in_exponent {
            // Exponent ends at non-numeric character
            in_exponent = false;
            current.push(c);
        } else {
            current.push(c);
        }

        i += 1;
    }

    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    if parts.is_empty() {
        parts.push(String::new());
    }

    parts
}

/// Parse a product of units using a specific registry.
fn parse_unit_product_with_registry(s: &str, registry: &HashMap<String, UnitEntry>) -> UnitResult<Unit> {
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

    let mut result = parse_unit_with_power_registry(tokens[0], registry)?;
    for token in &tokens[1..] {
        let next = parse_unit_with_power_registry(token, registry)?;
        result = &result * &next;
    }

    Ok(result)
}

/// Parse a single unit with optional power using a specific registry.
fn parse_unit_with_power_registry(s: &str, registry: &HashMap<String, UnitEntry>) -> UnitResult<Unit> {
    let s = s.trim();

    // Check for power notation
    if let Some(idx) = s.find('^') {
        let (name, power_str) = s.split_at(idx);
        let power_str = &power_str[1..]; // Skip the '^'

        let power = parse_power(power_str)?;
        let base_unit = lookup_simple_unit_with_registry(name, registry)?;
        Ok(base_unit.pow(power))
    } else if let Some(idx) = s.find("**") {
        // Python-style power notation
        let (name, power_str) = s.split_at(idx);
        let power_str = &power_str[2..]; // Skip the '**'

        let power = parse_power(power_str)?;
        let base_unit = lookup_simple_unit_with_registry(name, registry)?;
        Ok(base_unit.pow(power))
    } else {
        lookup_simple_unit_with_registry(s, registry)
    }
}

/// Parse a power exponent (integer or fraction)
fn parse_power(s: &str) -> UnitResult<Rational8> {
    let s = s.trim();

    // Check for fraction notation (e.g., "1/2")
    if let Some(idx) = s.find('/') {
        let (num_str, den_str) = s.split_at(idx);
        let den_str = &den_str[1..];

        let num: i16 = num_str.trim().parse().map_err(|_| {
            UnitError::ParseError(format!("invalid power numerator: {}", num_str))
        })?;
        let den: i16 = den_str.trim().parse().map_err(|_| {
            UnitError::ParseError(format!("invalid power denominator: {}", den_str))
        })?;

        if den == 0 {
            return Err(UnitError::ParseError("power denominator cannot be zero".into()));
        }

        Ok(Rational8::new(num, den))
    } else {
        // Simple integer power
        let exp: i16 = s.parse().map_err(|_| {
            UnitError::ParseError(format!("invalid power: {}", s))
        })?;
        Ok(Rational8::new(exp, 1))
    }
}

/// Look up a simple unit name using a specific registry.
fn lookup_simple_unit_with_registry(name: &str, registry: &HashMap<String, UnitEntry>) -> UnitResult<Unit> {
    let name = name.trim();
    let name_lower = name.to_lowercase();

    if let Some(entry) = registry.get(&name_lower) {
        return Ok(entry.unit.clone());
    }

    // Unit not found - provide helpful suggestions
    let suggestions = find_similar_units(name, registry, 3);
    Err(UnitError::UnknownUnit {
        name: name.to_string(),
        suggestions,
    })
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
    let registry = UNIT_REGISTRY.read()
        .map_err(|_| UnitError::ParseError("failed to acquire registry lock".into()))?;
    parse_quantity_with_registry(s, &registry)
}

/// Parse a quantity string using a specific registry.
fn parse_quantity_with_registry(s: &str, registry: &HashMap<String, UnitEntry>) -> UnitResult<Quantity> {
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

    let unit = parse_unit_with_registry(unit_str, registry)?;

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
        assert!(matches!(result, Err(UnitError::UnknownUnit { .. })));
    }

    #[test]
    fn test_unknown_unit_with_suggestions() {
        let result = parse_unit("metrs");
        match result {
            Err(UnitError::UnknownUnit { name, suggestions }) => {
                assert_eq!(name, "metrs");
                // Should suggest "meters" or similar
                assert!(!suggestions.is_empty());
            }
            _ => panic!("Expected UnknownUnit error"),
        }
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

    // ========================================================================
    // Unicode Parsing Tests
    // ========================================================================

    #[test]
    fn test_unicode_superscript_power() {
        let m2 = parse_unit("m²").unwrap();
        let dim = m2.dimension();
        assert_eq!(dim.length, Rational8::new(2, 1));

        let m3 = parse_unit("m³").unwrap();
        let dim = m3.dimension();
        assert_eq!(dim.length, Rational8::new(3, 1));
    }

    #[test]
    fn test_unicode_negative_power() {
        let s_inv = parse_unit("s⁻¹").unwrap();
        let dim = s_inv.dimension();
        assert_eq!(dim.time, Rational8::new(-1, 1));

        let accel = parse_unit("m/s²").unwrap();
        let dim = accel.dimension();
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_unicode_micro() {
        let um = parse_unit("µm").unwrap();
        assert_eq!(um.dimension(), M.dimension());
    }

    #[test]
    fn test_unicode_multiplication() {
        let momentum = parse_unit("kg·m").unwrap();
        let dim = momentum.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.length, Rational8::ONE);

        let momentum2 = parse_unit("kg×m").unwrap();
        assert_eq!(momentum2.dimension(), momentum.dimension());
    }

    #[test]
    fn test_unicode_division() {
        let velocity = parse_unit("m÷s").unwrap();
        let dim = velocity.dimension();
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-1, 1));
    }

    // ========================================================================
    // LaTeX Format Tests
    // ========================================================================

    #[test]
    fn test_latex_braces() {
        let m2 = parse_unit("m^{2}").unwrap();
        let dim = m2.dimension();
        assert_eq!(dim.length, Rational8::new(2, 1));

        let energy = parse_unit("kg m^{2} / s^{2}").unwrap();
        let dim = energy.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.length, Rational8::new(2, 1));
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_latex_cdot() {
        let momentum = parse_unit(r"kg \cdot m").unwrap();
        let dim = momentum.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.length, Rational8::ONE);
    }

    #[test]
    fn test_latex_times() {
        let area = parse_unit(r"m \times m").unwrap();
        let dim = area.dimension();
        assert_eq!(dim.length, Rational8::new(2, 1));
    }

    // ========================================================================
    // Per Notation Tests
    // ========================================================================

    #[test]
    fn test_per_notation() {
        let velocity = parse_unit("km per hour").unwrap();
        let dim = velocity.dimension();
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-1, 1));

        let velocity2 = parse_unit("m per s").unwrap();
        assert_eq!(velocity2.dimension(), (&*M / &*S).dimension());
    }

    #[test]
    fn test_per_notation_case_insensitive() {
        let v1 = parse_unit("km PER hour").unwrap();
        let v2 = parse_unit("km Per hour").unwrap();
        assert_eq!(v1.dimension(), v2.dimension());
    }

    // ========================================================================
    // Subscript/Astrophysical Notation Tests
    // ========================================================================

    #[test]
    fn test_subscript_solar() {
        let msun = parse_unit("M_sun").unwrap();
        assert_eq!(msun.dimension(), KG.dimension());

        let rsun = parse_unit("R_sun").unwrap();
        assert_eq!(rsun.dimension(), M.dimension());
    }

    #[test]
    fn test_subscript_planetary() {
        let mjup = parse_unit("M_jup").unwrap();
        assert_eq!(mjup.dimension(), KG.dimension());

        let mearth = parse_unit("M_earth").unwrap();
        assert_eq!(mearth.dimension(), KG.dimension());
    }

    // ========================================================================
    // Parentheses Support Tests
    // ========================================================================

    #[test]
    fn test_parentheses_simple() {
        let force = parse_unit("(kg m)/s^2").unwrap();
        let dim = force.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_parentheses_denominator() {
        let unit = parse_unit("m/(s^2)").unwrap();
        let dim = unit.dimension();
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_parentheses_with_power() {
        let unit = parse_unit("(m/s)^2").unwrap();
        let dim = unit.dimension();
        assert_eq!(dim.length, Rational8::new(2, 1));
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_parentheses_complex() {
        let unit = parse_unit("(kg m^2)/(s^2)").unwrap();
        let dim = unit.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.length, Rational8::new(2, 1));
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_unbalanced_parens_error() {
        let result = parse_unit("(m/s");
        assert!(result.is_err());

        let result = parse_unit("m/s)");
        assert!(result.is_err());
    }

    // ========================================================================
    // UnitRegistry Tests
    // ========================================================================

    #[test]
    fn test_registry_new() {
        let registry = UnitRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_with_builtins() {
        let registry = UnitRegistry::with_builtins();
        assert!(!registry.is_empty());

        let m = registry.lookup("m").unwrap();
        assert_eq!(m.dimension(), M.dimension());

        let km = registry.lookup("km").unwrap();
        assert_eq!(km.dimension(), KM.dimension());
    }

    #[test]
    fn test_registry_register() {
        let mut registry = UnitRegistry::new();
        registry.register(&["custom", "cust"], M.clone());

        let custom = registry.lookup("custom").unwrap();
        assert_eq!(custom.dimension(), M.dimension());

        let cust = registry.lookup("cust").unwrap();
        assert_eq!(cust.dimension(), M.dimension());
    }

    #[test]
    fn test_registry_builder_pattern() {
        let registry = UnitRegistry::new()
            .with_unit(&["custom1"], M.clone())
            .with_alias(&["custom2", "c2"], KG.clone());

        assert!(registry.lookup("custom1").is_some());
        assert!(registry.lookup("custom2").is_some());
        assert!(registry.lookup("c2").is_some());
    }

    #[test]
    fn test_registry_parse_unit() {
        let registry = UnitRegistry::with_builtins();

        let velocity = registry.parse_unit("m/s").unwrap();
        assert_eq!(velocity.dimension(), (&*M / &*S).dimension());

        let energy = registry.parse_unit("kg m^2 / s^2").unwrap();
        let dim = energy.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.length, Rational8::new(2, 1));
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }

    #[test]
    fn test_registry_parse_quantity() {
        let registry = UnitRegistry::with_builtins();

        let q = registry.parse_quantity("100 km").unwrap();
        assert!((q.value() - 100.0).abs() < 1e-10);
        assert_eq!(q.unit().dimension(), KM.dimension());
    }

    #[test]
    fn test_registry_merge() {
        let mut registry1 = UnitRegistry::new();
        registry1.register(&["unit1"], M.clone());

        let mut registry2 = UnitRegistry::new();
        registry2.register(&["unit2"], KG.clone());

        registry1.merge(&registry2);

        assert!(registry1.lookup("unit1").is_some());
        assert!(registry1.lookup("unit2").is_some());
    }

    #[test]
    fn test_registry_names() {
        let mut registry = UnitRegistry::new();
        registry.register(&["a", "b", "c"], M.clone());

        let names = registry.names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"a"));
        assert!(names.contains(&"b"));
        assert!(names.contains(&"c"));
    }

    // ========================================================================
    // Levenshtein Distance Tests
    // ========================================================================

    #[test]
    fn test_levenshtein_distance_identical() {
        assert_eq!(levenshtein_distance("meter", "meter"), 0);
    }

    #[test]
    fn test_levenshtein_distance_one_char() {
        assert_eq!(levenshtein_distance("meter", "meters"), 1);
        assert_eq!(levenshtein_distance("metr", "meter"), 1);
    }

    #[test]
    fn test_levenshtein_distance_swap() {
        assert_eq!(levenshtein_distance("metrs", "meters"), 1);
    }

    #[test]
    fn test_levenshtein_distance_empty() {
        assert_eq!(levenshtein_distance("", "meter"), 5);
        assert_eq!(levenshtein_distance("meter", ""), 5);
    }

    // ========================================================================
    // Normalization Function Tests
    // ========================================================================

    #[test]
    fn test_normalize_unicode() {
        assert_eq!(normalize_unicode("m²"), "m^2");
        assert_eq!(normalize_unicode("s⁻¹"), "s^-1");
        assert_eq!(normalize_unicode("kg·m"), "kg*m");
        assert_eq!(normalize_unicode("µm"), "um");
    }

    #[test]
    fn test_normalize_latex() {
        assert_eq!(normalize_latex("m^{2}"), "m^2");
        assert_eq!(normalize_latex(r"kg \cdot m"), "kg * m");
        assert_eq!(normalize_latex(r"\mu m"), "u m");
    }

    #[test]
    fn test_normalize_per_notation() {
        assert_eq!(normalize_per_notation("km per hour"), "km / hour");
        assert_eq!(normalize_per_notation("m per s"), "m / s");
    }

    #[test]
    fn test_normalize_subscripts() {
        let result = normalize_subscripts("M_sun");
        assert_eq!(result, "msun");
    }

    // ========================================================================
    // Combined/Integration Tests
    // ========================================================================

    #[test]
    fn test_astrophysical_flux_unit() {
        let flux = parse_unit("erg/cm^2/s").unwrap();
        let dim = flux.dimension();
        // Energy / area / time = mass * length^2 / time^2 / length^2 / time
        // = mass / time^3
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-3, 1));
    }

    #[test]
    fn test_unicode_astrophysical() {
        let flux = parse_unit("erg/cm²/s").unwrap();
        let dim = flux.dimension();
        assert_eq!(dim.mass, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-3, 1));
    }

    #[test]
    fn test_mixed_formats() {
        // Unicode superscript with division
        let accel = parse_unit("m·s⁻²").unwrap();
        let dim = accel.dimension();
        assert_eq!(dim.length, Rational8::ONE);
        assert_eq!(dim.time, Rational8::new(-2, 1));
    }
}
