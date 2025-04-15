// Extend RectTrait traits for the `geo-traits` crate

use geo_traits::{CoordTrait, RectTrait};
use geo_types::{coord, polygon, CoordNum, Line, Polygon};

static RECT_INVALID_BOUNDS_ERROR: &str = "Failed to create Rect: 'min' coordinate's x/y value must be smaller or equal to the 'max' x/y value";

pub trait RectTraitExt<T: CoordNum>: RectTrait<T = T> {
    fn width(&self) -> T {
        self.max().x() - self.min().x()
    }

    fn height(&self) -> T {
        self.max().y() - self.min().y()
    }

    fn to_polygon(&self) -> Polygon<T> {
        polygon![
            (x: self.max().x(), y: self.min().y()),
            (x: self.max().x(), y: self.max().y()),
            (x: self.min().x(), y: self.max().y()),
            (x: self.min().x(), y: self.min().y()),
            (x: self.max().x(), y: self.min().y()),
        ]
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

    fn has_valid_bounds(&self) -> bool {
        self.min().x() <= self.max().x() && self.min().y() <= self.max().y()
    }

    fn assert_valid_bounds(&self) {
        if !self.has_valid_bounds() {
            panic!("{}", RECT_INVALID_BOUNDS_ERROR);
        }
    }
}

impl<R, T> RectTraitExt<T> for R
where
    R: RectTrait<T = T>,
    T: CoordNum,
{
}
