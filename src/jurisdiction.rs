//! The main lightweight object used to identify a jurisdiction/country and its metadata.

use crate::alpha::*;
use crate::definition::{Definition, DEFINITIONS};
#[cfg(feature = "region")]
use crate::region::*;

use anyhow::format_err;
use std::str::FromStr;

/// A pointer sized object encoding countries and areas of the world.
///
/// The size of this structure is minimized such that passing it around will be limited
/// overhead, with implemented methods performing lookup in static table instead.
#[derive(Clone, Debug)]
pub struct Jurisdiction {
    definition: &'static Definition,
}

impl std::cmp::PartialEq<Jurisdiction> for Jurisdiction {
    fn eq(&self, other: &Jurisdiction) -> bool {
        self.definition.country_code == other.definition.country_code
    }
}

impl std::cmp::PartialEq<Alpha2> for Jurisdiction {
    fn eq(&self, other: &Alpha2) -> bool {
        &self.definition.alpha2 == other
    }
}

impl std::cmp::PartialEq<Alpha3> for Jurisdiction {
    fn eq(&self, other: &Alpha3) -> bool {
        &self.definition.alpha3 == other
    }
}

impl FromStr for Jurisdiction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(alpha2) = serde_plain::from_str::<Alpha2>(s) {
            Ok(Jurisdiction::from(alpha2))
        } else if let Ok(alpha3) = serde_plain::from_str::<Alpha3>(s) {
            Ok(Jurisdiction::from(alpha3))
        } else {
            Err(format_err!(
                "unrecognized ISO 3166 alpha country code: {}",
                s
            ))
        }
    }
}

impl Jurisdiction {
    pub(crate) fn new(country_code: u16) -> Jurisdiction {
        let def = DEFINITIONS.get(&country_code);
        debug_assert!(
            def.is_some(),
            "passed country code is not defined in DEFINITIONS"
        );
        Jurisdiction {
            definition: def.unwrap(),
        }
    }

    /// Return the english name of this jurisdiction.
    pub fn name(&self) -> &str {
        self.definition.name
    }

    /// Return the ISO-3166 numeric country code made up of 3 characters.
    ///
    /// # Origin
    /// The definition is sourced from ISO-3166 standard.
    pub fn country_code(&self) -> u16 {
        self.definition.country_code
    }

    /// Return the two letter [Alpha2] representation for this `Jurisdiction`.
    ///
    /// # Origin
    /// The definition is sourced from ISO-3166 standard.
    ///
    /// [Alpha2]: enum.Alpha2.html
    pub fn alpha2(&self) -> Alpha2 {
        self.definition.alpha2
    }

    /// Return the two letter [Alpha3] representation for this `Jurisdiction`.
    ///
    /// # Origin
    /// The definition is sourced from the ISO-3166 standard.
    ///
    /// [Alpha3]: enum.Alpha3.html
    pub fn alpha3(&self) -> Alpha3 {
        self.definition.alpha3
    }

    /// Return the [Region] on earth this `Jurisdiction` is situated in.
    ///
    /// # Origin
    /// The definition is sourced from the statistics division of the UN
    /// for standard country and area codes for statistical use (M49).
    ///
    /// [Region]: region/enum.Region.html
    #[cfg(feature = "region")]
    pub fn region(&self) -> Region {
        self.definition.region
    }

    /// Return the [SubRegion] of a [Region] this `Jurisdiction` is situated in.
    ///
    /// # Origin
    /// The definition is sourced from the statistics division of the UN
    /// for standard country and area codes for statistical use (M49).
    ///
    /// [Region]: region/enum.Region.html
    /// [SubRegion]: region/enum.SubRegion.html
    #[cfg(feature = "region")]
    pub fn sub_region(&self) -> SubRegion {
        self.definition.sub_region
    }

    /// Return the [IntermediateRegion] of a [SubRegion] this `Jurisdiction` is situated in.
    ///
    /// Not all `Jursidictions` has a defined `IntermediateRegion`. These will thus return [Undefined].
    ///
    /// # Origin
    /// The definition is sourced from the statistics division of the UN
    /// for standard country and area codes for statistical use (M49).
    ///
    /// [IntermediateRegion]: region/enum.IntermediateRegion.html
    /// [Undefined]: region/enum.IntermediateRegion.html#variant.Undefined
    /// [SubRegion]: region/enum.SubRegion.html
    #[cfg(feature = "region")]
    pub fn intermediate_region(&self) -> IntermediateRegion {
        self.definition.intermediate_region
    }

    /// Return the 3 character numeric identifier for the [Region] this `Jurisdiction` is situated in.
    ///
    /// # Origin
    /// The definition is sourced from the statistics division of the UN
    /// for standard country and area codes for statistical use (M49).
    ///
    /// [Region]: region/enum.Region.html
    #[cfg(feature = "region")]
    pub fn region_code(&self) -> u16 {
        self.definition.region_code
    }

    /// Return the 3 character numeric identifier for the [SubRegion] this `Jurisdiction` is situated in.
    ///
    /// # Origin
    /// The definition is sourced from the statistics division of the UN
    /// for standard country and area codes for statistical use (M49).
    ///
    /// [SubRegion]: region/enum.SubRegion.html
    #[cfg(feature = "region")]
    pub fn sub_region_code(&self) -> u16 {
        self.definition.sub_region_code
    }

    /// Return the 3 character numeric identifier for the [IntermediateRegion]
    /// this `Jurisdiction` is situated in.
    ///
    /// # Origin
    /// The definition is sourced from the statistics division of the UN
    /// for standard country and area codes for statistical use (M49).
    ///
    /// [IntermediateRegion]: region/enum.IntermediateRegion.html
    #[cfg(feature = "region")]
    pub fn intermediate_region_code(&self) -> Option<u16> {
        self.definition.intermediate_region_code
    }

    /// Return all Jurisdictions zoning to specified region.
    #[cfg(feature = "region")]
    pub fn in_region(region: Region) -> Vec<Jurisdiction> {
        region.jurisdictions()
    }

    /// Return all Jurisdictions zoning to specified sub region.
    #[cfg(feature = "region")]
    pub fn in_sub_region(sub: SubRegion) -> Vec<Jurisdiction> {
        sub.jurisdictions()
    }

    /// Return all Jurisdictions zoning to specified sub region.
    #[cfg(feature = "region")]
    pub fn in_intermediate_region(inter: IntermediateRegion) -> Vec<Jurisdiction> {
        inter.jurisdictions()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_trait_alpha2() {
        let no = Jurisdiction::from(Alpha2::NO);
        assert_eq!(no.name(), "Norway");
    }

    #[test]
    fn test_from_trait_alpha3() {
        let no = Jurisdiction::from(Alpha3::NOR);
        assert_eq!(no.name(), "Norway");
    }

    #[test]
    fn test_compare_jurisdiction_with_alpha2() {
        let no = Jurisdiction::from(Alpha2::NO);

        assert_eq!(no, Alpha2::NO);
    }

    #[test]
    fn test_compare_jurisdiction_with_alpha3() {
        let no = Jurisdiction::from(Alpha3::NOR);

        assert_eq!(no, Alpha3::NOR);
    }

    #[test]
    fn test_in_region() {
        let europe = Jurisdiction::in_region(Region::Europe);
        assert!(europe.contains(&Jurisdiction::from(Alpha2::NO)));

        let undefined = Jurisdiction::in_region(Region::Undefined);
        assert_eq!(undefined.len(), 1);
        assert!(undefined.contains(&Jurisdiction::from(Alpha3::ATA)));
    }

    #[test]
    fn test_in_sub_region() {
        let africa = Jurisdiction::in_region(Region::Africa);
        assert!(africa.contains(&Jurisdiction::from(Alpha2::AO)));
    }

    #[test]
    fn test_jurisdiction_getters() {
        let norway = Jurisdiction::from(Alpha2::NO);

        assert_eq!(norway.name(), "Norway");
        assert_eq!(norway.alpha2(), Alpha2::NO);
        assert_eq!(norway.alpha3(), Alpha3::NOR);
        assert_eq!(norway.country_code(), 578);
        assert_eq!(norway.region(), Region::Europe);
        assert_eq!(norway.sub_region(), SubRegion::NorthernEurope);
        assert_eq!(norway.intermediate_region(), IntermediateRegion::Undefined);
        assert_eq!(norway.region_code(), 150);
        assert_eq!(norway.sub_region_code(), 154);
        assert_eq!(norway.intermediate_region_code(), None);
    }

    #[test]
    fn test_jurisdiction_from_str_unknown() {
        let jur = Jurisdiction::from_str("rofl");
        assert!(jur.is_err());
    }

    #[test]
    fn test_jurisdiction_from_str_alpha2() {
        let jur = Jurisdiction::from_str("NO");
        assert!(jur.is_ok());
        let no = jur.unwrap();
        assert_eq!(no, Alpha2::NO);
    }

    #[test]
    fn test_jurisdiction_from_str_alpha3() {
        let jur = Jurisdiction::from_str("NOR");
        assert!(jur.is_ok());
        let no = jur.unwrap();
        assert_eq!(no, Alpha3::NOR);
    }

    #[test]
    fn test_alpha2_display() {
        assert_eq!(Alpha2::NO.to_string(), "NO");
    }

    #[test]
    fn test_alpha3_display() {
        assert_eq!(Alpha3::NOR.to_string(), "NOR");
    }
}
