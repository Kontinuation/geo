// Extend GeometryTrait traits for the `geo-traits` crate

use core::marker::PhantomData;

use geo_traits::*;
use geo_types::*;

use crate::*;

pub trait GeometryTraitExt<T: CoordNum>: GeometryTrait<T = T> + GeoTraitExtWithTypeMarker {
    type PointTypeExt<'a>: 'a + PointTraitExt<T>
    where
        Self: 'a;

    type LineStringTypeExt<'a>: 'a + LineStringTraitExt<T>
    where
        Self: 'a;

    type PolygonTypeExt<'a>: 'a + PolygonTraitExt<T>
    where
        Self: 'a;

    type MultiPointTypeExt<'a>: 'a + MultiPointTraitExt<T>
    where
        Self: 'a;

    type MultiLineStringTypeExt<'a>: 'a + MultiLineStringTraitExt<T>
    where
        Self: 'a;

    type MultiPolygonTypeExt<'a>: 'a + MultiPolygonTraitExt<T>
    where
        Self: 'a;

    type TriangleTypeExt<'a>: 'a + TriangleTraitExt<T>
    where
        Self: 'a;

    type RectTypeExt<'a>: 'a + RectTraitExt<T>
    where
        Self: 'a;

    type LineTypeExt<'a>: 'a + LineTraitExt<T>
    where
        Self: 'a;

    type GeometryCollectionTypeExt<'a>: 'a + GeometryCollectionTraitExt<T>
    where
        Self: 'a;

    fn as_type_ext(
        &self,
    ) -> GeometryTypeExt<
        '_,
        T,
        Self::PointTypeExt<'_>,
        Self::LineStringTypeExt<'_>,
        Self::PolygonTypeExt<'_>,
        Self::MultiPointTypeExt<'_>,
        Self::MultiLineStringTypeExt<'_>,
        Self::MultiPolygonTypeExt<'_>,
        Self::GeometryCollectionTypeExt<'_>,
        Self::RectTypeExt<'_>,
        Self::TriangleTypeExt<'_>,
        Self::LineTypeExt<'_>,
    >;
}

#[derive(Debug)]
pub enum GeometryTypeExt<'a, T, P, LS, Y, MP, ML, MY, GC, R, TT, L>
where
    T: CoordNum,
    P: PointTraitExt<T>,
    LS: LineStringTraitExt<T>,
    Y: PolygonTraitExt<T>,
    MP: MultiPointTraitExt<T>,
    ML: MultiLineStringTraitExt<T>,
    MY: MultiPolygonTraitExt<T>,
    GC: GeometryCollectionTraitExt<T>,
    R: RectTraitExt<T>,
    TT: TriangleTraitExt<T>,
    L: LineTraitExt<T>,
{
    Point(&'a P),
    LineString(&'a LS),
    Polygon(&'a Y),
    MultiPoint(&'a MP),
    MultiLineString(&'a ML),
    MultiPolygon(&'a MY),
    GeometryCollection(&'a GC),
    Rect(&'a R),
    Triangle(&'a TT),
    Line(&'a L),
    _Unused(&'a PhantomData<T>),
}

#[macro_export]
macro_rules! forward_geometry_trait_ext_funcs {
    ($t:ty) => {
        fn as_type_ext(
            &self,
        ) -> GeometryTypeExt<
            '_,
            $t,
            Self::PointTypeExt<'_>,
            Self::LineStringTypeExt<'_>,
            Self::PolygonTypeExt<'_>,
            Self::MultiPointTypeExt<'_>,
            Self::MultiLineStringTypeExt<'_>,
            Self::MultiPolygonTypeExt<'_>,
            Self::GeometryCollectionTypeExt<'_>,
            Self::RectTypeExt<'_>,
            Self::TriangleTypeExt<'_>,
            Self::LineTypeExt<'_>,
        > {
            match self.as_type() {
                GeometryType::Point(p) => GeometryTypeExt::Point(p),
                GeometryType::LineString(ls) => GeometryTypeExt::LineString(ls),
                GeometryType::Polygon(p) => GeometryTypeExt::Polygon(p),
                GeometryType::MultiPoint(mp) => GeometryTypeExt::MultiPoint(mp),
                GeometryType::MultiLineString(mls) => GeometryTypeExt::MultiLineString(mls),
                GeometryType::MultiPolygon(mp) => GeometryTypeExt::MultiPolygon(mp),
                GeometryType::GeometryCollection(gc) => GeometryTypeExt::GeometryCollection(gc),
                GeometryType::Rect(r) => GeometryTypeExt::Rect(r),
                GeometryType::Triangle(t) => GeometryTypeExt::Triangle(t),
                GeometryType::Line(l) => GeometryTypeExt::Line(l),
            }
        }
    };
}

impl<T> GeometryTraitExt<T> for Geometry<T>
where
    T: CoordNum,
{
    type PointTypeExt<'b>
        = Point<T>
    where
        Self: 'b;
    type LineStringTypeExt<'b>
        = LineString<T>
    where
        Self: 'b;
    type PolygonTypeExt<'b>
        = Polygon<T>
    where
        Self: 'b;
    type MultiPointTypeExt<'b>
        = MultiPoint<T>
    where
        Self: 'b;
    type MultiLineStringTypeExt<'b>
        = MultiLineString<T>
    where
        Self: 'b;
    type MultiPolygonTypeExt<'b>
        = MultiPolygon<T>
    where
        Self: 'b;
    type GeometryCollectionTypeExt<'b>
        = GeometryCollection<T>
    where
        Self: 'b;
    type TriangleTypeExt<'b>
        = Triangle<T>
    where
        Self: 'b;
    type RectTypeExt<'b>
        = Rect<T>
    where
        Self: 'b;
    type LineTypeExt<'b>
        = Line<T>
    where
        Self: 'b;

    forward_geometry_trait_ext_funcs!(T);
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for Geometry<T> {
    type Marker = GeometryTraitExtMarker;
}

impl<'a, T> GeometryTraitExt<T> for &'a Geometry<T>
where
    T: CoordNum,
{
    type PointTypeExt<'b>
        = Point<T>
    where
        Self: 'b;
    type LineStringTypeExt<'b>
        = LineString<T>
    where
        Self: 'b;
    type PolygonTypeExt<'b>
        = Polygon<T>
    where
        Self: 'b;
    type MultiPointTypeExt<'b>
        = MultiPoint<T>
    where
        Self: 'b;
    type MultiLineStringTypeExt<'b>
        = MultiLineString<T>
    where
        Self: 'b;
    type MultiPolygonTypeExt<'b>
        = MultiPolygon<T>
    where
        Self: 'b;
    type GeometryCollectionTypeExt<'b>
        = GeometryCollection<T>
    where
        Self: 'b;
    type TriangleTypeExt<'b>
        = Triangle<T>
    where
        Self: 'b;
    type RectTypeExt<'b>
        = Rect<T>
    where
        Self: 'b;
    type LineTypeExt<'b>
        = Line<T>
    where
        Self: 'b;

    forward_geometry_trait_ext_funcs!(T);
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeMarker for &'a Geometry<T> {
    type Marker = GeometryTraitExtMarker;
}

impl<T> GeometryTraitExt<T> for UnimplementedGeometry<T>
where
    T: CoordNum,
{
    type PointTypeExt<'a>
        = UnimplementedPoint<T>
    where
        Self: 'a;
    type LineStringTypeExt<'a>
        = UnimplementedLineString<T>
    where
        Self: 'a;
    type PolygonTypeExt<'a>
        = UnimplementedPolygon<T>
    where
        Self: 'a;
    type MultiPointTypeExt<'a>
        = UnimplementedMultiPoint<T>
    where
        Self: 'a;
    type MultiLineStringTypeExt<'a>
        = UnimplementedMultiLineString<T>
    where
        Self: 'a;
    type MultiPolygonTypeExt<'a>
        = UnimplementedMultiPolygon<T>
    where
        Self: 'a;
    type GeometryCollectionTypeExt<'a>
        = UnimplementedGeometryCollection<T>
    where
        Self: 'a;
    type TriangleTypeExt<'a>
        = UnimplementedTriangle<T>
    where
        Self: 'a;
    type RectTypeExt<'a>
        = UnimplementedRect<T>
    where
        Self: 'a;
    type LineTypeExt<'a>
        = UnimplementedLine<T>
    where
        Self: 'a;

    forward_geometry_trait_ext_funcs!(T);
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for UnimplementedGeometry<T> {
    type Marker = GeometryTraitExtMarker;
}
