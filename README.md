# Jurisdiction

Lightweight static `Jurisdiction` information.

This crate provides interfaces to work with jurisdictions for areas around the world.
Information about a jurisdiction includes
* ISO 3166 Alpha2 and Alpha3 character codes.
* ISO 3166 numeric country code.
* UN M49 region classifications.

The Jurisdiction object is a lightweight object, the size of a pointer,
suitable for transfer in API surfaces throughout an ecosystem. Serialization on
API boundaries may choose to employ any of the standardized classification formats.

## Example

```rust
use anyhow::{Result, anyhow, format_err};
use jurisdiction::{Jurisdiction, Alpha2, Alpha3};
use jurisdiction::region::{Region, SubRegion};
use std::str::FromStr;

fn supported_jurisdiction(alpha: &str) -> Result<Jurisdiction> {
    let jurisdiction = Jurisdiction::from_str(alpha)?;
    match jurisdiction.alpha2() {
        Alpha2::NO | Alpha2::SE | Alpha2::DK => Ok(jurisdiction),
        _ => Err(format_err!("only scandinavian countries are supported")),
    }
}

fn main() {
    let jurisdiction = supported_jurisdiction("NO").expect("unsupported");

    assert_eq!(jurisdiction, Alpha2::NO);
    assert_eq!(jurisdiction.alpha2(), Alpha2::NO);
    assert_eq!(jurisdiction.alpha2().to_string(), "NO");

    assert_eq!(jurisdiction, Alpha3::NOR);
    assert_eq!(jurisdiction.alpha3(), Alpha3::NOR);
    assert_eq!(jurisdiction.alpha3().to_string(), "NOR");

    assert_eq!(jurisdiction.country_code(), 578);

    assert_eq!(jurisdiction.region(), Region::Europe);
    assert_eq!(jurisdiction.sub_region(), SubRegion::NorthernEurope);
}
```

See [docs.rs](http://docs.rs/jurisdiction) for more extensive API documentation and examples.
