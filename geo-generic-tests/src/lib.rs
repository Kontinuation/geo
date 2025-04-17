// A module for testing the geo-* ecosystem of crates with a focus on generics.
// This is not a crate for being reused, only for simulating the usage of the
// geo-traits, geo-traits-ext and geo-generic-alg crates.

mod wkb;

// Re-export WKB module components
pub use wkb::error;
pub use wkb::reader;
pub use wkb::{Endianness, WKBType};
