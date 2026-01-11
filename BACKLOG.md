# iridium-units Backlog

## Planned Features

### Unit/Quantity String Parsing

**Priority:** High (needed for iridium-astro)

Add the ability to parse unit strings and quantity strings from plain text:

#### Unit Parsing
- Parse unit symbols: `"m"`, `"km"`, `"s"`, `"kg"`
- Parse unit names: `"meter"`, `"meters"`, `"kilometer"`, `"second"`
- Parse composite units: `"m/s"`, `"km/s"`, `"kg*m/s^2"`, `"m s^-1"`
- Support common aliases: `"metre"` = `"meter"`, `"sec"` = `"second"`

#### Quantity Parsing
- Parse value + unit: `"100 km"`, `"9.8 m/s^2"`, `"1.5e8 km"`
- Handle various formats: `"100km"`, `"100 km"`, `"100.0 km"`

#### Implementation Approach
- Use `nom` parser combinators (already a dependency)
- Create a `UnitRegistry` for name/symbol lookups
- Support case-insensitive matching for names
- Allow user-defined aliases

#### Example API
```rust
use iridium_units::parse::{parse_unit, parse_quantity};

let unit = parse_unit("km/s")?;
let qty = parse_quantity("100 km")?;

// With registry for custom units
let registry = UnitRegistry::default()
    .with_alias("metres", &M);
let unit = registry.parse_unit("metres")?;
```

#### Use Case: iridium-astro Ephemeris
Ephemeris files from various sources use different unit conventions:
- CCSDS OEM files use `"km"`, `"km/s"`
- SP3 files use `"km"`, `"dm/s"`
- Some formats use `"m"`, `"m/s"`

The parser needs to handle all common astronomical unit representations.

---

### Logarithmic Units

**Priority:** Medium

Support for magnitude, dex, and decibel units as planned in the original design.

---

### Physical Type Classification

**Priority:** Low

Add `PhysicalType` enum for classifying quantities (velocity, acceleration, energy, etc.).
