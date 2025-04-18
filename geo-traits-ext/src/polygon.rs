// Extend PolygonTrait traits for the `geo-traits` crate

use geo_traits::{PolygonTrait, UnimplementedLineString, UnimplementedPolygon};
use geo_types::{CoordNum, LineString, Polygon};

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

    // fn exterior_coord_iter(&'_ self) -> impl Iterator<Item = Coord<T>> + '_ {
    //     // match self.exterior() {
    //     //     None => todo!(),
    //     //     Some(exterior) => {
    //     //         exterior.coord_iter()
    //     //     }
    //     // }
    //     // let exterior = self.exterior().unwrap();
    //     // exterior.coord_iter()
    //     self.exterior().unwrap().coord_iter()
    // }
}

#[macro_export]
macro_rules! forward_polygon_trait_ext_funcs {
    () => {
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
    type RingTypeExt<'a>
        = Self::RingType<'a>
    where
        Self: 'a;

    forward_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for Polygon<T> {
    type Marker = PolygonTag;
}

impl<'a, T> PolygonTraitExt<T> for &'a Polygon<T>
where
    T: CoordNum,
{
    type RingTypeExt<'b>
        = &'a LineString<T>
    where
        Self: 'b;

    forward_polygon_trait_ext_funcs!();
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeTag for &'a Polygon<T> {
    type Marker = PolygonTag;
}

impl<T> PolygonTraitExt<T> for UnimplementedPolygon<T>
where
    T: CoordNum,
{
    type RingTypeExt<'a>
        = UnimplementedLineString<T>
    where
        Self: 'a;

    forward_polygon_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedPolygon<T> {
    type Marker = PolygonTag;
}
