// Extend MultiPointTrait traits for the `geo-traits` crate

use geo_traits::{MultiPointTrait, PointTrait, UnimplementedMultiPoint, UnimplementedPoint};
use geo_types::{to_geo::ToGeoCoord, Coord, CoordNum, MultiPoint, Point};

use crate::{GeoTraitExtWithTypeMarker, MultiPointTraitExtMarker, PointTraitExt};

pub trait MultiPointTraitExt<T: CoordNum>:
    MultiPointTrait<T = T> + GeoTraitExtWithTypeMarker
{
    type PointTypeExt<'a>: 'a + PointTraitExt<T>
    where
        Self: 'a;

    fn point_ext(&self, i: usize) -> Option<Self::PointTypeExt<'_>>;
    fn point_unchecked_ext(&self, i: usize) -> Self::PointTypeExt<'_>;
    fn points_ext(&self) -> impl Iterator<Item = Self::PointTypeExt<'_>>;

    fn coord_iter(&self) -> impl DoubleEndedIterator<Item = Coord<T>> {
        self.points().flat_map(|p| p.coord().map(|c| c.to_coord()))
    }
}

#[macro_export]
macro_rules! forward_multi_point_trait_ext_funcs {
    () => {
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

impl<T> MultiPointTraitExt<T> for MultiPoint<T>
where
    T: CoordNum,
{
    type PointTypeExt<'a>
        = <Self as MultiPointTrait>::PointType<'a>
    where
        Self: 'a;

    forward_multi_point_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for MultiPoint<T> {
    type Marker = MultiPointTraitExtMarker;
}

impl<'a, T> MultiPointTraitExt<T> for &'a MultiPoint<T>
where
    T: CoordNum,
{
    type PointTypeExt<'b>
        = &'a Point<T>
    where
        Self: 'b;

    forward_multi_point_trait_ext_funcs!();
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeMarker for &'a MultiPoint<T> {
    type Marker = MultiPointTraitExtMarker;
}

impl<T> MultiPointTraitExt<T> for UnimplementedMultiPoint<T>
where
    T: CoordNum,
{
    type PointTypeExt<'a>
        = UnimplementedPoint<T>
    where
        Self: 'a;

    forward_multi_point_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for UnimplementedMultiPoint<T> {
    type Marker = MultiPointTraitExtMarker;
}
