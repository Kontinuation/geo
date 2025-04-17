//! Extend CoordTrait traits for the `geo-traits` crate

use geo_traits::{CoordTrait, UnimplementedCoord};
use geo_types::{Coord, CoordNum};

pub trait CoordTraitExt<T: CoordNum>: CoordTrait<T = T> {
    // We don't need to extend anything here, because we already have the
    // `ToGeoCoord` trait in `to_geo.rs`
}

impl<T> CoordTraitExt<T> for Coord<T> where T: CoordNum {}

impl<'a, T> CoordTraitExt<T> for &'a Coord<T> where T: CoordNum {}

impl<T> CoordTraitExt<T> for UnimplementedCoord<T> where T: CoordNum {}
