# Equivalencies Guide

Equivalencies enable conversions between units with different dimensions when there is a physical relationship between them. Unlike regular unit conversion (meters to kilometers), equivalencies bridge fundamentally different physical quantities (wavelength to frequency).

## Using Equivalencies

```rust
use iridium_units::prelude::*;
use iridium_units::equivalencies::spectral;

let wavelength = 500.0 * NM;
let frequency = wavelength.to_equiv(&HZ, spectral())?;
```

## Available Equivalencies

### Spectral (`spectral`)

Converts between wavelength, frequency, energy, and wavenumber using:
- c = λν (speed of light)
- E = hν (photon energy)
- k = 1/λ (wavenumber)

```rust
use iridium_units::equivalencies::spectral;

// Wavelength ↔ Frequency
let wavelength = 500.0 * NM;
let freq = wavelength.to_equiv(&HZ, spectral())?;  // ~6×10¹⁴ Hz

// Wavelength ↔ Energy
let energy = wavelength.to_equiv(&EV, spectral())?;  // ~2.5 eV

// Energy ↔ Frequency
let photon_energy = 1.0 * EV;
let freq = photon_energy.to_equiv(&HZ, spectral())?;
```

---

### Spectral Density (`spectral_density`, `ab_magnitude`)

Converts between spectral flux density representations and magnitudes.

#### Fλ ↔ Fν Conversion

Converts flux per wavelength to flux per frequency (and vice versa).

**Physics:** Fλ = Fν × c/λ² (energy conservation: Fλ dλ = Fν dν)

```rust
use iridium_units::equivalencies::spectral_density;
use iridium_units::systems::astrophysical::{JANSKY, FLAM};

// Requires spectral coordinate (wavelength or frequency)
let wavelength = 500.0 * NM;

// Convert 1 Jansky to erg/s/cm²/Å at 500 nm
let f_nu = 1.0 * JANSKY;
let f_lambda = f_nu.to_equiv(&FLAM, spectral_density(wavelength))?;
```

#### AB Magnitude System

Converts between flux density and AB magnitudes.

**Physics:** m_AB = -2.5 × log₁₀(Fν / 3631 Jy)

```rust
use iridium_units::equivalencies::ab_magnitude;
use iridium_units::systems::logarithmic::MAG;

// Flux to AB magnitude
let flux = 1.0 * JANSKY;
let mag = flux.to_equiv(&MAG, ab_magnitude())?;  // ~8.9 mag

// AB magnitude to flux
let mag = 20.0 * MAG;
let flux = mag.to_equiv(&JANSKY, ab_magnitude())?;  // ~3.6×10⁻⁵ Jy
```

**Convenience functions:**
```rust
use iridium_units::equivalencies::spectral_density::{ab_mag_to_jansky, jansky_to_ab_mag};

let flux_jy = ab_mag_to_jansky(20.0);  // 3.631×10⁻⁵
let mag = jansky_to_ab_mag(1.0);       // ~8.9
```

---

### Doppler (`doppler_radio`, `doppler_optical`, `doppler_relativistic`)

Converts between frequency shift and velocity using different conventions.

#### Radio Convention
v/c = (ν₀ - ν) / ν₀

Used in radio astronomy. Can give v > c for large redshifts.

```rust
use iridium_units::equivalencies::doppler_radio;

let rest_freq = 1.420405751768e9 * HZ;  // 21 cm line
let observed = 1.420e9 * HZ;

// Get velocity from frequency shift
let velocity = observed.to_equiv(&(M / S), doppler_radio(rest_freq))?;
```

#### Optical Convention
v/c = (λ - λ₀) / λ₀ = (ν₀ - ν) / ν

Used in optical astronomy for redshift z.

```rust
use iridium_units::equivalencies::doppler_optical;

let rest_freq = 5e14 * HZ;
let velocity = observed.to_equiv(&(M / S), doppler_optical(rest_freq))?;
```

#### Relativistic Convention
v/c = (ν₀² - ν²) / (ν₀² + ν²)

Physically correct for all velocities. Never exceeds c.

```rust
use iridium_units::equivalencies::doppler_relativistic;

let rest_freq = 1e9 * HZ;
let velocity = observed.to_equiv(&(M / S), doppler_relativistic(rest_freq))?;
```

---

### Parallax (`parallax`)

Converts between parallax angle and distance.

**Physics:** d = 1 AU / tan(p) ≈ 1/p for small angles

```rust
use iridium_units::equivalencies::parallax;
use iridium_units::systems::si::ARCSEC;

// Proxima Centauri: parallax = 0.7687 arcsec
let p = 0.7687 * ARCSEC;
let distance = p.to_equiv(&PARSEC, parallax())?;  // ~1.30 pc
```

---

### Mass-Energy (`mass_energy`)

Converts between mass and energy using E = mc².

```rust
use iridium_units::equivalencies::mass_energy;

// Mass to energy
let mass = 1.0 * KG;
let energy = mass.to_equiv(&J, mass_energy())?;  // ~9×10¹⁶ J

// Electron rest mass energy
let m_e = 9.109e-31 * KG;
let E = m_e.to_equiv(&MEV, mass_energy())?;  // ~0.511 MeV
```

---

### Temperature (`temperature`, `temperature_energy`)

#### Temperature Scales
Converts between Kelvin and other temperature scales (Celsius, Fahrenheit, Rankine).

```rust
use iridium_units::equivalencies::temperature;
use iridium_units::systems::si::{K, DEG_C};

let t_k = 300.0 * K;
let t_c = t_k.to_equiv(DEG_C, temperature())?;  // 26.85 °C
```

#### Thermal Energy
Converts between temperature and energy using E = kT.

```rust
use iridium_units::equivalencies::temperature_energy;

// Room temperature in eV
let T = 300.0 * K;
let E = T.to_equiv(&EV, temperature_energy())?;  // ~0.026 eV

// 1 eV corresponds to what temperature?
let E = 1.0 * EV;
let T = E.to_equiv(&K, temperature_energy())?;  // ~11600 K
```

---

### Brightness Temperature (`brightness_temperature`, `brightness_temperature_planck`)

Converts between flux density and brightness temperature for radio astronomy.

#### Rayleigh-Jeans Approximation
T_b = S_ν × c² / (2k × ν² × Ω)

Valid when hν << kT (radio frequencies, high temperatures).

```rust
use iridium_units::equivalencies::brightness_temperature;

let freq = 1.420405751768e9 * HZ;  // 21 cm
let beam = 1e-6 * SR;              // 1 µsr beam

let flux = 1.0 * JANSKY;
let T_b = flux.to_equiv(&K, brightness_temperature(freq, beam))?;
```

#### Full Planck Function
Accurate at all temperatures, including where Rayleigh-Jeans breaks down.

```rust
use iridium_units::equivalencies::brightness_temperature_planck;

let T_b = flux.to_equiv(&K, brightness_temperature_planck(freq, beam))?;
```

---

### Logarithmic Units (`magnitude_flux`, `db_power`, `db_amplitude`, `dex_ratio`)

Converts between logarithmic and linear scales.

#### Stellar Magnitudes
m = -2.5 × log₁₀(F/F₀)

```rust
use iridium_units::equivalencies::magnitude_flux;

// Magnitude to flux ratio
let mag = 5.0 * MAG;
let flux_ratio = mag.to_equiv(&Unit::dimensionless(), magnitude_flux())?;
// 5 mag = 0.01 (100× fainter than reference)
```

#### Decibels (Power)
dB = 10 × log₁₀(P/P₀)

```rust
use iridium_units::equivalencies::db_power;

let signal = 10.0 * DB;
let power_ratio = signal.to_equiv(&Unit::dimensionless(), db_power())?;
// 10 dB = 10× power
```

#### Dex (Orders of Magnitude)
dex = log₁₀(x/x₀)

```rust
use iridium_units::equivalencies::dex_ratio;

let order = 2.0 * DEX;
let ratio = order.to_equiv(&Unit::dimensionless(), dex_ratio())?;
// 2 dex = 100×
```

---

### Dimensionless Angles (`dimensionless_angles`)

Treats angles (radians, steradians) as dimensionless for physics equations where this is mathematically required.

**The Problem:**
```rust
// Rotational energy E = ½Iω²
let I = 2.0 * KG * M * M;   // kg·m²
let omega = 3.0 * RAD / S;    // rad/s

let E = 0.5 * &I * &omega * &omega;
// Dimension: kg·m²·rad²/s² (has spurious rad²)
// Should be: kg·m²/s² (Joules)
```

**The Solution:**
```rust
use iridium_units::equivalencies::dimensionless_angles;

let E = (0.5 * &I * &omega * &omega)
    .to_equiv(&J, dimensionless_angles())?;
// Now correctly in Joules
```

**Use cases:**
- Rotational mechanics: E = ½Iω², τ = Iα, L = Iω
- Angular frequency: ω = 2πf (rad/s ↔ Hz)
- Work from torque: W = τθ
- Radiant intensity: I × Ω = P (W/sr × sr = W)

```rust
// Angular frequency conversion
let omega = 2.0 * PI * RAD / S;  // 2π rad/s
let f = omega.to_equiv(&HZ, dimensionless_angles())?;  // 2π Hz

// Pure angle to dimensionless
let theta = PI * RAD;
let value = theta.to_equiv(&Unit::dimensionless(), dimensionless_angles())?;
// value = π
```

**Helper functions:**
```rust
use iridium_units::equivalencies::dimensionless_angles::{has_angle_dimension, angle_power};

// Check if a unit contains angle dimensions
assert!(has_angle_dimension(&(RAD / S)));  // true
assert!(!has_angle_dimension(&Unit::from(HZ)));  // false

// Get total angle power
let omega_sq = (RAD / S).pow(2);
assert_eq!(angle_power(&omega_sq), 2);  // rad²
```

---

## Multiple Equivalencies

You can try multiple equivalencies in sequence:

```rust
use iridium_units::prelude::*;
use iridium_units::equivalencies::{spectral, mass_energy};

let quantity = /* some quantity */;
let result = quantity.to_equiv_list(&target_unit, &[spectral(), mass_energy()])?;
```

The first matching equivalency will be used.

## Creating Custom Equivalencies

```rust
use iridium_units::equivalencies::{Equivalency, Converter};

fn my_equivalency() -> Equivalency {
    Equivalency::new("my_equivalency", |from, to| {
        // Check if this equivalency applies
        if /* conditions */ {
            Some(Converter::new(
                |x| Ok(/* forward conversion */),
                |x| Ok(/* backward conversion */),
            ))
        } else {
            None
        }
    })
}
```
