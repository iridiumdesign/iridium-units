//! Unit systems.
//!
//! This module provides predefined units from various unit systems:
//!
//! - [`si`]: SI (International System of Units)
//! - [`cgs`]: CGS (Centimeter-Gram-Second)
//! - [`astrophysical`]: Astrophysical units
//! - [`imperial`]: Imperial/US customary units
//! - [`logarithmic`]: Logarithmic units (magnitudes, decibels, dex)

pub mod astrophysical;
pub mod cgs;
pub mod imperial;
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
    pub use super::astrophysical::*;
    pub use super::cgs::GRAM;
    pub use super::imperial::{FOOT, INCH, MILE, POUND, YARD};
    pub use super::logarithmic::{MAG, APPARENT_MAG, ABSOLUTE_MAG, DB, DEX, MILLIMAG};
}
