use core::marker::PhantomData;

use crate::iterator::LineStringIterator;
use crate::{CoordTrait, Dimensions, UnimplementedCoord};

/// A trait for accessing data from a generic LineString.
///
/// A LineString is an ordered collection of two or more [points][CoordTrait], representing a path
/// between locations.
///
/// # Semantics
///
/// 1. A LineString is _closed_ if it is empty, **or** if the first and last coordinates are the same.
/// 2. The _boundary_ of a LineString is either:
///     - **empty** if it is _closed_ (see **1**) **or**
///     - contains the **start** and **end** coordinates.
/// 3. The _interior_ is the (infinite) set of all coordinates along the LineString, _not including_ the boundary.
/// 4. A LineString is _simple_ if it does not intersect except **optionally** at the first and last coordinates (in which case it is also _closed_, see **1**).
/// 5. A _simple_ **and** _closed_ LineString is a `LinearRing` as defined in the OGC-SFA.
///
/// # Validity
///
/// A LineString is valid if it is either empty or contains 2 or more coordinates.
///
/// Further, a closed LineString **must not** self-intersect. Note that its validity is **not** enforced,
/// and operations and predicates are **undefined** on invalid LineStrings.
pub trait LineStringTrait: Sized {
    /// The coordinate type of this geometry
    type T;

    /// The type of each underlying coordinate, which implements [CoordTrait]
    type CoordType<'a>: 'a + CoordTrait<T = Self::T>
    where
        Self: 'a;

    /// The dimension of this geometry
    fn dim(&self) -> Dimensions;

    /// An iterator over the coordinates in this LineString
    fn coords(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::CoordType<'_>> {
        LineStringIterator::new(self, 0, self.num_coords())
    }

    /// The number of coordinates in this LineString
    fn num_coords(&self) -> usize;

    /// Access to a specified coordinate in this LineString
    /// Will return None if the provided index is out of bounds
    #[inline]
    fn coord(&self, i: usize) -> Option<Self::CoordType<'_>> {
        if i >= self.num_coords() {
            None
        } else {
            unsafe { Some(self.coord_unchecked(i)) }
        }
    }

    /// Access to a specified coordinate in this LineString
    ///
    /// # Safety
    ///
    /// Accessing an index out of bounds is UB.
    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_>;
}

/// An empty struct that implements [LineStringTrait].
///
/// This can be used as the `LineStringType` of the `GeometryTrait` by implementations that don't
/// have a LineString concept
pub struct UnimplementedLineString<T>(PhantomData<T>);

impl<T> LineStringTrait for UnimplementedLineString<T> {
    type T = T;
    type CoordType<'a>
        = UnimplementedCoord<Self::T>
    where
        Self: 'a;

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }

    fn num_coords(&self) -> usize {
        unimplemented!()
    }

    unsafe fn coord_unchecked(&self, _i: usize) -> Self::CoordType<'_> {
        unimplemented!()
    }
}
