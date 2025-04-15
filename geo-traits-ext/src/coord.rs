//! Extend CoordTrait traits for the `geo-traits` crate

use geo_traits::CoordTrait;
use geo_types::CoordNum;

pub trait CoordTraitExt<T: CoordNum>: CoordTrait<T = T> {
    // We don't need to extend anything here, because we already have the
    // `ToGeoCoord` trait in `to_geo.rs`
}

impl<C, T> CoordTraitExt<T> for C
where
    C: CoordTrait<T = T>,
    T: CoordNum,
{
}
