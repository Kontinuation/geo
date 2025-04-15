//! Extend CoordTrait traits for the `geo-traits` crate

use geo_traits::CoordTrait;
use geo_types::CoordNum;

pub trait CoordTraitExt<T: CoordNum>: CoordTrait<T = T> {
    fn to_coord(&self) -> geo_types::Coord<Self::T> {
        geo_types::Coord {
            x: self.x(),
            y: self.y(),
        }
    }
}

impl<C, T> CoordTraitExt<T> for C
where
    C: CoordTrait<T = T>,
    T: CoordNum,
{
}
