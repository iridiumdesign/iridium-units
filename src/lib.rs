//! # iridium-units
//!
//! Units of measure for Rust with runtime dimensional analysis.
//!
//! In iridium-units a unit is a value, not a type: you parse it, store it,
//! pass it around, and combine it with other units, while the library tracks
//! dimensions and catches mismatched operations at runtime. This suits units
//! that aren't known until runtime — parsed from strings, configuration, file
//! formats, or a database — where encoding each unit as a distinct type isn't
//! practical. It provides multiple unit systems and named equivalencies for
//! converting between different physical domains (e.g., mass to energy).
//!
//! ## Quick Start
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! // Create quantities by multiplying values with units
//! // Units are const values — use them directly
//! let distance = 100.0 * M;
//! let time = 9.58 * S;
//! let speed = &distance / &time;
//!
//! // Convert between units
//! let speed_kmh = speed.to(&(KM / H)).unwrap();
//! println!("{}", speed_kmh); // ~37.58 km / h
//!
//! // Dimensional analysis is automatic
//! let energy = 10.0 * KG * M.pow(2) / S.pow(2);
//! let in_joules = energy.to(J).unwrap();
//! ```
//!
//! ## Unit Systems
//!
//! The library provides units from multiple systems:
//!
//! - **SI units**: meter, second, kilogram, ampere, kelvin, etc.
//! - **CGS units**: centimeter, gram, dyne, erg, etc.
//! - **Astrophysical units**: parsec, AU, solar mass, light year, etc.
//! - **Imperial units**: mile, foot, inch, pound, etc.
//!
//! ## Parsing Units and Quantities
//!
//! Units and quantities can be parsed from strings with flexible syntax support:
//!
//! ```no_run
//! # #[cfg(feature = "astrophysics")]
//! # fn main() {
//! use iridium_units::prelude::*;
//! use std::str::FromStr;
//!
//! // Parse units from strings
//! let meter = Unit::from_str("m").unwrap();
//! let velocity_unit = Unit::from_str("km/s").unwrap();
//! let accel_unit = Unit::from_str("m/s^2").unwrap();
//!
//! // Parse quantities (value + unit)
//! let distance: Quantity = "100 km".parse().unwrap();
//! let speed: Quantity = "9.8 m/s^2".parse().unwrap();
//!
//! // Unicode and technical formats are supported
//! let area = parse_unit("m²").unwrap();           // Unicode superscript
//! let wavelength = parse_unit("µm").unwrap();     // Unicode micro
//! let flux = parse_unit("erg/cm²/s").unwrap();    // Astrophysical
//! let latex = parse_unit("kg m^{2} / s^{2}").unwrap();  // LaTeX braces
//! let natural = parse_unit("km per hour").unwrap();     // Natural language
//! let astro = parse_unit("M_sun").unwrap();       // Astrophysical subscripts
//! let parens = parse_unit("(kg m)/s^2").unwrap(); // Parentheses
//! # }
//! # #[cfg(not(feature = "astrophysics"))]
//! # fn main() {}
//! ```
//!
//! See the [`parsing`] module for comprehensive documentation of supported formats.
//!
//! ## Custom Unit Registry
//!
//! For applications needing custom units (e.g., loaded from a database):
//!
//! ```
//! use iridium_units::prelude::*;
//!
//! let registry = UnitRegistry::with_builtins()
//!     .with_unit(&["my_unit", "mu"], Unit::from(M));
//!
//! let unit = registry.parse_unit("my_unit").unwrap();
//! ```
//!
//! ## Equivalencies
//!
//! Some physical quantities can be converted through physical laws even though
//! they have different dimensions:
//!
//! ```no_run
//! # #[cfg(feature = "astrophysics")]
//! # fn main() {
//! use iridium_units::prelude::*;
//! use iridium_units::equivalencies::spectral;
//!
//! let wavelength = 500.0 * NM;
//! let frequency = wavelength.to_equiv(HZ, spectral()).unwrap();
//! # }
//! # #[cfg(not(feature = "astrophysics"))]
//! # fn main() {}
//! ```

pub mod dimension;
pub mod error;
pub mod quantity;
pub mod unit;

pub mod constants;
pub mod equivalencies;
pub mod parsing;
pub mod systems;

// Re-export main types
pub use dimension::{Dimension, Rational16};
pub use error::{UnitError, UnitResult};
pub use quantity::{batch_convert, batch_convert_into, conversion_factor, Quantity};
pub use unit::base::BaseUnit;
pub use unit::Unit;

// Re-export parsing functions and types
pub use parsing::{
    lookup_unit, parse_quantity, parse_unit, register_unit, register_unit_override, UnitRegistry,
};

/// Prelude module for convenient imports.
///
/// This module re-exports the most commonly used types and units.
///
/// ```
/// use iridium_units::prelude::*;
/// ```
pub mod prelude {
    pub use crate::dimension::{Dimension, Rational16};
    pub use crate::error::{UnitError, UnitResult};
    pub use crate::quantity::{batch_convert, batch_convert_into, conversion_factor, Quantity};
    pub use crate::unit::base::BaseUnit;
    pub use crate::unit::Unit;

    // Re-export parsing functions and types
    pub use crate::parsing::{
        lookup_unit, parse_quantity, parse_unit, register_unit, register_unit_override,
        UnitRegistry,
    };

    // Re-export common SI units
    pub use crate::systems::si::{
        // Base units
        A,
        // Derived units
        C,
        CD,
        // Length
        CM,
        // Time
        DAY,
        // Temperature
        DEG_C,
        DEG_F,
        F,
        GHZ,
        H,
        HZ,
        J,
        K,
        KG,
        KHZ,
        KM,
        M,
        MHZ,
        MIN,
        MM,
        MOL,
        MS,
        N,
        NM,
        NS,
        OHM,
        PA,
        RAD,
        S,
        SR,
        THZ,
        UM,
        US,
        V,
        W,
        YR,
    };

    // Re-export astrophysical units
    #[cfg(feature = "astrophysics")]
    pub use crate::systems::astrophysical::{
        ANGSTROM, AU, BARN, DYN, ERG, GAUSS, JANSKY, LIGHT_YEAR, PARSEC, SOLAR_LUMINOSITY,
        SOLAR_MASS, SOLAR_RADIUS,
    };

    // Re-export CGS units
    #[cfg(feature = "cgs")]
    pub use crate::systems::cgs::{CENTIMETER, DYNE, ERG as ERG_CGS, GRAM};

    // Re-export imperial units
    pub use crate::systems::imperial::{FOOT, INCH, MILE, POUND, YARD};
}
