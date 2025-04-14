use core::marker::PhantomData;

use crate::{CoordTrait, Dimensions, UnimplementedCoord};

/// A trait for accessing data from a generic Triangle.
///
/// A triangle is a bounded area whose three vertices are defined by [coordinates][CoordTrait].
///
/// # Semantics and Validity
///
/// The semantics and validity are that of the equivalent [`PolygonTrait`][`crate::PolygonTrait`];
/// in addition, the three vertices **must not** be collinear and they *must* be distinct.
///
/// Irrespective of input order the resulting geometry has counter-clockwise (ccw) order
/// and its vertices are yielded in ccw order by iterators.
pub trait TriangleTrait: Sized {
    /// The coordinate type of this geometry
    type T;

    /// The type of each underlying coordinate, which implements [CoordTrait]
    type CoordType<'a>: 'a + CoordTrait<T = Self::T>
    where
        Self: 'a;

    /// The dimension of this geometry
    fn dim(&self) -> Dimensions;

    /// Access the first coordinate in this Triangle
    fn first(&self) -> Self::CoordType<'_>;

    /// Access the second coordinate in this Triangle
    fn second(&self) -> Self::CoordType<'_>;

    /// Access the third coordinate in this Triangle
    fn third(&self) -> Self::CoordType<'_>;

    /// Access the three underlying coordinates
    fn coords(&self) -> [Self::CoordType<'_>; 3] {
        [self.first(), self.second(), self.third()]
    }
}

/// An empty struct that implements [TriangleTrait].
///
/// This can be used as the `TriangleType` of the `GeometryTrait` by implementations that don't
/// have a Triangle concept
pub struct UnimplementedTriangle<T>(PhantomData<T>);

impl<T> TriangleTrait for UnimplementedTriangle<T> {
    type T = T;
    type CoordType<'a>
        = UnimplementedCoord<Self::T>
    where
        Self: 'a;

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }

    fn first(&self) -> Self::CoordType<'_> {
        unimplemented!()
    }

    fn second(&self) -> Self::CoordType<'_> {
        unimplemented!()
    }

    fn third(&self) -> Self::CoordType<'_> {
        unimplemented!()
    }
}
