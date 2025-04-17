// Extend MultiPointTrait traits for the `geo-traits` crate

use geo_traits::{MultiPointTrait, PointTrait};
use geo_types::{to_geo::ToGeoCoord, Coord, CoordNum};

pub trait MultiPointTraitExt<T: CoordNum>: MultiPointTrait<T = T> {
    fn coord_iter(&self) -> impl DoubleEndedIterator<Item = Coord<T>> {
        self.points().flat_map(|p| p.coord().map(|c| c.to_coord()))
    }
}

impl<T, MP> MultiPointTraitExt<T> for MP
where
    T: CoordNum,
    MP: MultiPointTrait<T = T>,
{
}
