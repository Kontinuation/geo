use core::marker::PhantomData;

use crate::{CoordTrait, Dimensions, UnimplementedCoord};

/// A trait for accessing data from a generic Line.
///
/// A Line is a line segment made up of exactly two [coordinates][CoordTrait].
///
/// # Semantics
///
/// The _interior_ and _boundary_ are defined as with a
/// `LineString` with the two end points.
pub trait LineTrait: Sized {
    /// The coordinate type of this geometry
    type T;

    /// The type of each underlying coordinate, which implements [CoordTrait]
    type CoordType<'a>: 'a + CoordTrait<T = Self::T>
    where
        Self: 'a;

    /// The dimension of this geometry
    fn dim(&self) -> Dimensions;

    /// Access the start coordinate in this Line
    fn start(&self) -> Self::CoordType<'_>;

    /// Access the start coordinate in this Line
    fn end(&self) -> Self::CoordType<'_>;

    /// Access the two underlying coordinates
    fn coords(&self) -> [Self::CoordType<'_>; 2] {
        [self.start(), self.end()]
    }
}

/// An empty struct that implements [LineTrait].
///
/// This can be used as the `LineType` of the `GeometryTrait` by implementations that don't
/// have a Line concept
pub struct UnimplementedLine<T>(PhantomData<T>);

impl<T> LineTrait for UnimplementedLine<T> {
    type T = T;
    type CoordType<'a>
        = UnimplementedCoord<Self::T>
    where
        Self: 'a;

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }

    fn start(&self) -> Self::CoordType<'_> {
        unimplemented!()
    }

    fn end(&self) -> Self::CoordType<'_> {
        unimplemented!()
    }
}
