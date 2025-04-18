#[cfg(test)]
mod tests {
    use crate::simple::{
        coord::SimpleCoord, line_string::SimpleLineString, point::SimplePoint,
        polygon::SimplePolygon,
    };
    use geo_generic_alg::*;
    use geo_traits::PointTrait;
    use geo_types::to_geo::ToGeoCoord;

    #[test]
    fn test_point_alg() {
        let point = SimplePoint::new(1.0, 2.0);
        let area = point.signed_area();
        assert_eq!(area, 0.0);
        let rect = point.bounding_rect();
        assert_eq!(rect.min(), point.coord().unwrap().to_coord());
        assert_eq!(rect.max(), point.coord().unwrap().to_coord());

        let point_ref = &point;
        let area = point_ref.signed_area();
        assert_eq!(area, 0.0);
        let rect = point_ref.bounding_rect();
        assert_eq!(rect.min(), point.coord().unwrap().to_coord());
        assert_eq!(rect.max(), point.coord().unwrap().to_coord());
    }

    #[test]
    fn test_polygon_alg() {
        let polygon = SimplePolygon::new(SimpleLineString::new(vec![
            SimpleCoord::new(0.0, 0.0),
            SimpleCoord::new(1.0, 0.0),
            SimpleCoord::new(1.0, 1.0),
            SimpleCoord::new(0.0, 1.0),
            SimpleCoord::new(0.0, 0.0),
        ]));
        let area = polygon.signed_area();
        assert_eq!(area, 1.0);

        let rect = polygon.bounding_rect().unwrap();
        assert_eq!(rect.min().x, 0.0);
        assert_eq!(rect.min().y, 0.0);
        assert_eq!(rect.max().x, 1.0);
        assert_eq!(rect.max().y, 1.0);
    }
}
