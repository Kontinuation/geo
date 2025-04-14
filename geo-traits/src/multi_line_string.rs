use core::marker::PhantomData;

use crate::iterator::MultiLineStringIterator;
use crate::line_string::UnimplementedLineString;
use crate::{Dimensions, LineStringTrait};

/// A trait for accessing data from a generic MultiLineString.
///
/// A MultiLineString is a collection of [`LineString`s][LineStringTrait].
///
/// # Semantics
///
/// The _boundary_ of a MultiLineString is obtained by applying the "mod 2" union rule:
/// A Point is in the boundary of a MultiLineString if it is in the boundaries of an
/// odd number of elements of the MultiLineString.
///
/// The _interior_ of a MultiLineString is the union of the interior, and boundary of
/// the constituent LineStrings, _except_ for the boundary as defined above. In other words,
/// it is the set difference of the boundary from the union of the interior and boundary of
/// the constituents.
///
/// A MultiLineString is _simple_ if and only if all of its elements are simple and the only
/// intersections between any two elements occur at Points that are on the boundaries of
/// both elements. A MultiLineString is _closed_ if all of its elements are closed.
/// The boundary of a closed MultiLineString is always empty.
pub trait MultiLineStringTrait: Sized {
    /// The coordinate type of this geometry
    type T;

    /// The type of each underlying LineString, which implements [LineStringTrait]
    type LineStringType<'a>: 'a + LineStringTrait<T = Self::T>
    where
        Self: 'a;

    /// The dimension of this geometry
    fn dim(&self) -> Dimensions;

    /// An iterator over the LineStrings in this MultiLineString
    fn line_strings(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::LineStringType<'_>> {
        MultiLineStringIterator::new(self, 0, self.num_line_strings())
    }

    /// The number of line_strings in this MultiLineString
    fn num_line_strings(&self) -> usize;

    /// Access to a specified line_string in this MultiLineString
    /// Will return None if the provided index is out of bounds
    fn line_string(&self, i: usize) -> Option<Self::LineStringType<'_>> {
        if i >= self.num_line_strings() {
            None
        } else {
            unsafe { Some(self.line_string_unchecked(i)) }
        }
    }

    /// Access to a specified line_string in this MultiLineString
    ///
    /// # Safety
    ///
    /// Accessing an index out of bounds is UB.
    unsafe fn line_string_unchecked(&self, i: usize) -> Self::LineStringType<'_>;
}

/// An empty struct that implements [MultiLineStringTrait].
///
/// This can be used as the `MultiLineStringType` of the `GeometryTrait` by implementations that
/// don't have a MultiLineString concept
pub struct UnimplementedMultiLineString<T>(PhantomData<T>);

impl<T> MultiLineStringTrait for UnimplementedMultiLineString<T> {
    type T = T;
    type LineStringType<'a>
        = UnimplementedLineString<Self::T>
    where
        Self: 'a;

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }

    fn num_line_strings(&self) -> usize {
        unimplemented!()
    }

    unsafe fn line_string_unchecked(&self, _i: usize) -> Self::LineStringType<'_> {
        unimplemented!()
    }
}
