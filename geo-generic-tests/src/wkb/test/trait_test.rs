use geo_generic_alg::*;
use geo_traits::*;
use geo_traits_ext::{GeoTraitExtWithTypeTag, LineStringTag, PointTag};
use geos::WKBWriter;

use crate::wkb::reader::read_wkb;

use super::data::*;

trait IntersectsGeo<Rhs = Self> {
    fn intersects_geo(&self, rhs: &Rhs) -> bool;
}

trait IntersectsTrait<LM, RM, Rhs = Self> {
    fn intersects_trait(&self, rhs: &Rhs) -> bool;
}

impl<T, P, Rhs> IntersectsTrait<PointTag, PointTag, Rhs> for P
where
    P: PointTrait<T = T>,
    T: CoordNum,
    Rhs: PointTrait<T = T>,
{
    fn intersects_trait(&self, _rhs: &Rhs) -> bool {
        false
    }
}

impl<T, P, Rhs> IntersectsTrait<PointTag, LineStringTag, Rhs> for P
where
    P: PointTrait<T = T>,
    T: CoordNum,
    Rhs: LineStringTrait<T = T>,
{
    fn intersects_trait(&self, _rhs: &Rhs) -> bool {
        false
    }
}

impl<T, LS, Rhs> IntersectsTrait<LineStringTag, PointTag, Rhs> for LS
where
    LS: LineStringTrait<T = T>,
    T: CoordNum,
    Rhs: PointTrait<T = T>,
{
    fn intersects_trait(&self, _rhs: &Rhs) -> bool {
        false
    }
}

impl<T, LS, Rhs> IntersectsTrait<LineStringTag, LineStringTag, Rhs> for LS
where
    LS: LineStringTrait<T = T>,
    T: CoordNum,
    Rhs: LineStringTrait<T = T>,
{
    fn intersects_trait(&self, _rhs: &Rhs) -> bool {
        false
    }
}

impl<LHS, RHS> IntersectsGeo<RHS> for LHS
where
    LHS: GeoTraitExtWithTypeTag,
    RHS: GeoTraitExtWithTypeTag,
    LHS: IntersectsTrait<LHS::Marker, RHS::Marker, RHS>,
{
    fn intersects_geo(&self, rhs: &RHS) -> bool {
        self.intersects_trait(rhs)
    }
}

trait AreaTestTrait<M, T> {
    fn area_test_trait(&self) -> T;
}

impl<T, P> AreaTestTrait<PointTag, T> for P
where
    P: PointTrait<T = T>,
    T: CoordNum,
{
    fn area_test_trait(&self) -> T {
        T::zero()
    }
}

impl<T, LS> AreaTestTrait<LineStringTag, T> for LS
where
    LS: LineStringTrait<T = T>,
    T: CoordNum,
{
    fn area_test_trait(&self) -> T {
        T::one()
    }
}

trait AreaTest<T> {
    fn area_test(&self) -> T;
}

impl<T, G> AreaTest<T> for G
where
    G: GeoTraitExtWithTypeTag,
    G: AreaTestTrait<G::Marker, T>,
    T: CoordNum,
{
    fn area_test(&self) -> T {
        self.area_test_trait()
    }
}

#[cfg(test)]
mod tests {
    use geo_generic_alg::area::AreaTrait;

    use super::*;

    // impl_geo_traits_for_point!(f64, crate::wkb::reader::Point);

    #[test]
    fn test_intersects_trait() {
        let orig = point_2d();
        let buf = geo_to_wkb_geom(orig);
        let wkb = read_wkb(&buf).unwrap();

        let orig2 = linestring_2d();
        let buf2 = geo_to_wkb_geom(orig2);
        let wkb2 = read_wkb(&buf2).unwrap();

        match (wkb.as_type(), wkb2.as_type()) {
            (geo_traits::GeometryType::Point(pt), geo_traits::GeometryType::Point(pt2)) => {
                let area = pt.area_test();
                println!("area: {}", area);
                let area = pt.area_test();
                println!("area: {}", area);

                let intersects = pt.intersects_trait(pt2);
                assert_eq!(intersects, false);

                let intersects = pt.intersects_geo(pt2);
                assert_eq!(intersects, false);
            }
            (geo_traits::GeometryType::Point(pt), geo_traits::GeometryType::LineString(ls)) => {
                let area = ls.area_test_trait();
                println!("area: {}", area);
                let area = ls.area_test();
                println!("area: {}", area);

                let intersects = pt.intersects_trait(ls);
                assert_eq!(intersects, false);

                let intersects = pt.intersects_geo(ls);
                assert_eq!(intersects, false);
            }
            (geo_traits::GeometryType::LineString(ls), geo_traits::GeometryType::Point(pt)) => {
                let intersects = ls.intersects_trait(pt);
                assert_eq!(intersects, false);

                let intersects = ls.intersects_geo(pt);
                assert_eq!(intersects, false);
            }
            (
                geo_traits::GeometryType::LineString(ls),
                geo_traits::GeometryType::LineString(ls2),
            ) => {
                let intersects = ls.intersects_trait(ls2);
                assert_eq!(intersects, false);

                let intersects = ls.intersects_geo(ls2);
                assert_eq!(intersects, false);
            }
            _ => panic!("Expected a Point"),
        }
    }

    #[test]
    fn test_point_trait() {
        let orig = point_2d();
        let buf = geo_to_wkb_geom(orig);
        let wkb = read_wkb(&buf).unwrap();
        assert_eq!(wkb.dim(), geo_traits::Dimensions::Xy);

        let geom_trait = wkb.as_type();
        match geom_trait {
            geo_traits::GeometryType::Point(pt) => {
                let coord = pt.coord().unwrap();
                assert_eq!(coord.x(), orig.0.x);
                assert_eq!(coord.y(), orig.0.y);

                // coord.bounding_rect();
            }
            _ => panic!("Expected a Point"),
        }
    }

    #[test]
    fn test_line_string_trait() {
        let orig = linestring_2d();
        let buf = geo_to_wkb_geom(orig.clone());
        let wkb = read_wkb(&buf).unwrap();
        assert_eq!(wkb.dim(), geo_traits::Dimensions::Xy);

        let area = wkb.signed_area_trait();
        println!("area: {}", area);

        match wkb.as_type() {
            geo_traits::GeometryType::LineString(ls) => {
                assert_eq!(ls.num_coords(), orig.0.len());
                let area = ls.signed_area_trait();
                println!("area: {}", area);
            }
            _ => panic!("Expected a LineString"),
        }
    }

    #[test]
    fn test_polygon_trait() {
        let orig = polygon_2d();
        let buf = geo_to_wkb_geom(orig.clone());
        let wkb = read_wkb(&buf).unwrap();
        assert_eq!(wkb.dim(), geo_traits::Dimensions::Xy);

        let area = wkb.signed_area_trait();
        println!("area: {}", area);

        match wkb.as_type() {
            geo_traits::GeometryType::Polygon(p) => {
                assert_eq!(p.exterior().unwrap().num_coords(), orig.exterior().0.len());
                let area = p.signed_area_trait();
                println!("area: {}", area);
            }
            _ => panic!("Expected a Polygon"),
        }
    }

    #[test]
    fn test_geometry_collection_trait() {
        let orig = geometry_collection_2d();
        let buf = geo_to_wkb_geom(orig.clone());
        let wkb = read_wkb(&buf).unwrap();
        assert_eq!(wkb.dim(), geo_traits::Dimensions::Xy);

        match wkb.as_type() {
            geo_traits::GeometryType::GeometryCollection(gc) => {
                assert_eq!(gc.num_geometries(), orig.0.len());

                gc.geometries().into_iter().zip(orig.0.iter()).for_each(
                    |(geom, orig_geom)| match (geom.as_type(), orig_geom) {
                        (geo_traits::GeometryType::Point(pt), Geometry::Point(orig_pt)) => {
                            assert_eq!(pt.coord().unwrap().x(), orig_pt.0.x);
                            assert_eq!(pt.coord().unwrap().y(), orig_pt.0.y);
                        }
                        (
                            geo_traits::GeometryType::LineString(ls),
                            Geometry::LineString(orig_ls),
                        ) => {
                            assert_eq!(ls.num_coords(), orig_ls.0.len());
                        }
                        (geo_traits::GeometryType::Polygon(p), Geometry::Polygon(orig_p)) => {
                            assert_eq!(
                                p.exterior().unwrap().num_coords(),
                                orig_p.exterior().0.len()
                            );
                        }
                        (
                            geo_traits::GeometryType::MultiPoint(mp),
                            Geometry::MultiPoint(orig_mp),
                        ) => {
                            assert_eq!(mp.num_points(), orig_mp.0.len());
                        }
                        (
                            geo_traits::GeometryType::MultiLineString(mls),
                            Geometry::MultiLineString(orig_mls),
                        ) => {
                            assert_eq!(mls.num_line_strings(), orig_mls.0.len());
                        }
                        (
                            geo_traits::GeometryType::MultiPolygon(mp),
                            Geometry::MultiPolygon(orig_mp),
                        ) => {
                            assert_eq!(mp.num_polygons(), orig_mp.0.len());
                        }
                        (
                            geo_traits::GeometryType::GeometryCollection(gc),
                            Geometry::GeometryCollection(orig_gc),
                        ) => {
                            assert_eq!(gc.num_geometries(), orig_gc.0.len());
                        }
                        _ => panic!("Expected a Point"),
                    },
                );
            }
            _ => panic!("Expected a GeometryCollection"),
        }
    }

    fn geo_to_wkb_geom<G>(geo: G) -> Vec<u8>
    where
        G: TryInto<geos::Geometry>,
    {
        let geos_geom: geos::Geometry = match geo.try_into() {
            Ok(geos_geom) => geos_geom,
            Err(_) => panic!("Failed to convert to geos::Geometry"),
        };

        let mut wkb_writer = WKBWriter::new().unwrap();
        wkb_writer.write_wkb(&geos_geom).unwrap().into()
    }
}

// impl<T, P, Rhs> IntersectsTrait<PointTraitMarker<T>, Rhs> for P
// where
//     P: PointTrait<T = T>,
//     T: CoordNum,
//     Rhs: PolygonTrait<T = T>,
// {
//     fn intersects_trait(&self, _rhs: &Rhs) -> bool {
//         false
//     }
// }
