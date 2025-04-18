// Extend RectTrait traits for the `geo-traits` crate

use geo_traits::{CoordTrait, RectTrait, UnimplementedCoord, UnimplementedRect};
use geo_types::{coord, Coord, CoordNum, Line, LineString, Polygon, Rect};

use crate::{CoordTraitExt, GeoTraitExtWithTypeMarker, RectTraitExtMarker};

static RECT_INVALID_BOUNDS_ERROR: &str = "Failed to create Rect: 'min' coordinate's x/y value must be smaller or equal to the 'max' x/y value";

pub trait RectTraitExt<T: CoordNum>: RectTrait<T = T> + GeoTraitExtWithTypeMarker {
    type CoordTypeExt<'a>: 'a + CoordTraitExt<T>
    where
        Self: 'a;

    fn min_ext(&self) -> Self::CoordTypeExt<'_>;
    fn max_ext(&self) -> Self::CoordTypeExt<'_>;

    fn width(&self) -> T {
        self.max().x() - self.min().x()
    }

    fn height(&self) -> T {
        self.max().y() - self.min().y()
    }

    fn to_polygon(&self) -> Polygon<T>
    where
        T: Clone,
    {
        let min_coord = self.min();
        let max_coord = self.max();

        let min_x = min_coord.x();
        let min_y = min_coord.y();
        let max_x = max_coord.x();
        let max_y = max_coord.y();

        let line_string = LineString::new(vec![
            Coord { x: min_x, y: min_y },
            Coord { x: min_x, y: max_y },
            Coord { x: max_x, y: max_y },
            Coord { x: max_x, y: min_y },
            Coord { x: min_x, y: min_y },
        ]);

        Polygon::new(line_string, vec![])
    }

    fn to_lines(&self) -> [Line<T>; 4] {
        [
            Line::new(
                coord! {
                    x: self.max().x(),
                    y: self.min().y(),
                },
                coord! {
                    x: self.max().x(),
                    y: self.max().y(),
                },
            ),
            Line::new(
                coord! {
                    x: self.max().x(),
                    y: self.max().y(),
                },
                coord! {
                    x: self.min().x(),
                    y: self.max().y(),
                },
            ),
            Line::new(
                coord! {
                    x: self.min().x(),
                    y: self.max().y(),
                },
                coord! {
                    x: self.min().x(),
                    y: self.min().y(),
                },
            ),
            Line::new(
                coord! {
                    x: self.min().x(),
                    y: self.min().y(),
                },
                coord! {
                    x: self.max().x(),
                    y: self.min().y(),
                },
            ),
        ]
    }

    fn to_line_string(&self) -> LineString<T>
    where
        T: Clone,
    {
        let min_coord = self.min();
        let max_coord = self.max();

        let min_x = min_coord.x();
        let min_y = min_coord.y();
        let max_x = max_coord.x();
        let max_y = max_coord.y();

        LineString::new(vec![
            Coord { x: min_x, y: min_y },
            Coord { x: min_x, y: max_y },
            Coord { x: max_x, y: max_y },
            Coord { x: max_x, y: min_y },
            Coord { x: min_x, y: min_y },
        ])
    }

    fn has_valid_bounds(&self) -> bool {
        let min_coord = self.min();
        let max_coord = self.max();
        min_coord.x() <= max_coord.x() && min_coord.y() <= max_coord.y()
    }

    fn assert_valid_bounds(&self) {
        if !self.has_valid_bounds() {
            panic!("{}", RECT_INVALID_BOUNDS_ERROR);
        }
    }

    fn contains_point(&self, coord: &Coord<T>) -> bool
    where
        T: PartialOrd,
    {
        let min_coord = self.min();
        let max_coord = self.max();

        let min_x = min_coord.x();
        let min_y = min_coord.y();
        let max_x = max_coord.x();
        let max_y = max_coord.y();

        (min_x <= coord.x && coord.x <= max_x) && (min_y <= coord.y && coord.y <= max_y)
    }

    fn contains_rect(&self, rect: &Self) -> bool
    where
        T: PartialOrd,
    {
        let self_min = self.min();
        let self_max = self.max();
        let other_min = rect.min();
        let other_max = rect.max();

        let self_min_x = self_min.x();
        let self_min_y = self_min.y();
        let self_max_x = self_max.x();
        let self_max_y = self_max.y();

        let other_min_x = other_min.x();
        let other_min_y = other_min.y();
        let other_max_x = other_max.x();
        let other_max_y = other_max.y();

        (self_min_x <= other_min_x && other_max_x <= self_max_x)
            && (self_min_y <= other_min_y && other_max_y <= self_max_y)
    }
}

#[macro_export]
macro_rules! forward_rect_trait_ext_funcs {
    () => {
        fn min_ext(&self) -> Self::CoordTypeExt<'_> {
            <Self as RectTrait>::min(self)
        }

        fn max_ext(&self) -> Self::CoordTypeExt<'_> {
            <Self as RectTrait>::max(self)
        }
    };
}

impl<T> RectTraitExt<T> for Rect<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'a>
        = Coord<T>
    where
        Self: 'a;

    forward_rect_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for Rect<T> {
    type Marker = RectTraitExtMarker;
}

impl<'a, T> RectTraitExt<T> for &'a Rect<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'b>
        = Coord<T>
    where
        Self: 'b;

    forward_rect_trait_ext_funcs!();
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeMarker for &'a Rect<T> {
    type Marker = RectTraitExtMarker;
}

impl<T> RectTraitExt<T> for UnimplementedRect<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'a>
        = UnimplementedCoord<T>
    where
        Self: 'a;

    forward_rect_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeMarker for UnimplementedRect<T> {
    type Marker = RectTraitExtMarker;
}
