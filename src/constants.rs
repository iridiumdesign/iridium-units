//! Physical constants.
//!
//! This module provides fundamental physical constants used throughout
//! the library. Values are from CODATA 2018 where applicable.
//!
//! All constants are `f64` values in SI base units.
//!
//! # Examples
//!
//! ```
//! use iridium_units::prelude::*;
//! use iridium_units::constants::*;
//!
//! // Calculate Earth's surface gravity: g = GM/r²
//! let g = GRAVITATIONAL_CONSTANT * EARTH_MASS / (EARTH_RADIUS * EARTH_RADIUS);
//! assert!((g - 9.8).abs() < 0.1);  // ~9.8 m/s²
//!
//! // Photon energy at 500 nm: E = hc/λ
//! let wavelength = 500e-9;  // 500 nm in meters
//! let energy_j = PLANCK_CONSTANT * SPEED_OF_LIGHT / wavelength;
//! let energy_ev = energy_j / ELEMENTARY_CHARGE;
//! assert!((energy_ev - 2.48).abs() < 0.01);  // ~2.48 eV
//! ```

// =============================================================================
// Fundamental Constants (CODATA 2018)
// =============================================================================

/// Speed of light in vacuum (m/s) - exact
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Planck constant (J·s) - exact
pub const PLANCK_CONSTANT: f64 = 6.626_070_15e-34;

/// Reduced Planck constant ħ = h/(2π) (J·s)
pub const HBAR: f64 = PLANCK_CONSTANT / (2.0 * std::f64::consts::PI);

/// Elementary charge (C) - exact
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19;

/// Boltzmann constant (J/K) - exact
pub const BOLTZMANN_CONSTANT: f64 = 1.380_649e-23;

/// Avogadro constant (1/mol) - exact
pub const AVOGADRO_CONSTANT: f64 = 6.022_140_76e23;

/// Newtonian gravitational constant (m³/(kg·s²))
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

/// Vacuum magnetic permeability μ₀ (N/A²)
pub const VACUUM_PERMEABILITY: f64 = 1.256_637_062_12e-6;

/// Vacuum electric permittivity ε₀ (F/m)
pub const VACUUM_PERMITTIVITY: f64 = 8.854_187_812_8e-12;

/// Fine-structure constant α (dimensionless)
pub const FINE_STRUCTURE_CONSTANT: f64 = 7.297_352_569_3e-3;

// =============================================================================
// Derived Constants
// =============================================================================

/// Stefan-Boltzmann constant σ (W/(m²·K⁴))
pub const STEFAN_BOLTZMANN_CONSTANT: f64 = 5.670_374_419e-8;

/// Wien wavelength displacement constant (m·K)
pub const WIEN_WAVELENGTH_CONSTANT: f64 = 2.897_771_955e-3;

/// Wien frequency displacement constant (Hz/K)
pub const WIEN_FREQUENCY_CONSTANT: f64 = 5.878_925_757e10;

/// First radiation constant c₁ = 2πhc² (W·m²)
pub const FIRST_RADIATION_CONSTANT: f64 = 3.741_771_852e-16;

/// Second radiation constant c₂ = hc/k (m·K)
pub const SECOND_RADIATION_CONSTANT: f64 = 1.438_776_877e-2;

/// Rydberg constant R∞ (1/m)
pub const RYDBERG_CONSTANT: f64 = 1.097_373_156_816_0e7;

// =============================================================================
// Particle Masses
// =============================================================================

/// Electron mass (kg)
pub const ELECTRON_MASS: f64 = 9.109_383_701_5e-31;

/// Proton mass (kg)
pub const PROTON_MASS: f64 = 1.672_621_923_69e-27;

/// Neutron mass (kg)
pub const NEUTRON_MASS: f64 = 1.674_927_498_04e-27;

/// Atomic mass unit (kg) - 1/12 of ¹²C mass
pub const ATOMIC_MASS_UNIT: f64 = 1.660_539_066_60e-27;

// =============================================================================
// Atomic Units
// =============================================================================

/// Bohr radius a₀ (m)
pub const BOHR_RADIUS: f64 = 5.291_772_109_03e-11;

/// Hartree energy Eₕ (J)
pub const HARTREE_ENERGY: f64 = 4.359_744_722_207_1e-18;

// =============================================================================
// Astronomical Constants (IAU 2015 Resolution B3)
// =============================================================================

/// Astronomical unit AU (m) - exact
pub const ASTRONOMICAL_UNIT: f64 = 1.495_978_707e11;

/// Parsec (m) - derived from AU
pub const PARSEC: f64 = 3.085_677_581_491_367e16;

/// Light-year (m) - based on Julian year
pub const LIGHT_YEAR: f64 = 9.460_730_472_580_8e15;

/// Solar mass parameter GM☉ (m³/s²) - exact (IAU 2015)
pub const SOLAR_GM: f64 = 1.327_124_400_41e20;

/// Solar mass (kg) - derived from GM☉/G
pub const SOLAR_MASS: f64 = 1.988_409_870_698_051e30;

/// Nominal solar radius R☉ (m) - exact (IAU 2015)
pub const SOLAR_RADIUS: f64 = 6.957e8;

/// Nominal solar luminosity L☉ (W) - exact (IAU 2015)
pub const SOLAR_LUMINOSITY: f64 = 3.828e26;

/// Nominal solar effective temperature (K) - exact (IAU 2015)
pub const SOLAR_TEMPERATURE: f64 = 5772.0;

/// Jupiter mass (kg)
pub const JUPITER_MASS: f64 = 1.898_13e27;

/// Nominal Jupiter equatorial radius (m)
pub const JUPITER_RADIUS: f64 = 6.991_1e7;

/// Earth mass (kg)
pub const EARTH_MASS: f64 = 5.972_17e24;

/// Nominal Earth equatorial radius (m)
pub const EARTH_RADIUS: f64 = 6.378_1e6;

// =============================================================================
// Time Constants
// =============================================================================

/// Julian year (s) - exactly 365.25 days
pub const JULIAN_YEAR: f64 = 365.25 * 86400.0;

/// Tropical year (s) - approximately
pub const TROPICAL_YEAR: f64 = 365.242_19 * 86400.0;

/// Sidereal day (s)
pub const SIDEREAL_DAY: f64 = 86_164.090_5;

// =============================================================================
// Electromagnetic Constants
// =============================================================================

/// Impedance of free space Z₀ = μ₀c (Ω)
pub const IMPEDANCE_OF_FREE_SPACE: f64 = 376.730_313_668;

/// Classical electron radius rₑ (m)
pub const CLASSICAL_ELECTRON_RADIUS: f64 = 2.817_940_326_2e-15;

/// Thomson cross section (m²)
pub const THOMSON_CROSS_SECTION: f64 = 6.652_458_732_1e-29;

/// Magnetic flux quantum Φ₀ = h/(2e) (Wb)
pub const MAGNETIC_FLUX_QUANTUM: f64 = 2.067_833_848e-15;

/// Josephson constant Kⱼ = 2e/h (Hz/V)
pub const JOSEPHSON_CONSTANT: f64 = 4.835_978_484e14;

/// von Klitzing constant Rₖ = h/e² (Ω)
pub const VON_KLITZING_CONSTANT: f64 = 2.581_280_745e4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_squared() {
        // c² should be about 9e16 m²/s²
        let c_sq = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
        assert!((c_sq - 8.987_551_787e16).abs() / 8.987_551_787e16 < 1e-9);
    }

    #[test]
    fn test_hbar() {
        // ħ = h / 2π ≈ 1.054e-34 J·s
        assert!((HBAR - 1.054_571_817e-34).abs() / 1.054_571_817e-34 < 1e-6);
    }

    #[test]
    fn test_ev_in_joules() {
        // 1 eV = e × 1 V = 1.602e-19 J
        assert!((ELEMENTARY_CHARGE - 1.602_176_634e-19).abs() < 1e-28);
    }

    #[test]
    fn test_solar_mass_from_gm() {
        // M☉ = GM☉ / G
        let m_sun = SOLAR_GM / GRAVITATIONAL_CONSTANT;
        assert!((m_sun - SOLAR_MASS).abs() / SOLAR_MASS < 1e-4);
    }
}
