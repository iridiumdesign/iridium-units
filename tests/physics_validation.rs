//! Independent physics reference checks for the public conversion APIs.

use iridium_units::prelude::*;

fn assert_relative(actual: f64, expected: f64, relative_tolerance: f64) {
    let scale = expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= relative_tolerance * scale,
        "expected {expected}, got {actual}"
    );
}

#[cfg(feature = "astrophysics")]
#[test]
fn visible_wavelength_matches_frequency() {
    use iridium_units::equivalencies::spectral;

    let frequency = (500.0 * NM).to_equiv(THZ, spectral()).unwrap();

    assert_relative(frequency.value(), 599.584916, 1e-9);
}

#[cfg(feature = "astrophysics")]
#[test]
fn hydrogen_21_centimeter_line_matches_reference_frequency() {
    use iridium_units::equivalencies::spectral;

    // The commonly quoted 21 cm line has a rest frequency of 1.420405752 GHz.
    let frequency = (0.2110611405 * M).to_equiv(GHZ, spectral()).unwrap();

    assert_relative(frequency.value(), 1.420405752, 1e-9);
}

#[cfg(feature = "astrophysics")]
#[test]
fn one_arcsecond_of_parallax_is_one_parsec() {
    use iridium_units::equivalencies::parallax;
    use iridium_units::systems::si::ARCSEC;

    let distance = (1.0 * ARCSEC).to_equiv(PARSEC, parallax()).unwrap();

    assert_relative(distance.value(), 1.0, 1e-12);
}

#[test]
fn one_kilogram_matches_mass_energy_reference() {
    use iridium_units::constants::SPEED_OF_LIGHT;
    use iridium_units::equivalencies::mass_energy;

    let energy = (1.0 * KG).to_equiv(J, mass_energy()).unwrap();

    assert_relative(energy.value(), SPEED_OF_LIGHT * SPEED_OF_LIGHT, 1e-12);
}

#[test]
fn one_electronvolt_matches_temperature_reference() {
    use iridium_units::equivalencies::temperature_energy;
    use iridium_units::systems::si::EV;

    let temperature = (1.0 * EV).to_equiv(K, temperature_energy()).unwrap();

    assert_relative(temperature.value(), 11_604.518_121_550, 1e-12);
}

#[cfg(feature = "astrophysics")]
#[test]
fn one_jansky_matches_si_spectral_flux_density() {
    use iridium_units::systems::astrophysical::JANSKY;

    let si_unit = W / M.pow(2) / HZ;
    let flux = (1.0 * JANSKY).to(&si_unit).unwrap();

    assert_relative(flux.value(), 1e-26, 1e-12);
}

#[cfg(feature = "astrophysics")]
#[test]
fn doppler_conventions_match_their_reference_formulas() {
    use iridium_units::constants::SPEED_OF_LIGHT;
    use iridium_units::equivalencies::{doppler_optical, doppler_radio, doppler_relativistic};

    let rest_frequency = 1.0 * GHZ;
    let velocity = 300.0 * &(KM / S);
    let beta = 300_000.0 / SPEED_OF_LIGHT;

    let radio_frequency = velocity
        .to_equiv(GHZ, doppler_radio(rest_frequency.clone()))
        .unwrap();
    let optical_frequency = velocity
        .to_equiv(GHZ, doppler_optical(rest_frequency.clone()))
        .unwrap();
    let relativistic_frequency = velocity
        .to_equiv(GHZ, doppler_relativistic(rest_frequency.clone()))
        .unwrap();

    assert_relative(radio_frequency.value(), 1.0 - beta, 1e-12);
    assert_relative(optical_frequency.value(), 1.0 / (1.0 + beta), 1e-12);
    assert_relative(
        relativistic_frequency.value(),
        ((1.0 - beta) / (1.0 + beta)).sqrt(),
        1e-12,
    );
}
