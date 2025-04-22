// Extend MultiPointTrait traits for the `geo-traits` crate

use geo_traits::{MultiPointTrait, PointTrait, UnimplementedMultiPoint};
use geo_types::{to_geo::ToGeoCoord, Coord, CoordNum, MultiPoint};

use crate::{GeoTraitExtWithTypeTag, MultiPointTag, PointTraitExt};

pub trait MultiPointTraitExt:
    MultiPointTrait + GeoTraitExtWithTypeTag<Tag = MultiPointTag>
where
    <Self as MultiPointTrait>::T: CoordNum,
{
    type PointTypeExt<'a>: 'a + PointTraitExt<T = <Self as MultiPointTrait>::T>
    where
        Self: 'a;

    fn point_ext(&self, i: usize) -> Option<Self::PointTypeExt<'_>>;
    fn point_unchecked_ext(&self, i: usize) -> Self::PointTypeExt<'_>;
    fn points_ext(&self) -> impl Iterator<Item = Self::PointTypeExt<'_>>;

    fn coord_iter(&self) -> impl DoubleEndedIterator<Item = Coord<<Self as MultiPointTrait>::T>> {
        self.points().flat_map(|p| p.coord().map(|c| c.to_coord()))
    }
}

#[macro_export]
macro_rules! forward_multi_point_trait_ext_funcs {
    () => {
        type PointTypeExt<'__l_inner>
            = <Self as MultiPointTrait>::PointType<'__l_inner>
        where
            Self: '__l_inner;

        fn point_ext(&self, i: usize) -> Option<Self::PointTypeExt<'_>> {
            <Self as MultiPointTrait>::point(self, i)
        }

        fn point_unchecked_ext(&self, i: usize) -> Self::PointTypeExt<'_> {
            unsafe { <Self as MultiPointTrait>::point_unchecked(self, i) }
        }

        fn points_ext(&self) -> impl Iterator<Item = Self::PointTypeExt<'_>> {
            <Self as MultiPointTrait>::points(self)
        }
    };
}

impl<T> MultiPointTraitExt for MultiPoint<T>
where
    T: CoordNum,
{
    forward_multi_point_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for MultiPoint<T> {
    type Tag = MultiPointTag;
}

impl<T> MultiPointTraitExt for &MultiPoint<T>
where
    T: CoordNum,
{
    forward_multi_point_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for &MultiPoint<T> {
    type Tag = MultiPointTag;
}

impl<T> MultiPointTraitExt for UnimplementedMultiPoint<T>
where
    T: CoordNum,
{
    forward_multi_point_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedMultiPoint<T> {
    type Tag = MultiPointTag;
}
