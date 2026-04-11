//! Integration tests for iridium-units.
//!
//! These tests exercise end-to-end workflows that combine multiple library
//! features: parsing, conversion, equivalencies, arithmetic, and display.

use iridium_units::prelude::*;

// =============================================================================
// End-to-end calculations
// =============================================================================

#[cfg(feature = "astrophysics")]
#[test]
fn stellar_luminosity_from_flux_and_distance() {
    // Given a star's flux and distance, compute luminosity: L = 4π d² F
    let flux = 1.0e-10 * &(W / M.pow(2)); // W/m²
    let distance = 10.0 * PARSEC;

    // Convert distance to meters for multiplication
    let distance_m = distance.to(M).unwrap();

    // L = 4π d² F
    let luminosity = 4.0 * std::f64::consts::PI * &distance_m.pow(2) * &flux;
    let in_solar = luminosity.to(SOLAR_LUMINOSITY).unwrap();

    // 10 pc, 1e-10 W/m² → should be a reasonable stellar luminosity
    assert!(in_solar.value() > 0.0);
    assert!(in_solar.unit().dimension() == SOLAR_LUMINOSITY.dimension);
}

#[test]
fn kinetic_energy_calculation() {
    // E = ½mv²
    let mass = 70.0 * KG;
    let speed = 10.0 * &(M / S);
    let energy = 0.5 * &mass * &speed.pow(2);

    let in_joules = energy.to(J).unwrap();
    assert!((in_joules.value() - 3500.0).abs() < 1e-6);
}

#[test]
fn earth_surface_gravity() {
    // g = GM/r²
    use iridium_units::constants::{GRAVITATIONAL_CONSTANT, EARTH_MASS, EARTH_RADIUS};

    let surface_gravity = GRAVITATIONAL_CONSTANT * EARTH_MASS / (EARTH_RADIUS * EARTH_RADIUS);
    // Should be ~9.8 m/s²
    assert!((surface_gravity - 9.8).abs() < 0.1);
}

#[test]
fn pressure_from_force_and_area() {
    let force = 100.0 * N;
    let area = 2.0 * &M.pow(2);
    let pressure = &force / &area;

    let in_pascal = pressure.to(PA).unwrap();
    assert!((in_pascal.value() - 50.0).abs() < 1e-10);
}

// =============================================================================
// Chained equivalency conversions
// =============================================================================

#[cfg(feature = "astrophysics")]
#[test]
fn spectral_wavelength_to_frequency_to_energy() {
    use iridium_units::equivalencies::spectral;
    use iridium_units::systems::si::EV;

    // 500 nm → frequency → energy
    let wavelength = 500.0 * NM;
    let freq = wavelength.to_equiv(HZ, spectral()).unwrap();
    let energy = freq.to_equiv(EV, spectral()).unwrap();

    // 500 nm photon ≈ 2.48 eV
    assert!((energy.value() - 2.48).abs() < 0.01);
}

#[cfg(feature = "astrophysics")]
#[test]
fn parallax_to_distance_roundtrip() {
    use iridium_units::equivalencies::parallax;
    use iridium_units::systems::si::ARCSEC;

    let plx = 0.1 * ARCSEC; // 100 mas
    let dist = plx.to_equiv(PARSEC, parallax()).unwrap();
    assert!((dist.value() - 10.0).abs() < 1e-10);

    // Roundtrip
    let plx_back = dist.to_equiv(ARCSEC, parallax()).unwrap();
    assert!((plx_back.value() - 0.1).abs() < 1e-10);
}

#[test]
fn mass_energy_equivalence() {
    use iridium_units::equivalencies::mass_energy;

    let mass = 1.0 * KG;
    let energy = mass.to_equiv(J, mass_energy()).unwrap();

    // E = mc² ≈ 8.99e16 J
    let c = iridium_units::constants::SPEED_OF_LIGHT;
    assert!((energy.value() - c * c).abs() / (c * c) < 1e-9);
}

// =============================================================================
// Temperature conversions
// =============================================================================

#[test]
fn boiling_point_across_scales() {
    let boiling_c = 100.0 * DEG_C;
    let boiling_f = boiling_c.to(DEG_F).unwrap();
    let boiling_k = boiling_c.to(K).unwrap();

    assert!((boiling_f.value() - 212.0).abs() < 1e-6);
    assert!((boiling_k.value() - 373.15).abs() < 1e-6);
}

#[test]
fn absolute_zero_across_scales() {
    let abs_zero_k = 0.0 * K;
    let abs_zero_c = abs_zero_k.to(DEG_C).unwrap();
    let abs_zero_f = abs_zero_k.to(DEG_F).unwrap();

    assert!((abs_zero_c.value() - (-273.15)).abs() < 1e-6);
    assert!((abs_zero_f.value() - (-459.67)).abs() < 1e-6);
}

#[test]
fn temperature_arithmetic() {
    // Adding temperatures converts rhs to lhs unit first
    // 100°C + 50°C: rhs 50°C stays as 50°C, result is 150°C
    let temp1 = 100.0 * DEG_C;
    let temp2 = 50.0 * DEG_C;
    let result = &temp1 + &temp2;

    assert!((result.value() - 150.0).abs() < 1e-10);
}

// =============================================================================
// Parsing → conversion → display roundtrips
// =============================================================================

#[test]
fn parse_and_convert_quantity() {
    let dist: Quantity = "100 km".parse().unwrap();
    let in_miles = dist.to(MILE).unwrap();

    assert!((in_miles.value() - 62.1371).abs() < 0.001);
}

#[test]
fn parse_compound_unit() {
    let accel_unit = parse_unit("m/s^2").unwrap();
    let accel = 9.8 * &accel_unit;
    let expected_dim = (M / S.pow(2)).dimension();

    assert_eq!(accel.unit().dimension(), expected_dim);
    assert!((accel.value() - 9.8).abs() < 1e-10);
}

#[test]
fn parse_unit_and_convert() {
    let km = parse_unit("km").unwrap();
    let mi = parse_unit("mi").unwrap();

    let dist = 100.0 * &km;
    let in_miles = dist.to(&mi).unwrap();
    assert!((in_miles.value() - 62.1371).abs() < 0.001);
}

#[cfg(feature = "astrophysics")]
#[test]
fn parse_astrophysical_units() {
    let pc = parse_unit("pc").unwrap();
    let au = parse_unit("AU").unwrap();
    let ly = parse_unit("ly").unwrap();

    // 1 pc in AU
    let one_pc = 1.0 * &pc;
    let in_au = one_pc.to(&au).unwrap();
    assert!((in_au.value() - 206265.0).abs() / 206265.0 < 0.001);

    // 1 pc in light-years
    let in_ly = one_pc.to(&ly).unwrap();
    assert!((in_ly.value() - 3.2616).abs() < 0.001);
}

#[test]
fn display_format() {
    let speed = 100.0 * &(KM / H);
    let displayed = format!("{}", speed);

    assert!(displayed.contains("100"));
    assert!(displayed.contains("km"));
}

// =============================================================================
// Dimensional analysis error cases
// =============================================================================

#[test]
fn add_incompatible_dimensions_fails() {
    let length = 10.0 * M;
    let time = 5.0 * S;

    let result = length.checked_add(&time);
    assert!(result.is_err());
}

#[test]
fn convert_incompatible_dimensions_fails() {
    let mass = 1.0 * KG;
    let result = mass.to(M);
    assert!(result.is_err());
}

#[test]
fn add_compatible_dimensions_succeeds() {
    let a = 100.0 * M;
    let b = 0.5 * KM;
    let sum = a.checked_add(&b).unwrap();

    // Result in first operand's unit (m)
    assert!((sum.value() - 600.0).abs() < 1e-10);
}

// =============================================================================
// Unit system cross-conversions
// =============================================================================

#[test]
fn si_to_imperial() {
    let marathon = 42.195 * KM;
    let in_miles = marathon.to(MILE).unwrap();
    assert!((in_miles.value() - 26.2188).abs() < 0.001);

    let in_feet = marathon.to(FOOT).unwrap();
    assert!((in_feet.value() - 138_435.0).abs() < 10.0);
}

#[cfg(feature = "cgs")]
#[test]
fn si_to_cgs() {
    use iridium_units::systems::cgs::{DYNE, ERG};

    let force = 1.0 * N;
    let in_dyne = force.to(DYNE).unwrap();
    assert!((in_dyne.value() - 1e5).abs() < 1e-5);

    let energy = 1.0 * J;
    let in_erg = energy.to(ERG).unwrap();
    assert!((in_erg.value() - 1e7).abs() < 1e-3);
}

// =============================================================================
// Batch conversion
// =============================================================================

#[test]
fn batch_convert_wavelengths() {
    let wavelengths_nm: Vec<f64> = vec![400.0, 500.0, 600.0, 700.0];
    let results = iridium_units::quantity::batch_convert(&wavelengths_nm, NM, UM).unwrap();

    assert_eq!(results.len(), 4);
    assert!((results[0] - 0.4).abs() < 1e-10);
    assert!((results[1] - 0.5).abs() < 1e-10);
    assert!((results[2] - 0.6).abs() < 1e-10);
    assert!((results[3] - 0.7).abs() < 1e-10);
}

#[test]
fn batch_convert_preserves_precision() {
    let values: Vec<f64> = (1..=1000).map(|i| i as f64).collect();
    let results = iridium_units::quantity::batch_convert(&values, KM, M).unwrap();

    for (i, &val) in results.iter().enumerate() {
        let expected = (i + 1) as f64 * 1000.0;
        assert!((val - expected).abs() < 1e-6);
    }
}

// =============================================================================
// Custom unit registry
// =============================================================================

#[test]
fn custom_registry_roundtrip() {
    let furlong = Unit::Base(iridium_units::unit::base::BaseUnit::new(
        "furlong", "fur", &[], iridium_units::dimension::Dimension::LENGTH, 201.168,
    ));

    let registry = UnitRegistry::with_builtins()
        .with_unit(&["furlong", "fur"], furlong);

    let parsed = registry.parse_unit("furlong").unwrap();
    let dist = 8.0 * &parsed;
    let in_miles = dist.to(MILE).unwrap();

    // 8 furlongs = 1 mile
    assert!((in_miles.value() - 1.0).abs() < 0.001);
}

// =============================================================================
// Logarithmic units
// =============================================================================

#[cfg(feature = "logarithmic")]
#[test]
fn magnitude_flux_roundtrip() {
    use iridium_units::systems::logarithmic::MAG;
    use iridium_units::equivalencies::logarithmic::magnitude_flux;

    let mag5 = 5.0 * MAG;
    let flux = mag5.to_equiv(&Unit::dimensionless(), magnitude_flux()).unwrap();
    assert!((flux.value() - 0.01).abs() < 1e-10);

    let back = flux.to_equiv(MAG, magnitude_flux()).unwrap();
    assert!((back.value() - 5.0).abs() < 1e-10);
}

#[cfg(feature = "logarithmic")]
#[test]
fn db_power_3db_rule() {
    use iridium_units::systems::logarithmic::DB;
    use iridium_units::equivalencies::logarithmic::db_power;

    // 3 dB ≈ 2x power, 10 dB = 10x power, 20 dB = 100x power
    for (db_val, expected_ratio) in [(3.0, 2.0), (10.0, 10.0), (20.0, 100.0)] {
        let db = db_val * DB;
        let ratio = db.to_equiv(&Unit::dimensionless(), db_power()).unwrap();
        assert!((ratio.value() - expected_ratio).abs() / expected_ratio < 0.01);
    }
}

// =============================================================================
// Fractional exponents
// =============================================================================

#[test]
fn square_root_of_area() {
    let area = 100.0 * &M.pow(2);
    let side = area.pow(Rational16::new(1, 2));

    assert!((side.value() - 10.0).abs() < 1e-10);
    assert_eq!(side.unit().dimension(), M.dimension);
}

#[test]
fn cube_root_of_volume() {
    let volume = 27.0 * &M.pow(3);
    let side = volume.pow(Rational16::new(1, 3));

    assert!((side.value() - 3.0).abs() < 1e-10);
    assert_eq!(side.unit().dimension(), M.dimension);
}
