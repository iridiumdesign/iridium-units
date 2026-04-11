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
//! ```no_run
//! # #[cfg(feature = "astrophysics")]
//! # fn main() {
//! use iridium_units::prelude::*;
//! use iridium_units::equivalencies::spectral;
//!
//! let wavelength = 500.0 * NM;
//! let frequency = wavelength.to_equiv(&HZ, spectral()).unwrap();
//! # }
//! # #[cfg(not(feature = "astrophysics"))]
//! # fn main() {}
//! ```

#[cfg(feature = "astrophysics")]
pub mod brightness_temperature;
pub mod dimensionless_angles;
#[cfg(feature = "astrophysics")]
pub mod doppler;
#[cfg(feature = "logarithmic")]
pub mod logarithmic;
pub mod mass_energy;
#[cfg(feature = "astrophysics")]
pub mod parallax;
#[cfg(feature = "astrophysics")]
pub mod spectral;
#[cfg(feature = "astrophysics")]
pub mod spectral_density;
pub mod temperature;

use crate::error::{UnitError, UnitResult};
use crate::quantity::Quantity;
use crate::unit::Unit;
use std::sync::Arc;

/// Type alias for the converter factory function used by equivalencies.
pub type ConverterFn = Arc<dyn Fn(&Unit, &Unit) -> Option<Converter> + Send + Sync>;

/// An equivalency that enables conversion between different physical dimensions.
#[derive(Clone)]
pub struct Equivalency {
    /// Name of this equivalency
    pub name: &'static str,
    /// Function that attempts to create a converter between two units
    converter_fn: ConverterFn,
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
    pub fn to_equiv(&self, target: impl Into<Unit>, equiv: Equivalency) -> UnitResult<Quantity> {
        self.to_equiv_list(target, &[equiv])
    }

    /// Convert to another unit using a list of equivalencies.
    pub fn to_equiv_list(&self, target: impl Into<Unit>, equivs: &[Equivalency]) -> UnitResult<Quantity> {
        let target = target.into();
        // First, try direct conversion
        if let Ok(q) = self.to(&target) {
            return Ok(q);
        }

        // Try each equivalency
        for equiv in equivs {
            if let Some(converter) = equiv.get_converter(self.unit(), &target) {
                // Convert to SI value first (handles offset units like °C)
                let si_value = self.unit().to_si(self.value());
                // Apply the equivalency conversion (may fail for invalid inputs)
                let converted_si = converter.convert(si_value).map_err(|msg| {
                    UnitError::NoEquivalency {
                        from: format!("{} ({})", self.unit(), msg),
                        to: target.to_string(),
                    }
                })?;
                // Convert from SI to target unit (handles offset units like °C)
                let target_value = target.from_si(converted_si);
                return Ok(Quantity::new(target_value, target));
            }
        }

        Err(UnitError::NoEquivalency {
            from: self.unit().to_string(),
            to: target.to_string(),
        })
    }
}

// Re-export commonly used equivalencies
#[cfg(feature = "astrophysics")]
pub use brightness_temperature::{brightness_temperature, brightness_temperature_intensity, brightness_temperature_planck};
pub use dimensionless_angles::dimensionless_angles;
#[cfg(feature = "astrophysics")]
pub use doppler::{doppler_optical, doppler_radio, doppler_relativistic};
#[cfg(feature = "logarithmic")]
pub use logarithmic::{magnitude_flux, db_power, db_amplitude, dex_ratio};
pub use mass_energy::mass_energy;
#[cfg(feature = "astrophysics")]
pub use parallax::parallax;
#[cfg(feature = "astrophysics")]
pub use spectral::spectral;
#[cfg(feature = "astrophysics")]
pub use spectral_density::{spectral_density, ab_magnitude, ab_magnitude_lambda};
pub use temperature::{temperature, temperature_energy};
