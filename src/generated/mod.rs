//! Generated code by build.

pub mod alpha {
    include!(concat!(env!("OUT_DIR"), "/src/generated/alpha.rs"));
}
pub mod definition {
    include!(concat!(env!("OUT_DIR"), "/src/generated/definition.rs"));
}
#[cfg(feature = "region")]
pub mod region {
    include!(concat!(env!("OUT_DIR"), "/src/generated/region.rs"));
}
