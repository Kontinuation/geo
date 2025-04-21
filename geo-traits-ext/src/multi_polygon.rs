// Extend MultiPolygonTrait traits for the `geo-traits` crate

use geo_traits::{MultiPolygonTrait, UnimplementedMultiPolygon};
use geo_types::{CoordNum, MultiPolygon};

use crate::{GeoTraitExtWithTypeTag, MultiPolygonTag, PolygonTraitExt};

pub trait MultiPolygonTraitExt: MultiPolygonTrait + GeoTraitExtWithTypeTag
where
    <Self as MultiPolygonTrait>::T: CoordNum,
{
    type PolygonTypeExt<'a>: 'a + PolygonTraitExt<T = <Self as MultiPolygonTrait>::T>
    where
        Self: 'a;

    fn polygon_ext(&self, i: usize) -> Option<Self::PolygonTypeExt<'_>>;
    fn polygon_unchecked_ext(&self, i: usize) -> Self::PolygonTypeExt<'_>;
    fn polygons_ext(&self) -> impl Iterator<Item = Self::PolygonTypeExt<'_>>;
}

#[macro_export]
macro_rules! forward_multi_polygon_trait_ext_funcs {
    () => {
        type PolygonTypeExt<'__l_inner>
            = <Self as MultiPolygonTrait>::PolygonType<'__l_inner>
        where
            Self: '__l_inner;

        fn polygon_ext(&self, i: usize) -> Option<Self::PolygonTypeExt<'_>> {
            <Self as MultiPolygonTrait>::polygon(self, i)
        }

        fn polygon_unchecked_ext(&self, i: usize) -> Self::PolygonTypeExt<'_> {
            unsafe { <Self as MultiPolygonTrait>::polygon_unchecked(self, i) }
        }

        fn polygons_ext(&self) -> impl Iterator<Item = Self::PolygonTypeExt<'_>> {
            <Self as MultiPolygonTrait>::polygons(self)
        }
    };
}

impl<T> MultiPolygonTraitExt for MultiPolygon<T>
where
    T: CoordNum,
{
    forward_multi_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for MultiPolygon<T> {
    type Tag = MultiPolygonTag;
}

impl<T> MultiPolygonTraitExt for &MultiPolygon<T>
where
    T: CoordNum,
{
    forward_multi_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for &MultiPolygon<T> {
    type Tag = MultiPolygonTag;
}

impl<T> MultiPolygonTraitExt for UnimplementedMultiPolygon<T>
where
    T: CoordNum,
{
    forward_multi_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedMultiPolygon<T> {
    type Tag = MultiPolygonTag;
}
