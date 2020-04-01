//! Definitions internal to the crate

use crate::alpha::*;
#[cfg(feature = "region")]
use crate::region::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Definition {
    pub country_code: u16,
    pub name: &'static str,
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    #[cfg(feature = "region")]
    pub region: Region,
    #[cfg(feature = "region")]
    pub sub_region: SubRegion,
    #[cfg(feature = "region")]
    pub intermediate_region: IntermediateRegion,
    #[cfg(feature = "region")]
    pub region_code: u16,
    #[cfg(feature = "region")]
    pub sub_region_code: u16,
    #[cfg(feature = "region")]
    pub intermediate_region_code: Option<u16>,
}

lazy_static! {
    pub static ref DEFINITIONS: HashMap<u16, &'static Definition> = {
        let mut map = HashMap::new();

        for def in crate::generated::definition::GENERATED_DEFINITIONS.iter() {
            map.insert(def.country_code, def);
        }

        map
    };
}
