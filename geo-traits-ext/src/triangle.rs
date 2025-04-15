// Extend TriangleTrait traits for the `geo-traits` crate

use geo_traits::TriangleTrait;
use geo_types::{polygon, to_geo::ToGeoCoord, CoordNum, Line, Polygon};

pub trait TriangleTraitExt<T: CoordNum>: TriangleTrait<T = T> {
    fn to_array(&self) -> [Self::CoordType<'_>; 3] {
        [self.first(), self.second(), self.third()]
    }

    fn to_lines(&self) -> [Line<T>; 3] {
        [
            Line::new(self.first().to_coord(), self.second().to_coord()),
            Line::new(self.second().to_coord(), self.third().to_coord()),
            Line::new(self.third().to_coord(), self.first().to_coord()),
        ]
    }

    fn to_polygon(&self) -> Polygon<T> {
        polygon![
            self.first().to_coord(),
            self.second().to_coord(),
            self.third().to_coord(),
            self.first().to_coord(),
        ]
    }
}

impl<T, TT> TriangleTraitExt<T> for TT
where
    TT: TriangleTrait<T = T>,
    T: CoordNum,
{
}
