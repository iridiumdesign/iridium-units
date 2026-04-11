//! Unit systems.
//!
//! This module provides predefined units from various unit systems:
//!
//! - [`si`] — SI (International System of Units)
//! - [`cgs`] — CGS (Centimeter-Gram-Second)
//! - [`astrophysical`] — Astrophysical units
//! - [`imperial`] — Imperial/US customary units
//! - [`logarithmic`] — Logarithmic units (magnitudes, decibels, dex)

#[cfg(feature = "astrophysics")]
pub mod astrophysical;
#[cfg(feature = "cgs")]
pub mod cgs;
pub mod imperial;
#[cfg(feature = "logarithmic")]
pub mod logarithmic;
pub mod si;

/// Shorthand module for quick unit access.
///
/// ```
/// use iridium_units::systems::u;
///
/// let speed = 10.0 * &*u::M / &*u::S;
/// ```
pub mod u {
    pub use super::si::*;
    #[cfg(feature = "astrophysics")]
    pub use super::astrophysical::*;
    #[cfg(feature = "cgs")]
    pub use super::cgs::GRAM;
    pub use super::imperial::{FOOT, INCH, MILE, POUND, YARD};
    #[cfg(feature = "logarithmic")]
    pub use super::logarithmic::{MAG, APPARENT_MAG, ABSOLUTE_MAG, DB, DEX, MILLIMAG};
}
