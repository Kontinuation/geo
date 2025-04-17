// Extend LineTrait traits for the `geo-traits` crate

use geo_traits::{LineTrait, UnimplementedCoord, UnimplementedLine};
use geo_types::{Coord, CoordNum, Line};

use crate::CoordTraitExt;

pub trait LineTraitExt<T: CoordNum>: LineTrait<T = T> {
    type CoordTypeExt<'a>: 'a + CoordTraitExt<T>
    where
        Self: 'a;

    fn start_ext(&self) -> Self::CoordTypeExt<'_>;
    fn end_ext(&self) -> Self::CoordTypeExt<'_>;
    fn coords_ext(&self) -> [Self::CoordTypeExt<'_>; 2];
}

#[macro_export]
macro_rules! forward_line_trait_ext_funcs {
    () => {
        fn start_ext(&self) -> Self::CoordTypeExt<'_> {
            <Self as LineTrait>::start(self)
        }

        fn end_ext(&self) -> Self::CoordTypeExt<'_> {
            <Self as LineTrait>::end(self)
        }

        fn coords_ext(&self) -> [Self::CoordTypeExt<'_>; 2] {
            [self.start_ext(), self.end_ext()]
        }
    };
}

impl<T> LineTraitExt<T> for Line<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'a>
        = &'a Coord<T>
    where
        Self: 'a;
    forward_line_trait_ext_funcs!();
}

impl<'a, T> LineTraitExt<T> for &'a Line<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'b>
        = &'a Coord<T>
    where
        Self: 'b;
    forward_line_trait_ext_funcs!();
}

impl<T> LineTraitExt<T> for UnimplementedLine<T>
where
    T: CoordNum,
{
    type CoordTypeExt<'a>
        = UnimplementedCoord<T>
    where
        Self: 'a;
    forward_line_trait_ext_funcs!();
}
