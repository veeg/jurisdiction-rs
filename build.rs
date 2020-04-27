//! Generate the necessary definitions for `jurisidiction`.

#[path = "src/region.rs"]
mod region;

use crate::region::*;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

#[allow(unused)]
#[derive(Deserialize)]
struct CountryRegionDefinition {
    name: String,
    #[serde(rename = "alpha-2")]
    alpha2: String,
    #[serde(rename = "alpha-3")]
    alpha3: String,
    #[serde(rename = "country-code")]
    country_code: String,
    #[serde(rename = "iso_3166-2")]
    iso_3166_2: String,
    region: Region,
    #[serde(rename = "sub-region")]
    sub_region: SubRegion,
    #[serde(rename = "intermediate-region")]
    intermediate_region: IntermediateRegion,
    #[serde(rename = "region-code")]
    region_code: String,
    #[serde(rename = "sub-region-code")]
    sub_region_code: String,
    #[serde(rename = "intermediate-region-code")]
    intermediate_region_code: String,
}

fn generate_alpha(definitions: &[CountryRegionDefinition]) -> TokenStream {
    let alpha2 = generate_alpha2(&definitions);
    let alpha3 = generate_alpha3(&definitions);

    quote!(
        use serde::{Deserialize, Serialize};

        #alpha2

        #alpha3
    )
}

fn generate_alpha2(definitions: &[CountryRegionDefinition]) -> TokenStream {
    // Generate enum body
    let mut enum_body = TokenStream::new();
    for def in definitions.iter() {
        let a = Ident::new(&def.alpha2, Span::call_site());
        enum_body.extend(quote!(
            #a,
        ));
    }

    // Generate From impl body
    let mut from_match_body = TokenStream::new();
    for def in definitions.iter() {
        let a = Ident::new(&def.alpha2, Span::call_site());
        let cc = u16::from_str(&def.country_code).expect("country code not representable as u16");
        from_match_body.extend(quote!(
            Alpha2::#a => #cc,
        ));
    }

    quote!(
        /// Two alpha character ISO 3166 country code classification.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
        #[allow(missing_docs)]
        pub enum Alpha2 {
            #enum_body
        }

        impl From<Alpha2> for crate::Jurisdiction {
            fn from(alpha: Alpha2) -> Self {
                let country_code = match alpha {
                    #from_match_body
                };

                crate::Jurisdiction::new(country_code)
            }
        }

        impl std::fmt::Display for Alpha2 {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    serde_plain::to_string(&self).map_err(|_| std::fmt::Error)?
                )
            }
        }
    )
}

fn generate_alpha3(definitions: &[CountryRegionDefinition]) -> TokenStream {
    // Generate enum body
    let mut enum_body = TokenStream::new();
    for def in definitions.iter() {
        let a = Ident::new(&def.alpha3, Span::call_site());
        enum_body.extend(quote!(
            #a,
        ));
    }

    // Generate From impl body
    let mut from_match_body = TokenStream::new();
    for def in definitions.iter() {
        let a = Ident::new(&def.alpha3, Span::call_site());
        let cc = u16::from_str(&def.country_code).expect("country code not representable as u16");
        from_match_body.extend(quote!(
            Alpha3::#a => #cc,
        ));
    }

    quote!(
        /// Three alpha character ISO 3166 country code classification.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
        #[allow(missing_docs)]
        pub enum Alpha3 {
            #enum_body
        }

        impl From<Alpha3> for crate::Jurisdiction {
            fn from(alpha: Alpha3) -> Self {
                let country_code = match alpha {
                    #from_match_body
                };

                crate::Jurisdiction::new(country_code)
            }
        }

        impl std::fmt::Display for Alpha3 {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    serde_plain::to_string(&self).map_err(|_| std::fmt::Error)?
                )
            }
        }
    )
}

fn generate_region(definitions: &[CountryRegionDefinition]) -> TokenStream {
    // Gather all country codes for each region
    let mut regions: HashMap<&Region, Vec<u16>> = HashMap::new();
    let mut subs: HashMap<&SubRegion, Vec<u16>> = HashMap::new();
    let mut intermediates: HashMap<&IntermediateRegion, Vec<u16>> = HashMap::new();
    for def in definitions.iter() {
        let cc = u16::from_str(&def.country_code).expect("country code not representable as u16");
        regions.entry(&def.region).or_default().push(cc);
        subs.entry(&def.sub_region).or_default().push(cc);
        intermediates
            .entry(&def.intermediate_region)
            .or_default()
            .push(cc);
    }

    // Generate match arms for region
    let mut region_match: HashMap<Ident, TokenStream> = HashMap::new();
    for (region, codes) in regions {
        let i = Ident::new(&format!("{:?}", &region), Span::call_site());
        let mut t = TokenStream::new();
        for code in codes {
            t.extend(quote!( Jurisdiction::new(#code), ));
        }
        region_match.insert(i, t);
    }

    // Generate match arms for sub region
    let mut sub_match: HashMap<Ident, TokenStream> = HashMap::new();
    for (sub, codes) in subs {
        let i = Ident::new(&format!("{:?}", &sub), Span::call_site());
        let mut t = TokenStream::new();
        for code in codes {
            t.extend(quote!( Jurisdiction::new(#code), ));
        }
        sub_match.insert(i, t);
    }

    // Generate match arms for intermediate region
    let mut intermediate_match: HashMap<Ident, TokenStream> = HashMap::new();
    for (inter, codes) in intermediates {
        let i = Ident::new(&format!("{:?}", &inter), Span::call_site());
        let mut t = TokenStream::new();
        for code in codes {
            t.extend(quote!( Jurisdiction::new(#code), ));
        }
        intermediate_match.insert(i, t);
    }

    // Generate region match body
    let mut region_body = TokenStream::new();
    for (i, t) in region_match {
        region_body.extend(quote!(
            Region::#i => vec![#t],
        ));
    }

    // Generate sub region match body
    let mut sub_body = TokenStream::new();
    for (i, t) in sub_match {
        sub_body.extend(quote!(
            SubRegion::#i => vec![#t],
        ));
    }

    // Generate intermediate region match body
    let mut intermediate_body = TokenStream::new();
    for (i, t) in intermediate_match {
        intermediate_body.extend(quote!(
            IntermediateRegion::#i => vec![#t],
        ));
    }

    quote!(
        use crate::region::{Region, SubRegion, IntermediateRegion};
        use crate::Jurisdiction;

        impl Region {
            #[allow(clippy::trivially_copy_pass_by_ref)]
            pub(crate) fn jurisdictions(&self) -> Vec<Jurisdiction> {
                match *self {
                    #region_body
                }
            }
        }

        impl SubRegion {
            #[allow(clippy::trivially_copy_pass_by_ref)]
            pub(crate) fn jurisdictions(&self) -> Vec<Jurisdiction> {
                match *self {
                    #sub_body
                }
            }
        }

        impl IntermediateRegion {
            #[allow(clippy::trivially_copy_pass_by_ref)]
            pub(crate) fn jurisdictions(&self) -> Vec<Jurisdiction> {
                match *self {
                    #intermediate_body
                }
            }
        }
    )
}

fn generate_definition(definitions: &[CountryRegionDefinition]) -> TokenStream {
    let mut tokendefs = TokenStream::new();
    for def in definitions.iter() {
        let name = &def.name;
        let cc = u16::from_str(&def.country_code).expect("country code not representable as u16");
        let alpha2 = Ident::new(&def.alpha2, Span::call_site());
        let alpha3 = Ident::new(&def.alpha3, Span::call_site());
        let region = Ident::new(&format!("{:?}", &def.region), Span::call_site());
        let sub_region = Ident::new(&format!("{:?}", &def.sub_region), Span::call_site());
        let intermediate_region = Ident::new(
            &format!("{:?}", &def.intermediate_region),
            Span::call_site(),
        );

        let rc = u16::from_str(&def.region_code).unwrap_or(0);
        let sc = u16::from_str(&def.sub_region_code).unwrap_or(0);
        let irc = u16::from_str(&def.intermediate_region_code).unwrap_or(0);

        let irc: TokenStream = match irc {
            0 => proc_macro2::TokenTree::from(Ident::new("None", Span::call_site())).into(),
            _ => quote!(Some(#irc)),
        };

        tokendefs.extend(quote!(
            Definition {
                country_code: #cc,
                name: #name,
                alpha2: Alpha2::#alpha2,
                alpha3: Alpha3::#alpha3,
                #[cfg(feature = "region")]
                region: Region::#region,
                #[cfg(feature = "region")]
                sub_region: SubRegion::#sub_region,
                #[cfg(feature = "region")]
                intermediate_region: IntermediateRegion::#intermediate_region,
                #[cfg(feature = "region")]
                region_code: #rc,
                #[cfg(feature = "region")]
                sub_region_code: #sc,
                #[cfg(feature = "region")]
                intermediate_region_code: #irc,
            },
        ));
    }

    let array_size = definitions.len();
    quote!(
        #[cfg(feature = "region")]
        use crate::region::{Region, SubRegion, IntermediateRegion};
        use crate::alpha::{Alpha2, Alpha3};
        use crate::definition::Definition;

        pub const GENERATED_DEFINITIONS: [Definition; #array_size] = [
            #tokendefs
        ];
    )
}

fn main() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dir = format!("{}/src/generated/", out_dir);
    std::fs::create_dir_all(&dir)?;

    // Ensure that output directory exist

    // Parse the country-region JSON definitions
    let file = File::open("data/country-region.json")?;
    let definitions: Vec<CountryRegionDefinition> = serde_json::from_reader(file)?;

    // Alpha
    let generated = generate_alpha(&definitions);
    let mut f = File::create(format!("{}/alpha.rs", dir))?;
    f.write_all(generated.to_string().as_bytes())?;

    // Region
    let generated = generate_region(&definitions);
    let mut f = File::create(format!("{}/region.rs", dir))?;
    f.write_all(generated.to_string().as_bytes())?;

    // Definition
    let generated = generate_definition(&definitions);
    let mut f = File::create(format!("{}/definition.rs", dir))?;
    f.write_all(generated.to_string().as_bytes())?;

    Ok(())
}
