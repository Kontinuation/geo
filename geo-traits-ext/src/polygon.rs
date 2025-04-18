// Extend PolygonTrait traits for the `geo-traits` crate

use geo_traits::{PolygonTrait, UnimplementedPolygon};
use geo_types::{CoordNum, Polygon};

use crate::{GeoTraitExtWithTypeTag, LineStringTraitExt, PolygonTag};

pub trait PolygonTraitExt<T: CoordNum>: PolygonTrait<T = T> + GeoTraitExtWithTypeTag {
    type RingTypeExt<'a>: 'a + LineStringTraitExt<T>
    where
        Self: 'a;

    fn exterior_ext(&self) -> Option<Self::RingTypeExt<'_>>;
    fn interiors_ext(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::RingTypeExt<'_>>;
    fn interior_ext(&self, i: usize) -> Option<Self::RingTypeExt<'_>>;
    unsafe fn interior_unchecked_ext(&self, i: usize) -> Self::RingTypeExt<'_>;
}

#[macro_export]
macro_rules! forward_polygon_trait_ext_funcs {
    () => {
        type RingTypeExt<'__l_inner>
            = <Self as PolygonTrait>::RingType<'__l_inner>
        where
            Self: '__l_inner;

        fn exterior_ext(&self) -> Option<Self::RingTypeExt<'_>> {
            <Self as PolygonTrait>::exterior(self)
        }

        fn interiors_ext(
            &self,
        ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::RingTypeExt<'_>> {
            <Self as PolygonTrait>::interiors(self)
        }

        fn interior_ext(&self, i: usize) -> Option<Self::RingTypeExt<'_>> {
            <Self as PolygonTrait>::interior(self, i)
        }

        unsafe fn interior_unchecked_ext(&self, i: usize) -> Self::RingTypeExt<'_> {
            <Self as PolygonTrait>::interior_unchecked(self, i)
        }
    };
}

impl<T> PolygonTraitExt<T> for Polygon<T>
where
    T: CoordNum,
{
    forward_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for Polygon<T> {
    type Tag = PolygonTag;
}

impl<'a, T> PolygonTraitExt<T> for &'a Polygon<T>
where
    T: CoordNum,
{
    forward_polygon_trait_ext_funcs!();
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeTag for &'a Polygon<T> {
    type Tag = PolygonTag;
}

impl<T> PolygonTraitExt<T> for UnimplementedPolygon<T>
where
    T: CoordNum,
{
    forward_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedPolygon<T> {
    type Tag = PolygonTag;
}
