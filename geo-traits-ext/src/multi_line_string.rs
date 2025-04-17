// Extend MultiLineStringTrait traits for the `geo-traits` crate

use geo_traits::{LineStringTrait, MultiLineStringTrait};
use geo_types::to_geo::ToGeoCoord;
use geo_types::{Coord, CoordNum};

pub trait MultiLineStringTraitExt<T: CoordNum>: MultiLineStringTrait<T = T> {
    fn coord_iter(&self) -> impl Iterator<Item = Coord<T>> {
        CoordIter::new(self)
    }
}

struct CoordIter<'a, T: CoordNum, MLS: MultiLineStringTrait<T = T>> {
    multi_line_string: &'a MLS,
    current_line_string: Option<MLS::LineStringType<'a>>,
    idx_line_string: usize,
    idx_coord: usize,
}

impl<'a, T: CoordNum, MLS: MultiLineStringTrait<T = T>> CoordIter<'a, T, MLS> {
    fn new(multi_line_string: &'a MLS) -> Self {
        let current_line_string = multi_line_string.line_string(0);
        Self {
            multi_line_string,
            current_line_string,
            idx_line_string: 0,
            idx_coord: 0,
        }
    }
}

impl<'a, T: CoordNum, MLS: MultiLineStringTrait<T = T>> Iterator for CoordIter<'a, T, MLS> {
    type Item = Coord<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_line_string.is_none() {
            return None;
        }

        let ls = self.current_line_string.as_ref().unwrap();
        if self.idx_coord >= ls.num_coords() {
            // Load the next line string
            while self.idx_line_string < self.multi_line_string.num_line_strings() {
                self.idx_line_string += 1;
                self.idx_coord = 0;
                self.current_line_string = self.multi_line_string.line_string(self.idx_line_string);
                if self.current_line_string.is_some() {
                    break;
                }
            }
            self.next()
        } else {
            // Load the next coordinate
            let mut coord = None;
            while coord.is_none() && self.idx_coord < ls.num_coords() {
                coord = ls.coord(self.idx_coord);
                self.idx_coord += 1;
            }
            coord.map(|c| c.to_coord()).or_else(|| self.next())
        }
    }
}

impl<T, MLS> MultiLineStringTraitExt<T> for MLS
where
    T: CoordNum,
    MLS: MultiLineStringTrait<T = T>,
{
}
