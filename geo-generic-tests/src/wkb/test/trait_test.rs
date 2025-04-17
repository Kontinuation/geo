use geo_traits::{CoordTrait, LineStringTrait, PointTrait};
use geos::WKBWriter;

use crate::wkb::reader::read_wkb;

use super::data::*;

#[cfg(test)]
mod tests {
    use geo_traits::{GeometryTrait, PolygonTrait};

    use super::*;

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

        match wkb.as_type() {
            geo_traits::GeometryType::LineString(ls) => {
                assert_eq!(ls.num_coords(), orig.0.len());
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

        match wkb.as_type() {
            geo_traits::GeometryType::Polygon(p) => {
                assert_eq!(p.exterior().unwrap().num_coords(), orig.exterior().0.len());
            }
            _ => panic!("Expected a Polygon"),
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
