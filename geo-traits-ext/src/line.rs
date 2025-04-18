// Extend LineTrait traits for the `geo-traits` crate

use geo_traits::{LineTrait, UnimplementedLine};
use geo_types::{CoordNum, Line};

use crate::{CoordTraitExt, GeoTraitExtWithTypeTag, LineTag};

pub trait LineTraitExt<T: CoordNum>: LineTrait<T = T> + GeoTraitExtWithTypeTag {
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

impl<T> LineTraitExt<T> for Line<T>
where
    T: CoordNum,
{
    forward_line_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for Line<T> {
    type Marker = LineTag;
}

impl<'a, T> LineTraitExt<T> for &'a Line<T>
where
    T: CoordNum,
{
    forward_line_trait_ext_funcs!();
}

impl<'a, T: CoordNum> GeoTraitExtWithTypeTag for &'a Line<T> {
    type Marker = LineTag;
}

impl<T> LineTraitExt<T> for UnimplementedLine<T>
where
    T: CoordNum,
{
    forward_line_trait_ext_funcs!();
}

impl<T: CoordNum> GeoTraitExtWithTypeTag for UnimplementedLine<T> {
    type Marker = LineTag;
}
