# Getting Started with iridium-units

iridium-units is a Rust library for units of measure with runtime dimensional
analysis. A unit is a value you parse, store, and pass around — not a type you
fix at compile time — which suits units that aren't known until you run.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
iridium-units = "0.2"
```

## Basic Usage

### Creating Quantities

Quantities are created by multiplying a number by a unit:

```rust
use iridium_units::prelude::*;

// Basic quantities
let distance = 100.0 * M;        // 100 meters
let time = 9.58 * S;             // 9.58 seconds
let mass = 70.0 * KG;            // 70 kilograms

// Derived quantities through arithmetic
let speed = &distance / &time;     // ~10.4 m/s
let energy = 0.5 * &mass * &speed * &speed;  // kinetic energy
```

### Unit Conversion

Convert between compatible units using `.to()`:

```rust
use iridium_units::prelude::*;

let distance = 5.0 * KM;
let in_meters = distance.to(M)?;      // 5000 m
let in_miles = distance.to(MILE)?;     // ~3.1 miles

// Get just the numeric value
let meters_value = distance.to_value(M)?;  // 5000.0
# Ok::<(), iridium_units::error::UnitError>(())
```

### Dimensional Analysis

The library automatically tracks dimensions and prevents incompatible operations:

```rust
use iridium_units::prelude::*;

let distance = 100.0 * M;
let time = 10.0 * S;

// This works — creates velocity
let velocity = &distance / &time;

// This fails at runtime — can't add meters to seconds
let result = distance.checked_add(&time);
assert!(result.is_err());
```

### Parsing Units from Strings

Parse unit strings with flexible syntax:

```rust
use iridium_units::prelude::*;

// Simple units
let meter = parse_unit("m")?;
let kilometer = parse_unit("km")?;

// Composite units
let velocity = parse_unit("m/s")?;
let acceleration = parse_unit("m/s^2")?;
let force = parse_unit("kg*m/s^2")?;

// Unicode support
let area = parse_unit("m²")?;
let micro = parse_unit("µm")?;

// Natural language
let speed = parse_unit("km per hour")?;

// LaTeX notation
let energy = parse_unit("kg m^{2} s^{-2}")?;

// Parse quantities (value + unit)
let distance = parse_quantity("100 km")?;
let wavelength = parse_quantity("500 nm")?;
# Ok::<(), iridium_units::error::UnitError>(())
```

## Unit Systems

iridium-units provides several unit systems:

### SI Units (`systems::si`)

```rust
use iridium_units::systems::si::*;

// Base units
let length = 1.0 * M;      // meter
let time = 1.0 * S;        // second
let mass = 1.0 * KG;       // kilogram
let current = 1.0 * A;     // ampere
let temp = 1.0 * K;        // kelvin

// Derived units
let force = 1.0 * N;       // newton
let energy = 1.0 * J;      // joule
let power = 1.0 * W;       // watt
let frequency = 1.0 * HZ;  // hertz
let pressure = 1.0 * PA;   // pascal

// Prefixed units
let km = 1.0 * KM;         // kilometer
let nm = 1.0 * NM;         // nanometer
let ghz = 1.0 * GHZ;       // gigahertz
```

### Imperial Units (`systems::imperial`)

```rust
use iridium_units::systems::imperial::*;

let distance = 1.0 * MILE;    // mile
let height = 6.0 * FOOT;      // foot
let weight = 180.0 * POUND;   // pound
let volume = 1.0 * GALLON;    // gallon
```

### CGS Units (`systems::cgs`)

```rust
use iridium_units::systems::cgs::*;

let energy = 1.0 * ERG;      // erg
let force = 1.0 * DYNE;      // dyne
let field = 1.0 * GAUSS;     // gauss
```

### Logarithmic Units (`systems::logarithmic`)

```rust
use iridium_units::systems::logarithmic::*;

let mag = 5.0 * MAG;                // magnitude
let signal = 10.0 * DB;             // decibel
let order = 2.0 * DEX;              // dex (order of magnitude)
```

### Astrophysical Units (`systems::astrophysical`)

Behind the `astrophysics` feature flag:

```rust
use iridium_units::systems::astrophysical::*;

let d1 = 1.0 * PARSEC;       // parsec
let d2 = 1.0 * AU;           // astronomical unit
let d3 = 1.0 * LIGHT_YEAR;   // light year
```

## Equivalencies

Equivalencies enable conversions between different physical dimensions when
there's a physical relationship (like wavelength to frequency). See
[equivalencies.md](equivalencies.md) for full documentation.

```rust
use iridium_units::prelude::*;
use iridium_units::equivalencies::spectral;

// Convert wavelength to frequency
let wavelength = 500.0 * NM;
let frequency = wavelength.to_equiv(HZ, spectral())?;
# Ok::<(), iridium_units::error::UnitError>(())
```

## Physical Constants

Access CODATA 2018 physical constants:

```rust
use iridium_units::constants::*;

let c = SPEED_OF_LIGHT;           // 299792458 m/s
let h = PLANCK_CONSTANT;          // 6.62607015e-34 J·s
let k = BOLTZMANN_CONSTANT;       // 1.380649e-23 J/K
let G = GRAVITATIONAL_CONSTANT;   // 6.67430e-11 m³/(kg·s²)
```

## Error Handling

All fallible operations return `UnitResult<T>`:

```rust
use iridium_units::prelude::*;
use iridium_units::error::UnitError;

let mass = 1.0 * KG;
match mass.to(M) {
    Ok(converted) => println!("Converted: {}", converted),
    Err(UnitError::DimensionMismatch { from, to }) => {
        println!("Cannot convert {} to {}", from, to);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Batch Conversion

When converting many values of the same unit, the batch API computes the
conversion factor once and applies it across the slice, rather than
recomputing it per value:

```rust
use iridium_units::prelude::*;
use iridium_units::quantity::{batch_convert, conversion_factor};

let values_km: Vec<f64> = (0..10000).map(|i| i as f64).collect();
let values_m = batch_convert(&values_km, KM, M)?;

// Or get the factor for manual/SIMD operations
let factor = conversion_factor(KM, M)?;
# Ok::<(), iridium_units::error::UnitError>(())
```

## Cookbook

Practical recipes for common tasks.

### Speed, Distance, and Time

```rust
use iridium_units::prelude::*;

// Marathon pace: 42.195 km in 3:30:00
let distance = 42.195 * KM;
let time = 3.5 * H;
let pace = &distance / &time;

let pace_mph = pace.to(MILE / H).unwrap();
println!("Pace: {}", pace_mph);  // ~7.5 mi/h

let pace_ms = pace.to(M / S).unwrap();
println!("Pace: {}", pace_ms);  // ~3.35 m/s
```

### Temperature Conversion

```rust
use iridium_units::prelude::*;

// Boiling point of water
let boiling = 100.0 * DEG_C;
let in_fahrenheit = boiling.to(DEG_F).unwrap();
let in_kelvin = boiling.to(K).unwrap();

assert!((in_fahrenheit.value() - 212.0).abs() < 1e-6);
assert!((in_kelvin.value() - 373.15).abs() < 1e-6);
```

### Force and Pressure

```rust
use iridium_units::prelude::*;

// F = ma
let mass = 75.0 * KG;
let accel = 9.8 * &(M / S.pow(2));
let force = &mass * &accel;

let in_newtons = force.to(N).unwrap();
assert!((in_newtons.value() - 735.0).abs() < 0.1);

// Pressure = Force / Area
let area = 0.5 * &M.pow(2);
let pressure = &force / &area;
let in_pascal = pressure.to(PA).unwrap();
assert!((in_pascal.value() - 1470.0).abs() < 1.0);
```

### Parsing User Input

```rust
use iridium_units::prelude::*;

// Accept units from user input, config files, etc.
let input = "100 km/h";
let speed = parse_quantity(input).unwrap();
let in_ms = speed.to(M / S).unwrap();
println!("{} = {}", input, in_ms);
```

### Custom Units

```rust
use iridium_units::prelude::*;

// Define a custom unit
let furlong = Unit::Base(BaseUnit::new(
    "furlong", "fur", &[],
    Dimension::LENGTH,
    201.168  // meters per furlong
));

// Use it for conversion
let marathon = 42.195 * KM;
let in_furlongs = marathon.to(furlong).unwrap();
println!("{} furlongs", in_furlongs.value());  // ~209.7
```

## Next Steps

- [Equivalencies Guide](equivalencies.md) — Converting between different physical domains
- [Unit Systems Reference](unit-systems.md) — Complete unit tables
- [Advanced Topics](advanced.md) — Custom registries, batch operations, and more
