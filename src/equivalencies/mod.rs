//! Equivalencies for converting between different physical domains.
//!
//! Equivalencies enable conversions between units with different dimensions
//! when there is a physical relationship between them. For example:
//!
//! - Wavelength ↔ Frequency (via speed of light)
//! - Mass ↔ Energy (via E=mc²)
//! - Parallax angle ↔ Distance
//!
//! # Example
//!
//! ```ignore
//! use iridium_units::prelude::*;
//! use iridium_units::equivalencies::spectral;
//!
//! let wavelength = 500.0 * NM;
//! let frequency = wavelength.to_equiv(&HZ, spectral()).unwrap();
//! ```

pub mod doppler;
pub mod mass_energy;
pub mod parallax;
pub mod spectral;
pub mod temperature;

use crate::error::{UnitError, UnitResult};
use crate::quantity::Quantity;
use crate::unit::Unit;
use std::sync::Arc;

/// An equivalency that enables conversion between different physical dimensions.
#[derive(Clone)]
pub struct Equivalency {
    /// Name of this equivalency
    pub name: &'static str,
    /// Function that attempts to create a converter between two units
    converter_fn: Arc<dyn Fn(&Unit, &Unit) -> Option<Converter> + Send + Sync>,
}

impl Equivalency {
    /// Create a new equivalency with a converter function.
    pub fn new<F>(name: &'static str, converter_fn: F) -> Self
    where
        F: Fn(&Unit, &Unit) -> Option<Converter> + Send + Sync + 'static,
    {
        Equivalency {
            name,
            converter_fn: Arc::new(converter_fn),
        }
    }

    /// Try to create a converter between two units.
    pub fn get_converter(&self, from: &Unit, to: &Unit) -> Option<Converter> {
        (self.converter_fn)(from, to)
    }
}

impl std::fmt::Debug for Equivalency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Equivalency({})", self.name)
    }
}

/// A converter that can transform values between units.
///
/// Converters use `Result<f64, String>` to handle invalid inputs
/// (e.g., zero wavelength, negative temperature, superluminal velocity).
pub struct Converter {
    /// Function to convert from source to target unit
    pub forward: Box<dyn Fn(f64) -> Result<f64, String> + Send + Sync>,
    /// Function to convert from target to source unit
    pub backward: Box<dyn Fn(f64) -> Result<f64, String> + Send + Sync>,
}

impl Converter {
    /// Create a new converter with forward and backward functions that can fail.
    pub fn new<F, B>(forward: F, backward: B) -> Self
    where
        F: Fn(f64) -> Result<f64, String> + Send + Sync + 'static,
        B: Fn(f64) -> Result<f64, String> + Send + Sync + 'static,
    {
        Converter {
            forward: Box::new(forward),
            backward: Box::new(backward),
        }
    }

    /// Create a converter from infallible functions (for backwards compatibility).
    pub fn new_infallible<F, B>(forward: F, backward: B) -> Self
    where
        F: Fn(f64) -> f64 + Send + Sync + 'static,
        B: Fn(f64) -> f64 + Send + Sync + 'static,
    {
        Converter {
            forward: Box::new(move |x| Ok(forward(x))),
            backward: Box::new(move |x| Ok(backward(x))),
        }
    }

    /// Apply the forward conversion.
    pub fn convert(&self, value: f64) -> Result<f64, String> {
        (self.forward)(value)
    }

    /// Apply the backward conversion.
    pub fn convert_back(&self, value: f64) -> Result<f64, String> {
        (self.backward)(value)
    }
}

impl Quantity {
    /// Convert to another unit using equivalencies.
    ///
    /// First tries a direct dimensional conversion. If that fails,
    /// tries each equivalency in order until one succeeds.
    pub fn to_equiv(&self, target: &Unit, equiv: Equivalency) -> UnitResult<Quantity> {
        self.to_equiv_list(target, &[equiv])
    }

    /// Convert to another unit using a list of equivalencies.
    pub fn to_equiv_list(&self, target: &Unit, equivs: &[Equivalency]) -> UnitResult<Quantity> {
        // First, try direct conversion
        if let Ok(q) = self.to(target) {
            return Ok(q);
        }

        // Try each equivalency
        for equiv in equivs {
            if let Some(converter) = equiv.get_converter(self.unit(), target) {
                // Convert to SI value first
                let si_value = self.value() * self.unit().scale();
                // Apply the equivalency conversion (may fail for invalid inputs)
                let converted_si = converter.convert(si_value).map_err(|msg| {
                    UnitError::NoEquivalency {
                        from: format!("{} ({})", self.unit(), msg),
                        to: target.to_string(),
                    }
                })?;
                // Convert to target unit
                let target_value = converted_si / target.scale();
                return Ok(Quantity::new(target_value, target.clone()));
            }
        }

        Err(UnitError::NoEquivalency {
            from: self.unit().to_string(),
            to: target.to_string(),
        })
    }
}

// Re-export commonly used equivalencies
pub use doppler::{doppler_optical, doppler_radio, doppler_relativistic};
pub use mass_energy::mass_energy;
pub use parallax::parallax;
pub use spectral::spectral;
pub use temperature::{temperature, temperature_energy};
