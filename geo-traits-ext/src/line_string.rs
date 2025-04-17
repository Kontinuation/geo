// Extend LineStringTrait traits for the `geo-traits` crate

use geo_traits::LineStringTrait;
use geo_types::to_geo::ToGeoCoord;
use geo_types::{Coord, CoordNum, Line, Triangle};

pub trait LineStringTraitExt<T: CoordNum>: LineStringTrait<T = T> {
    /// Return an iterator yielding one [`Line`] for each line segment
    /// in the [`LineString`][`geo_types::LineString`].
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{wkt, Line, LineString};
    ///
    /// let line_string = wkt!(LINESTRING(0 0,5 0,7 9));
    /// let mut lines = line_string.lines();
    ///
    /// assert_eq!(
    ///     Some(Line::new((0, 0), (5, 0))),
    ///     lines.next()
    /// );
    /// assert_eq!(
    ///     Some(Line::new((5, 0), (7, 9))),
    ///     lines.next()
    /// );
    /// assert!(lines.next().is_none());
    /// ```
    fn lines(&'_ self) -> impl ExactSizeIterator<Item = Line<T>> + '_ {
        let num_coords = self.num_coords();
        (0..num_coords - 1).map(|i| unsafe {
            let coord1 = self.coord_unchecked(i);
            let coord2 = self.coord_unchecked(i + 1);
            Line::new(coord1.to_coord(), coord2.to_coord())
        })
    }

    /// Return an iterator yielding one [`Line`] for each line segment in the [`LineString`][`geo_types::LineString`],
    /// starting from the **end** point of the LineString, working towards the start.
    ///
    /// Note: This is like [`Self::lines`], but the sequence **and** the orientation of
    /// segments are reversed.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{wkt, Line, LineString};
    ///
    /// let line_string = wkt!(LINESTRING(0 0,5 0,7 9));
    /// let mut lines = line_string.rev_lines();
    ///
    /// assert_eq!(
    ///     Some(Line::new((7, 9), (5, 0))),
    ///     lines.next()
    /// );
    /// assert_eq!(
    ///     Some(Line::new((5, 0), (0, 0))),
    ///     lines.next()
    /// );
    /// assert!(lines.next().is_none());
    /// ```
    fn rev_lines(&'_ self) -> impl ExactSizeIterator<Item = Line<T>> + '_ {
        let num_coords = self.num_coords();
        (num_coords - 1..0).map(|i| unsafe {
            let coord1 = self.coord_unchecked(i);
            let coord2 = self.coord_unchecked(i - 1);
            Line::new(coord2.to_coord(), coord1.to_coord())
        })
    }

    /// An iterator which yields the coordinates of a [`LineString`][`geo_types::LineString`] as [Triangle]s
    fn triangles(&'_ self) -> impl ExactSizeIterator<Item = Triangle<T>> + '_ {
        let num_coords = self.num_coords();
        (0..num_coords - 2).map(|i| unsafe {
            let coord1 = self.coord_unchecked(i);
            let coord2 = self.coord_unchecked(i + 1);
            let coord3 = self.coord_unchecked(i + 2);
            Triangle::new(coord1.to_coord(), coord2.to_coord(), coord3.to_coord())
        })
    }

    fn coord_iter(&self) -> impl Iterator<Item = Coord<T>> {
        self.coords().map(|c| c.to_coord())
    }

    fn is_closed(&self) -> bool {
        match (self.coords().next(), self.coords().last()) {
            (Some(first), Some(last)) => first.to_coord() == last.to_coord(),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<LS, T> LineStringTraitExt<T> for LS
where
    LS: LineStringTrait<T = T>,
    T: CoordNum,
{
}
