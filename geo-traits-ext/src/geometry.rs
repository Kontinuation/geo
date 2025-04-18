// Extend GeometryTrait traits for the `geo-traits` crate

use core::marker::PhantomData;

use geo_traits::*;
use geo_types::*;

use crate::*;

pub trait GeometryTraitExt<T: CoordNum>: GeometryTrait<T = T> + GeoTraitExtWithTypeTag {
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
        type PointTypeExt<'__g_inner>
            = <Self as GeometryTrait>::PointType<'__g_inner>
        where
            Self: '__g_inner;

        type LineStringTypeExt<'__g_inner>
            = <Self as GeometryTrait>::LineStringType<'__g_inner>
        where
            Self: '__g_inner;

        type PolygonTypeExt<'__g_inner>
            = <Self as GeometryTrait>::PolygonType<'__g_inner>
        where
            Self: '__g_inner;

        type MultiPointTypeExt<'__g_inner>
            = <Self as GeometryTrait>::MultiPointType<'__g_inner>
        where
            Self: '__g_inner;

        type MultiLineStringTypeExt<'__g_inner>
            = <Self as GeometryTrait>::MultiLineStringType<'__g_inner>
        where
            Self: '__g_inner;

        type MultiPolygonTypeExt<'__g_inner>
            = <Self as GeometryTrait>::MultiPolygonType<'__g_inner>
        where
            Self: '__g_inner;

        type GeometryCollectionTypeExt<'__g_inner>
            = <Self as GeometryTrait>::GeometryCollectionType<'__g_inner>
        where
            Self: '__g_inner;

        type RectTypeExt<'__g_inner>
            = <Self as GeometryTrait>::RectType<'__g_inner>
        where
            Self: '__g_inner;

        type TriangleTypeExt<'__g_inner>
            = <Self as GeometryTrait>::TriangleType<'__g_inner>
        where
            Self: '__g_inner;

        type LineTypeExt<'__g_inner>
            = <Self as GeometryTrait>::LineType<'__g_inner>
        where
            Self: '__g_inner;

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
    forward_geometry_trait_ext_funcs!(T);
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for Geometry<T> {
    type Tag = GeometryTag;
}

impl<T> GeometryTraitExt<T> for &Geometry<T>
where
    T: CoordNum,
{
    forward_geometry_trait_ext_funcs!(T);
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for &Geometry<T> {
    type Tag = GeometryTag;
}

impl<T> GeometryTraitExt<T> for UnimplementedGeometry<T>
where
    T: CoordNum,
{
    forward_geometry_trait_ext_funcs!(T);
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedGeometry<T> {
    type Tag = GeometryTag;
}
