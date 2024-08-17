//!
//! product.rs
//!

use std::fmt;
use std::fmt::Display;

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

impl Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Product::iOS => write!(f, "iOS"),
            Product::iPadOS => write!(f, "iPadOS"),
            Product::macOS => write!(f, "macOS"),
            Product::tvOS => write!(f, "tvOS"),
            Product::visionOS => write!(f, "visionOS"),
            Product::watchOS => write!(f, "watchOS"),
            Product::Xcode => write!(f, "Xcode"),
        }
    }
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
