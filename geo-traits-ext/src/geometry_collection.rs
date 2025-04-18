// Extend GeometryCollectionTrait traits for the `geo-traits` crate

use geo_traits::{GeometryCollectionTrait, UnimplementedGeometry, UnimplementedGeometryCollection};
use geo_types::{CoordNum, Geometry, GeometryCollection};

use crate::{GeoTraitExtWithTypeMarker, GeometryCollectionTraitExtMarker, GeometryTraitExt};

pub trait GeometryCollectionTraitExt<T: CoordNum>:
    GeometryCollectionTrait<T = T> + GeoTraitExtWithTypeMarker
{
    type GeometryTypeExt<'a>: 'a + GeometryTraitExt<T>
    where
        Self: 'a;

    fn geometry_ext(&self, i: usize) -> Option<Self::GeometryTypeExt<'_>>;
    fn geometry_unchecked_ext(&self, i: usize) -> Self::GeometryTypeExt<'_>;
    fn geometries_ext(&self) -> impl Iterator<Item = Self::GeometryTypeExt<'_>>;
}

#[macro_export]
macro_rules! forward_geometry_collection_trait_ext_funcs {
    () => {
        fn geometry_ext(&self, i: usize) -> Option<Self::GeometryTypeExt<'_>> {
            <Self as GeometryCollectionTrait>::geometry(self, i)
        }

        fn geometry_unchecked_ext(&self, i: usize) -> Self::GeometryTypeExt<'_> {
            unsafe { <Self as GeometryCollectionTrait>::geometry_unchecked(self, i) }
        }

        fn geometries_ext(&self) -> impl Iterator<Item = Self::GeometryTypeExt<'_>> {
            <Self as GeometryCollectionTrait>::geometries(self)
        }
    };
}

impl<T> GeometryCollectionTraitExt<T> for GeometryCollection<T>
where
    T: CoordNum,
{
    type GeometryTypeExt<'a>
        = <Self as GeometryCollectionTrait>::GeometryType<'a>
    where
        Self: 'a;

    forward_geometry_collection_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for GeometryCollection<T> {
    type Marker = GeometryCollectionTraitExtMarker;
}

impl<'a, T> GeometryCollectionTraitExt<T> for &'a GeometryCollection<T>
where
    T: CoordNum,
{
    type GeometryTypeExt<'b>
        = &'a Geometry<T>
    where
        Self: 'b;

    forward_geometry_collection_trait_ext_funcs!();
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeMarker for &'a GeometryCollection<T> {
    type Marker = GeometryCollectionTraitExtMarker;
}

impl<T> GeometryCollectionTraitExt<T> for UnimplementedGeometryCollection<T>
where
    T: CoordNum,
{
    type GeometryTypeExt<'a>
        = UnimplementedGeometry<T>
    where
        Self: 'a;

    forward_geometry_collection_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for UnimplementedGeometryCollection<T> {
    type Marker = GeometryCollectionTraitExtMarker;
}
