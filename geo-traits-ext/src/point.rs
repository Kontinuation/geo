// Extend PointTrait traits for the `geo-traits` crate

use geo_traits::{PointTrait, UnimplementedCoord, UnimplementedPoint};
use geo_types::{Coord, CoordNum, Point};

use crate::{CoordTraitExt, GeoTraitExtWithTypeTag, PointTag};

pub trait PointTraitExt<T: CoordNum>: PointTrait<T = T> + GeoTraitExtWithTypeTag {
    type CoordTypeExt<'a>: 'a + CoordTraitExt<T>
    where
        Self: 'a;

    fn coord_ext(&self) -> Option<Self::CoordTypeExt<'_>>;
}

#[macro_export]
macro_rules! forward_point_trait_ext_funcs {
    () => {
        fn coord_ext(&self) -> Option<Self::CoordTypeExt<'_>> {
            <Self as PointTrait>::coord(self)
        }
    };
}

impl<T> PointTraitExt<T> for Point<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'a>
        = <Self as PointTrait>::CoordType<'a>
    where
        Self: 'a;

    forward_point_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for Point<T> {
    type Marker = PointTag;
}

impl<'a, T> PointTraitExt<T> for &'a Point<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'b>
        = &'a Coord<T>
    where
        Self: 'b;

    forward_point_trait_ext_funcs!();
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeTag for &'a Point<T> {
    type Marker = PointTag;
}

impl<T> PointTraitExt<T> for UnimplementedPoint<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'a>
        = UnimplementedCoord<T>
    where
        Self: 'a;

    forward_point_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedPoint<T> {
    type Marker = PointTag;
}
