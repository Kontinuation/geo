// Extend LineStringTrait traits for the `geo-traits` crate

use geo_traits::{LineStringTrait, UnimplementedLineString};
use geo_types::to_geo::ToGeoCoord;
use geo_types::{Coord, CoordNum, Line, LineString, Triangle};

use crate::{CoordTraitExt, GeoTraitExtWithTypeTag, LineStringTag};

pub trait LineStringTraitExt: LineStringTrait + GeoTraitExtWithTypeTag
where
    <Self as LineStringTrait>::T: CoordNum,
{
    type CoordTypeExt<'a>: 'a + CoordTraitExt<T = <Self as LineStringTrait>::T>
    where
        Self: 'a;

    fn coord_ext(&self, i: usize) -> Option<Self::CoordTypeExt<'_>>;
    fn coord_unchecked_ext(&self, i: usize) -> Self::CoordTypeExt<'_>;
    fn coords_ext(&self) -> impl Iterator<Item = Self::CoordTypeExt<'_>>;

    /// Return an iterator yielding one [`Line`] for each line segment
    /// in the [`LineString`][`geo_types::LineString`].
    fn lines(&'_ self) -> impl ExactSizeIterator<Item = Line<<Self as LineStringTrait>::T>> + '_ {
        let num_coords = self.num_coords();
        (0..num_coords.saturating_sub(1)).map(|i| unsafe {
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
    fn rev_lines(
        &'_ self,
    ) -> impl ExactSizeIterator<Item = Line<<Self as LineStringTrait>::T>> + '_ {
        let num_coords = self.num_coords();
        (num_coords - 1..0).map(|i| unsafe {
            let coord1 = self.coord_unchecked(i);
            let coord2 = self.coord_unchecked(i - 1);
            Line::new(coord2.to_coord(), coord1.to_coord())
        })
    }

    /// An iterator which yields the coordinates of a [`LineString`][`geo_types::LineString`] as [Triangle]s
    fn triangles(
        &'_ self,
    ) -> impl ExactSizeIterator<Item = Triangle<<Self as LineStringTrait>::T>> + '_ {
        let num_coords = self.num_coords();
        (0..num_coords - 2).map(|i| unsafe {
            let coord1 = self.coord_unchecked(i);
            let coord2 = self.coord_unchecked(i + 1);
            let coord3 = self.coord_unchecked(i + 2);
            Triangle::new(coord1.to_coord(), coord2.to_coord(), coord3.to_coord())
        })
    }

    fn coord_iter(&self) -> impl Iterator<Item = Coord<<Self as LineStringTrait>::T>> {
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

#[macro_export]
macro_rules! forward_line_string_trait_ext_funcs {
    () => {
        type CoordTypeExt<'__l_inner>
            = <Self as LineStringTrait>::CoordType<'__l_inner>
        where
            Self: '__l_inner;

        fn coord_ext(&self, i: usize) -> Option<Self::CoordTypeExt<'_>> {
            <Self as LineStringTrait>::coord(self, i)
        }

        fn coord_unchecked_ext(&self, i: usize) -> Self::CoordTypeExt<'_> {
            unsafe { <Self as LineStringTrait>::coord_unchecked(self, i) }
        }

        fn coords_ext(&self) -> impl Iterator<Item = Self::CoordTypeExt<'_>> {
            <Self as LineStringTrait>::coords(self)
        }
    };
}

impl<T> LineStringTraitExt for LineString<T>
where
    T: CoordNum,
{
    forward_line_string_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for LineString<T> {
    type Tag = LineStringTag;
}

impl<T> LineStringTraitExt for &LineString<T>
where
    T: CoordNum,
{
    forward_line_string_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for &LineString<T> {
    type Tag = LineStringTag;
}

impl<T> LineStringTraitExt for UnimplementedLineString<T>
where
    T: CoordNum,
{
    forward_line_string_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedLineString<T> {
    type Tag = LineStringTag;
}
