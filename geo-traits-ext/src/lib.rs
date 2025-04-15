//! Extended traits for the `geo-traits` crate
//!
//! This crate extends the `geo-traits` crate with additional traits and
//! implementations. The goal is to provide a set of traits that are useful for
//! implementing algorithms on top of the `geo` crate. Most of the methods are
//! inspired by the `geo-types` crate, but are implemented as traits on the
//! `geo-traits` types. Some methods returns concrete types defined in `geo-types`,
//! these methods are only for computing tiny, intermediate results during
//! algorithm execution.
//!
//! The crate is designed to support migration of the `geo` crate to use the
//! traits defined in `geo-traits` by providing generic implementations of the
//! geospatial algorithms, rather than implementing algorithms on concrete types
//! defined in `geo-types`.
//!
//! The crate is currently under active development and the API is subject to
//! change.

#![cfg_attr(not(feature = "std"), no_std)]

pub use coord::CoordTraitExt;
pub use line_string::LineStringTraitExt;
pub use rect::RectTraitExt;
pub use triangle::TriangleTraitExt;

mod coord;
mod line_string;
mod rect;
mod triangle;
