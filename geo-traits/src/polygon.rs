use core::marker::PhantomData;

use crate::iterator::PolygonInteriorIterator;
use crate::line_string::UnimplementedLineString;
use crate::{Dimensions, LineStringTrait};

/// A trait for accessing data from a generic Polygon.
///
/// A `Polygon`’s outer boundary (_exterior ring_) is represented by a
/// [`LineString`][LineStringTrait]. It may contain zero or more holes (_interior rings_), also
/// represented by `LineString`s.
///
/// # Semantics
///
/// The _boundary_ of the polygon is the union of the boundaries of the exterior and interiors.
/// The interior is all the points inside the polygon (not on the boundary).
///
/// The `Polygon` structure guarantees that all exterior and interior rings will
/// be _closed_, such that the first and last coordinate of each ring has
/// the same value.
///
/// # Validity
///
/// - The exterior and interior rings must be valid `LinearRing`s (see [`LineStringTrait`]).
///
/// - No two rings in the boundary may cross, and may intersect at a `Point` only as a tangent.
///   In other words, the rings must be distinct, and for every pair of common points in two
///   of the rings, there must be a neighborhood (a topological open set) around one that
///   does not contain the other point.
///
/// - The closure of the interior of the `Polygon` must equal the `Polygon` itself.
///   For instance, the exterior may not contain a spike.
///
/// - The interior of the polygon must be a connected point-set. That is, any two distinct
///   points in the interior must admit a curve between these two that lies in the interior.
pub trait PolygonTrait: Sized {
    /// The coordinate type of this geometry
    type T;

    /// The type of each underlying ring, which implements [LineStringTrait]
    type RingType<'a>: 'a + LineStringTrait<T = Self::T>
    where
        Self: 'a;

    /// The dimension of this geometry
    fn dim(&self) -> Dimensions;

    /// The exterior ring of the polygon
    fn exterior(&self) -> Option<Self::RingType<'_>>;

    /// An iterator of the interior rings of this Polygon
    fn interiors(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::RingType<'_>> {
        PolygonInteriorIterator::new(self, 0, self.num_interiors())
    }

    /// The number of interior rings in this Polygon
    fn num_interiors(&self) -> usize;

    /// Access to a specified interior ring in this Polygon
    /// Will return None if the provided index is out of bounds
    fn interior(&self, i: usize) -> Option<Self::RingType<'_>> {
        if i >= self.num_interiors() {
            None
        } else {
            unsafe { Some(self.interior_unchecked(i)) }
        }
    }

    /// Access to a specified interior ring in this Polygon
    ///
    /// # Safety
    ///
    /// Accessing an index out of bounds is UB.
    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_>;
}

/// An empty struct that implements [PolygonTrait].
///
/// This can be used as the `PolygonType` of the `GeometryTrait` by implementations that don't have a
/// Polygon concept
pub struct UnimplementedPolygon<T>(PhantomData<T>);

impl<T> PolygonTrait for UnimplementedPolygon<T> {
    type T = T;
    type RingType<'a>
        = UnimplementedLineString<Self::T>
    where
        Self: 'a;

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        unimplemented!()
    }

    fn num_interiors(&self) -> usize {
        unimplemented!()
    }

    unsafe fn interior_unchecked(&self, _i: usize) -> Self::RingType<'_> {
        unimplemented!()
    }
}
