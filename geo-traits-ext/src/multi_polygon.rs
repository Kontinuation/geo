// Extend MultiPolygonTrait traits for the `geo-traits` crate

use geo_traits::{MultiPolygonTrait, UnimplementedMultiPolygon, UnimplementedPolygon};
use geo_types::{CoordNum, MultiPolygon, Polygon};

use crate::{GeoTraitExtWithTypeMarker, MultiPolygonTraitExtMarker, PolygonTraitExt};

pub trait MultiPolygonTraitExt<T: CoordNum>:
    MultiPolygonTrait<T = T> + GeoTraitExtWithTypeMarker
{
    type PolygonTypeExt<'a>: 'a + PolygonTraitExt<T>
    where
        Self: 'a;

    fn polygon_ext(&self, i: usize) -> Option<Self::PolygonTypeExt<'_>>;
    fn polygon_unchecked_ext(&self, i: usize) -> Self::PolygonTypeExt<'_>;
    fn polygons_ext(&self) -> impl Iterator<Item = Self::PolygonTypeExt<'_>>;
}

#[macro_export]
macro_rules! forward_multi_polygon_trait_ext_funcs {
    () => {
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

impl<T> MultiPolygonTraitExt<T> for MultiPolygon<T>
where
    T: CoordNum,
{
    type PolygonTypeExt<'a>
        = <Self as MultiPolygonTrait>::PolygonType<'a>
    where
        Self: 'a;

    forward_multi_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for MultiPolygon<T> {
    type Marker = MultiPolygonTraitExtMarker;
}

impl<'a, T> MultiPolygonTraitExt<T> for &'a MultiPolygon<T>
where
    T: CoordNum,
{
    type PolygonTypeExt<'b>
        = &'a Polygon<T>
    where
        Self: 'b;

    forward_multi_polygon_trait_ext_funcs!();
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeMarker for &'a MultiPolygon<T> {
    type Marker = MultiPolygonTraitExtMarker;
}

impl<T> MultiPolygonTraitExt<T> for UnimplementedMultiPolygon<T>
where
    T: CoordNum,
{
    type PolygonTypeExt<'a>
        = UnimplementedPolygon<T>
    where
        Self: 'a;

    forward_multi_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for UnimplementedMultiPolygon<T> {
    type Marker = MultiPolygonTraitExtMarker;
}
