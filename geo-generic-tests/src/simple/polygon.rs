use delegate::delegate;
use geo_traits::PolygonTrait;
use geo_traits_ext::{forward_polygon_trait_ext_funcs, PolygonTraitExt};
use geo_types::CoordNum;

use super::line_string::SimpleLineString;

pub struct SimplePolygon<T: CoordNum> {
    exterior: SimpleLineString<T>,
}

impl<T: CoordNum> SimplePolygon<T> {
    pub fn new(exterior: SimpleLineString<T>) -> Self {
        Self { exterior }
    }
}

impl<'a, T: CoordNum> PolygonTrait for &'a SimplePolygon<T> {
    type T = T;

    type RingType<'b>
        = &'a SimpleLineString<T>
    where
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xy
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        Some(&self.exterior)
    }

    fn num_interiors(&self) -> usize {
        0
    }

    unsafe fn interior_unchecked(&self, _i: usize) -> Self::RingType<'_> {
        panic!("Polygon has no interiors")
    }
}

impl<T: CoordNum> PolygonTrait for SimplePolygon<T> {
    type T = T;

    type RingType<'a>
        = &'a SimpleLineString<T>
    where
        Self: 'a;

    delegate! {
        to(&self) {
            fn dim(&self) -> geo_traits::Dimensions;
            fn exterior(&self) -> Option<Self::RingType<'_>>;
            fn num_interiors(&self) -> usize;
            unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_>;
        }
    }
}

impl<T: CoordNum> PolygonTraitExt<T> for SimplePolygon<T> {
    type RingTypeExt<'a>
        = &'a SimpleLineString<T>
    where
        Self: 'a;

    forward_polygon_trait_ext_funcs!();
}

impl<'a, T: CoordNum> PolygonTraitExt<T> for &'a SimplePolygon<T> {
    type RingTypeExt<'b>
        = &'a SimpleLineString<T>
    where
        Self: 'b;

    forward_polygon_trait_ext_funcs!();
}

#[cfg(test)]
mod tests {
    use geo_traits::LineStringTrait;

    use crate::simple::coord::SimpleCoord;

    use super::*;

    #[test]
    fn test_polygon_trait() {
        let polygon = SimplePolygon::new(SimpleLineString::new(vec![
            SimpleCoord::new(0.0, 0.0),
            SimpleCoord::new(1.0, 0.0),
            SimpleCoord::new(1.0, 1.0),
            SimpleCoord::new(0.0, 1.0),
            SimpleCoord::new(0.0, 0.0),
        ]));

        assert_eq!(polygon.dim(), geo_traits::Dimensions::Xy);
        assert_eq!(polygon.exterior().unwrap().num_coords(), 5);
        assert_eq!(polygon.num_interiors(), 0);
    }

    #[test]
    fn test_polygon_trait_ref() {
        let polygon = SimplePolygon::new(SimpleLineString::new(vec![
            SimpleCoord::new(0.0, 0.0),
            SimpleCoord::new(1.0, 0.0),
            SimpleCoord::new(1.0, 1.0),
            SimpleCoord::new(0.0, 1.0),
            SimpleCoord::new(0.0, 0.0),
        ]));
        let polygon_ref = &polygon;

        assert_eq!(polygon_ref.dim(), geo_traits::Dimensions::Xy);
        assert_eq!(polygon_ref.exterior().unwrap().num_coords(), 5);
        assert_eq!(polygon_ref.num_interiors(), 0);
    }
}
