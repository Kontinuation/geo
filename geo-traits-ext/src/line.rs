// Extend LineTrait traits for the `geo-traits` crate

use geo_traits::{LineTrait, UnimplementedLine};
use geo_types::{CoordNum, Line};

use crate::{CoordTraitExt, GeoTraitExtWithTypeTag, LineTag};

pub trait LineTraitExt: LineTrait + GeoTraitExtWithTypeTag<Tag = LineTag>
where
    <Self as LineTrait>::T: CoordNum,
{
    type CoordTypeExt<'a>: 'a + CoordTraitExt<T = <Self as LineTrait>::T>
    where
        Self: 'a;

    fn start_ext(&self) -> Self::CoordTypeExt<'_>;
    fn end_ext(&self) -> Self::CoordTypeExt<'_>;
    fn coords_ext(&self) -> [Self::CoordTypeExt<'_>; 2];
}

#[macro_export]
macro_rules! forward_line_trait_ext_funcs {
    () => {
        type CoordTypeExt<'__l_inner>
            = <Self as LineTrait>::CoordType<'__l_inner>
        where
            Self: '__l_inner;

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

impl<T> LineTraitExt for Line<T>
where
    T: CoordNum,
{
    forward_line_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for Line<T> {
    type Tag = LineTag;
}

impl<T> LineTraitExt for &Line<T>
where
    T: CoordNum,
{
    forward_line_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for &Line<T> {
    type Tag = LineTag;
}

impl<T> LineTraitExt for UnimplementedLine<T>
where
    T: CoordNum,
{
    forward_line_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedLine<T> {
    type Tag = LineTag;
}
