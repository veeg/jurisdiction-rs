//! [UN M49] region definitions.
//!
//! This information is typeset from the UN methodology on
//! standard country or area codes for statistical use (M49)
//! to extract the region definition.
//!
//! [UN M49]: https://unstats.un.org/unsd/methodology/m49/overview

use serde::{Deserialize, Serialize};

/// The high level region a Jurisdiction may zone to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[allow(missing_docs)]
pub enum Region {
    Africa,
    Asia,
    Europe,
    Oceania,
    Americas,

    #[serde(other)]
    Undefined,
}

/// A subdivision within a [Region](enum.Region.html).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[allow(missing_docs)]
pub enum SubRegion {
    // Africa
    #[serde(rename = "Northern Africa")]
    NorthernAfrica,
    #[serde(rename = "Sub-Saharan Africa")]
    SubSaharanAfrica,
    // Asia
    #[serde(rename = "Eastern Asia")]
    EasternAsia,
    #[serde(rename = "Southern Asia")]
    SouthernAsia,
    #[serde(rename = "South-eastern Asia")]
    SouthEasternAsia,
    #[serde(rename = "Western Asia")]
    WesternAsia,
    #[serde(rename = "Central Asia")]
    CentralAsia,
    // America
    #[serde(rename = "Northern America")]
    NorthernAmerica,
    #[serde(rename = "Latin America and the Caribbean")]
    LatinAmericaAndTheCaribbean,
    // Europa
    #[serde(rename = "Northern Europe")]
    NorthernEurope,
    #[serde(rename = "Eastern Europe")]
    EasternEurope,
    #[serde(rename = "Southern Europe")]
    SouthernEurope,
    #[serde(rename = "Western Europe")]
    WesternEurope,
    // Oceania
    Polynesia,
    Melanesia,
    Micronesia,
    #[serde(rename = "Australia and New Zealand")]
    AustraliaAndNewZealand,

    #[serde(other)]
    Undefined,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[allow(missing_docs)]
pub enum IntermediateRegion {
    // Africa
    #[serde(rename = "Eastern Africa")]
    EasternAfrica,
    #[serde(rename = "Middle Africa")]
    MiddleAfrica,
    #[serde(rename = "Southern Africa")]
    SouthernAfrica,
    #[serde(rename = "Western Africa")]
    WesternAfrica,
    // America
    Caribbean,
    #[serde(rename = "Central America")]
    CentralAmerica,
    #[serde(rename = "South America")]
    SouthAmerica,
    // Europe,
    #[serde(rename = "Channel Islands")]
    ChannelIslands,

    #[serde(other)]
    Undefined,
}
