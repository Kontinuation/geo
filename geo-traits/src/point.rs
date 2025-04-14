use core::marker::PhantomData;

use crate::{CoordTrait, Dimensions, UnimplementedCoord};

/// A trait for accessing data from a generic Point.
///
/// # Semantics
///
/// The _interior_ of the point is itself (a singleton set),
/// and its _boundary_ is empty. A point is _valid_ if and
/// only if the `Coord` is valid.
pub trait PointTrait {
    /// The coordinate type of this geometry
    type T;

    /// The type of the underlying coordinate, which implements [CoordTrait]
    type CoordType<'a>: 'a + CoordTrait<T = Self::T>
    where
        Self: 'a;

    /// Dimensions of the coordinate tuple
    fn dim(&self) -> Dimensions;

    /// The location of this 0-dimensional geometry.
    ///
    /// According to Simple Features, a Point can have zero coordinates and be considered "empty".
    fn coord(&self) -> Option<Self::CoordType<'_>>;
}

/// An empty struct that implements [PointTrait].
///
/// This can be used as the `PointType` of the `GeometryTrait` by implementations that don't have a
/// Point concept
pub struct UnimplementedPoint<T>(PhantomData<T>);

impl<T> PointTrait for UnimplementedPoint<T> {
    type T = T;
    type CoordType<'a>
        = UnimplementedCoord<Self::T>
    where
        Self: 'a;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        unimplemented!()
    }

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }
}
