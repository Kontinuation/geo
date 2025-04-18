use geo_traits::*;
use geo_traits_ext::{
    forward_geometry_trait_ext_funcs, GeoTraitExtWithTypeTag, GeometryTag, GeometryTraitExt,
    GeometryTypeExt,
};
use geo_types::CoordNum;

use super::{line_string::SimpleLineString, point::SimplePoint, polygon::SimplePolygon};

pub enum SimpleGeometry<T: CoordNum> {
    Point(SimplePoint<T>),
    LineString(SimpleLineString<T>),
    Polygon(SimplePolygon<T>),
}

impl<'a, T: CoordNum> GeometryTrait for &'a SimpleGeometry<T> {
    type T = T;

    type PointType<'b>
        = SimplePoint<T>
    where
        Self: 'b;

    type LineStringType<'b>
        = SimpleLineString<T>
    where
        Self: 'b;

    type PolygonType<'b>
        = SimplePolygon<T>
    where
        Self: 'b;

    type MultiPointType<'b>
        = UnimplementedMultiPoint<T>
    where
        Self: 'b;

    type MultiLineStringType<'b>
        = UnimplementedMultiLineString<T>
    where
        Self: 'b;

    type MultiPolygonType<'b>
        = UnimplementedMultiPolygon<T>
    where
        Self: 'b;

    type GeometryCollectionType<'b>
        = UnimplementedGeometryCollection<T>
    where
        Self: 'b;

    type RectType<'b>
        = UnimplementedRect<T>
    where
        Self: 'b;

    type TriangleType<'b>
        = UnimplementedTriangle<T>
    where
        Self: 'b;

    type LineType<'b>
        = UnimplementedLine<T>
    where
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xy
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        match self {
            SimpleGeometry::Point(p) => geo_traits::GeometryType::Point(p),
            SimpleGeometry::LineString(ls) => geo_traits::GeometryType::LineString(ls),
            SimpleGeometry::Polygon(p) => geo_traits::GeometryType::Polygon(p),
        }
    }
}

impl<T: CoordNum> GeometryTrait for SimpleGeometry<T> {
    type T = T;

    type PointType<'b>
        = SimplePoint<T>
    where
        Self: 'b;

    type LineStringType<'b>
        = SimpleLineString<T>
    where
        Self: 'b;

    type PolygonType<'b>
        = SimplePolygon<T>
    where
        Self: 'b;

    type MultiPointType<'b>
        = UnimplementedMultiPoint<T>
    where
        Self: 'b;

    type MultiLineStringType<'b>
        = UnimplementedMultiLineString<T>
    where
        Self: 'b;

    type MultiPolygonType<'b>
        = UnimplementedMultiPolygon<T>
    where
        Self: 'b;

    type GeometryCollectionType<'b>
        = UnimplementedGeometryCollection<T>
    where
        Self: 'b;

    type RectType<'b>
        = UnimplementedRect<T>
    where
        Self: 'b;

    type TriangleType<'b>
        = UnimplementedTriangle<T>
    where
        Self: 'b;

    type LineType<'b>
        = UnimplementedLine<T>
    where
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xy
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        match self {
            SimpleGeometry::Point(p) => geo_traits::GeometryType::Point(p),
            SimpleGeometry::LineString(ls) => geo_traits::GeometryType::LineString(ls),
            SimpleGeometry::Polygon(p) => geo_traits::GeometryType::Polygon(p),
        }
    }
}

impl<'a, T: CoordNum> GeometryTraitExt<T> for &'a SimpleGeometry<T> {
    forward_geometry_trait_ext_funcs!(T);
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeTag for &'a SimpleGeometry<T> {
    type Tag = GeometryTag;
}

impl<T: CoordNum> GeometryTraitExt<T> for SimpleGeometry<T> {
    forward_geometry_trait_ext_funcs!(T);
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for SimpleGeometry<T> {
    type Tag = GeometryTag;
}

#[cfg(test)]
mod tests {
    use crate::simple::coord::SimpleCoord;

    use super::*;

    #[test]
    fn test_simple_geometry() {
        let point = SimplePoint::new(0.0, 0.0);
        let geom_point = SimpleGeometry::Point(point);

        let line_string =
            SimpleLineString::new(vec![SimpleCoord::new(0.0, 0.0), SimpleCoord::new(1.0, 1.0)]);
        let geom_line_string = SimpleGeometry::LineString(line_string);

        let exterior = SimpleLineString::new(vec![
            SimpleCoord::new(0.0, 0.0),
            SimpleCoord::new(1.0, 1.0),
            SimpleCoord::new(1.0, 0.0),
            SimpleCoord::new(0.0, 0.0),
        ]);
        let polygon = SimplePolygon::new(exterior);
        let geom_polygon = SimpleGeometry::Polygon(polygon);

        assert_eq!(geom_point.dim(), geo_traits::Dimensions::Xy);
        assert_eq!(geom_line_string.dim(), geo_traits::Dimensions::Xy);
        assert_eq!(geom_polygon.dim(), geo_traits::Dimensions::Xy);

        match geom_point.as_type() {
            geo_traits::GeometryType::Point(p) => {
                let coord = p.coord().unwrap();
                assert_eq!(coord.x(), 0.0);
                assert_eq!(coord.y(), 0.0);
            }
            _ => panic!("geom_point is not a point"),
        }

        match geom_line_string.as_type() {
            geo_traits::GeometryType::LineString(ls) => {
                let coords = ls.coords();
                for (i, coord) in coords.enumerate() {
                    assert_eq!(coord.x(), i as f64);
                    assert_eq!(coord.y(), i as f64);
                }
            }
            _ => panic!("geom_line_string is not a line string"),
        }

        match geom_polygon.as_type() {
            geo_traits::GeometryType::Polygon(p) => {
                let exterior = p.exterior();
                assert_eq!(exterior.unwrap().coords().len(), 4);
            }
            _ => panic!("geom_polygon is not a polygon"),
        }
    }
}
