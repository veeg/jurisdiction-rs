#![deny(intra_doc_link_resolution_failure)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(rust_2018_idioms)]

//! Lightweight static `Jurisdiction` information.
//!
//! This crate provides interfaces to work with jurisdictions for areas around the world.
//! Information about a jurisdiction includes
//! * ISO 3166 [Alpha2] and [Alpha3] character codes.
//! * ISO 3166 numeric country code.
//! * [UN M49] region classifications.
//!
//! The [Jurisdiction] object is a lightweight object, the size of a pointer,
//! suitable for transfer in API surfaces throughout an ecosystem. Serialization on
//! API boundaries may choose to employ any of the standardized classification formats.
//!
//! # Examples
//!
//! Retrieve jurisdiction from API boundary and validate supported jurisdictions
//! ```rust
//! use anyhow::{Result, anyhow, format_err};
//! use jurisdiction::{Jurisdiction, Alpha2, Alpha3};
//! use jurisdiction::region::{Region, SubRegion};
//! use std::str::FromStr;
//!
//! fn supported_jurisdiction(alpha: &str) -> Result<Jurisdiction> {
//!     let jurisdiction = Jurisdiction::from_str(alpha)?;
//!     match jurisdiction.alpha2() {
//!         Alpha2::NO | Alpha2::SE | Alpha2::DK => Ok(jurisdiction),
//!         _ => Err(format_err!("only scandinavian countries are supported")),
//!     }
//! }
//!
//! fn main() {
//!     let jurisdiction = supported_jurisdiction("NO").expect("unsupported");
//!
//!     assert_eq!(jurisdiction, Alpha2::NO);
//!     assert_eq!(jurisdiction.alpha2(), Alpha2::NO);
//!     assert_eq!(jurisdiction.alpha2().to_string(), "NO");
//!
//!     assert_eq!(jurisdiction, Alpha3::NOR);
//!     assert_eq!(jurisdiction.alpha3(), Alpha3::NOR);
//!     assert_eq!(jurisdiction.alpha3().to_string(), "NOR");
//!
//!     assert_eq!(jurisdiction.country_code(), 578);
//!
//!     assert_eq!(jurisdiction.region(), Region::Europe);
//!     assert_eq!(jurisdiction.sub_region(), SubRegion::NorthernEurope);
//! }
//! ```
//!
//! Construct `Jurisdiction` from string:
//! ```rust
//! # use jurisdiction::{Jurisdiction, Alpha2};
//! # use std::str::FromStr;
//! let jurisdiction = Jurisdiction::from_str("NO");
//! assert!(jurisdiction.is_ok());
//! assert_eq!(jurisdiction.unwrap(), Alpha2::NO);
//! ```
//!
//! Construct `Jurisdiction` from `Alpha2`/`Alpha3`:
//! ```rust
//! # use jurisdiction::{Jurisdiction, Alpha2, Alpha3};
//! # use std::str::FromStr;
//! let jurisdiction = Jurisdiction::from(Alpha2::NO);
//! assert_eq!(jurisdiction, Alpha2::NO);
//!
//! let jurisdiction = Jurisdiction::from(Alpha3::NOR);
//! assert_eq!(jurisdiction, Alpha3::NOR);
//! ```

//!
//! # Static jurisdiction information
//!
//! All the static information about a jurisdiction is embedded into the application binary
//! through a `lazy_static` hashmap declaration, populated on first use from const definitions.
//! This way, the only copy of the definition should reside in the hashmap.
//!
//! This map is not publicly exported from the crate, only accessible through `Jurisdiction`.
//! A `Jurisdiction` object simply contains the reference to the definition within this hashmap,
//! making all look-up operations a simple pointer dereference into the statically
//! stored item in this global hashmap.
//!
//!
//! # Features
//! This crate has the following features:
//!
//! * `region`: Include the [region] module with region definitions and `Jurisdiction` array
//! methods returning the zoning jurisdictions within these regions (`in_*_region`).
//!
//!
//! [UN M49]: https://unstats.un.org/unsd/methodology/m49/overview
//! [region]: mod.region.html
//! [Jurisdiction]: struct.Jurisdiction.html
//! [Alpha2]: enum.Alpha2.html
//! [Alpha3]: enum.Alpha3.html

mod definition;
mod generated;
mod jurisdiction;
#[cfg(feature = "region")]
pub mod region;

// Re-export generated modules
use crate::generated::alpha;

// Publicly export types
pub use crate::alpha::{Alpha2, Alpha3};
pub use crate::jurisdiction::Jurisdiction;

// Assert properties about crate types
use static_assertions as sa;

#[cfg(feature = "region")]
sa::assert_impl_all!(crate::region::Region: Sized, Send, Sync);
#[cfg(feature = "region")]
sa::assert_impl_all!(crate::region::SubRegion: Sized, Send, Sync);
#[cfg(feature = "region")]
sa::assert_impl_all!(crate::region::IntermediateRegion: Sized, Send, Sync);

#[cfg(feature = "region")]
sa::assert_eq_size!(crate::region::Region, u8);
#[cfg(feature = "region")]
sa::assert_eq_size!(crate::region::SubRegion, u8);
#[cfg(feature = "region")]
sa::assert_eq_size!(crate::region::IntermediateRegion, u8);

sa::assert_impl_all!(crate::definition::Definition: Sized, Send, Sync);

// Assert that the Jurisdiction object is the same size as a simple pointer.
sa::assert_eq_size!(Jurisdiction, usize);

sa::assert_eq_size!(Alpha2, u8);
sa::assert_eq_size!(Alpha3, u8);
