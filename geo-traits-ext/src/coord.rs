//! Extend CoordTrait traits for the `geo-traits` crate

use geo_traits::{CoordTrait, UnimplementedCoord};
use geo_types::{Coord, CoordNum};

use crate::{CoordTag, GeoTraitExtWithTypeTag};

pub trait CoordTraitExt<T: CoordNum>: CoordTrait<T = T> + GeoTraitExtWithTypeTag {
    // We don't need to extend anything here, because we already have the
    // `ToGeoCoord` trait in `to_geo.rs`
}

impl<T> CoordTraitExt<T> for Coord<T> where T: CoordNum {}

impl<T: CoordNum> GeoTraitExtWithTypeTag for Coord<T> {
    type Tag = CoordTag;
}

impl<'a, T> CoordTraitExt<T> for &'a Coord<T> where T: CoordNum {}

impl<'a, T: CoordNum> GeoTraitExtWithTypeTag for &'a Coord<T> {
    type Tag = CoordTag;
}

impl<T> CoordTraitExt<T> for UnimplementedCoord<T> where T: CoordNum {}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedCoord<T> {
    type Tag = CoordTag;
}
