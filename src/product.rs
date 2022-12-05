//!
//! product.rs
//!

use std::str::FromStr;

use strum_macros::EnumString;

#[allow(non_camel_case_types)]
#[derive(Debug, EnumString, PartialEq)]
pub(crate) enum Product {
    iOS,
    iPadOS,
    macOS,
    watchOS,
    tvOS,
    Xcode,
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn parse_product_existing_variants() {
    assert_eq!(Product::iOS, Product::from_str("iOS").unwrap());
    assert_eq!(Product::macOS, Product::from_str("macOS").unwrap());
    assert_eq!(Product::watchOS, Product::from_str("watchOS").unwrap());
    assert_eq!(Product::tvOS, Product::from_str("tvOS").unwrap());
    assert_eq!(Product::Xcode, Product::from_str("Xcode").unwrap());
}

#[test]
fn parse_product_nonexistent_variant() {
    let result = Product::from_str("blah");
    assert!(result.is_err());
    assert_eq!(strum::ParseError::VariantNotFound, result.unwrap_err());
}
