#[cfg(test)]
mod tests {
    use crate::simple::point::SimplePoint;
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
}
