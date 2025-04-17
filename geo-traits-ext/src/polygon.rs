// Extend PolygonTrait traits for the `geo-traits` crate

// use core::iter::{empty, Empty};

// use geo_traits::{LineStringTrait, PolygonTrait};
// use geo_types::{to_geo::ToGeoCoord, Coord, CoordNum};

// use crate::LineStringTraitExt;

use geo_traits::PolygonTrait;
use geo_types::CoordNum;

pub trait PolygonTraitExt<T: CoordNum>: PolygonTrait<T = T> {
    // fn exterior_coord_iter(&'_ self) -> impl Iterator<Item = Coord<T>> + '_ {
    //     // match self.exterior() {
    //     //     None => todo!(),
    //     //     Some(exterior) => {
    //     //         exterior.coord_iter()
    //     //     }
    //     // }
    //     // let exterior = self.exterior().unwrap();
    //     // exterior.coord_iter()
    //     self.exterior().unwrap().coord_iter()
    // }
}

impl<P, T> PolygonTraitExt<T> for P
where
    P: PolygonTrait<T = T>,
    T: CoordNum,
{
}
