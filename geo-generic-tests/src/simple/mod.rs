pub mod alg_test;
pub mod coord;
pub mod geometry;
pub mod line_string;
pub mod point;
pub mod polygon;

use coord::SimpleCoord;
use geometry::SimpleGeometry;
use line_string::SimpleLineString;
use point::SimplePoint;
use polygon::SimplePolygon;

use geo_generic_alg::area::*;
use geo_generic_alg::bounding_rect::*;
use geo_generic_alg::*;

impl_geo_traits_for_coord!(<T>, SimpleCoord<T>);
impl_geo_traits_for_point!(<T>, SimplePoint<T>);
impl_geo_traits_for_line_string!(<T>, SimpleLineString<T>);
impl_geo_traits_for_polygon!(<T>, SimplePolygon<T>);
impl_geo_traits_for_geometry!(<T>, SimpleGeometry<T>);
