//!
//! product.rs
//!

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, enum_utils::FromStr)]
pub(crate) enum Product {
    iOS,
    iPadOS,
    macOS,
    tvOS,
    visionOS,
    watchOS,
    Xcode,
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn parse_product_existing_variants() {
    assert_eq!(Product::iOS, "iOS".parse().unwrap());
    assert_eq!(Product::macOS, "macOS".parse().unwrap());
    assert_eq!(Product::tvOS, "tvOS".parse().unwrap());
    assert_eq!(Product::visionOS, "visionOS".parse().unwrap());
    assert_eq!(Product::watchOS, "watchOS".parse().unwrap());
    assert_eq!(Product::Xcode, "Xcode".parse().unwrap());
}

#[test]
fn parse_product_nonexistent_variant() {
    let result = "blah".parse::<Product>();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ());
}
