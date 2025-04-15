use core::marker::PhantomData;

use crate::Dimensions;

/// A trait for accessing data from a generic Coord.
///
/// # Semantics
///
/// This type does not represent any geospatial primitive,
/// but is used in their definitions. The only requirement
/// is that the coordinates it contains are valid numbers
/// (for eg. not `f64::NAN`).
pub trait CoordTrait: PartialEq {
    /// The coordinate type of this geometry
    type T;

    /// Dimensions of the coordinate tuple
    fn dim(&self) -> Dimensions;

    /// Access the n'th (0-based) element of the CoordinateTuple.
    /// Returns `None` if `n >= DIMENSION`.
    ///
    /// See also [`nth_or_panic()`](Self::nth_or_panic) and [`nth_unchecked()`](Self::nth_unchecked).
    ///
    /// # Panics
    ///
    /// This method may panic if [`dim()`](Self::dim) does not correspond to
    /// the actual number of dimensions in this coordinate.
    fn nth(&self, n: usize) -> Option<Self::T> {
        if n < self.dim().size() {
            Some(self.nth_or_panic(n))
        } else {
            None
        }
    }

    /// x component of this coord.
    fn x(&self) -> Self::T;

    /// y component of this coord.
    fn y(&self) -> Self::T;

    /// Returns a tuple that contains the x/horizontal & y/vertical component of the coord.
    fn x_y(&self) -> (Self::T, Self::T) {
        (self.x(), self.y())
    }

    /// Access the n'th (0-based) element of the CoordinateTuple.
    /// May panic if n >= DIMENSION.
    /// See also [`nth()`](Self::nth).
    fn nth_or_panic(&self, n: usize) -> Self::T;

    /// Access the n'th (0-based) element of the CoordinateTuple.
    /// May panic if n >= DIMENSION.
    ///
    /// See also [`nth()`](Self::nth), [`nth_or_panic()`](Self::nth_or_panic).
    ///
    /// You might want to override the default implementation of this method
    /// if you can provide a more efficient implementation.
    ///
    /// # Safety
    ///
    /// Though it may panic, the default implementation actually is safe. However, implementors
    /// are allowed to implement this method themselves with an unsafe implementation. See the
    /// individual implementations for more information on their own Safety considerations.
    unsafe fn nth_unchecked(&self, n: usize) -> Self::T {
        self.nth_or_panic(n)
    }
}

impl<T: Copy + PartialEq> CoordTrait for (T, T) {
    type T = T;

    fn nth_or_panic(&self, n: usize) -> Self::T {
        match n {
            0 => self.x(),
            1 => self.y(),
            _ => panic!("(T, T) only supports 2 dimensions"),
        }
    }

    fn dim(&self) -> Dimensions {
        Dimensions::Xy
    }

    fn x(&self) -> Self::T {
        self.0
    }

    fn y(&self) -> Self::T {
        self.1
    }
}

/// An empty struct that implements [CoordTrait].
///
/// This can be used as the `CoordType` of the `GeometryTrait` by implementations that don't have a
/// Coord concept
pub struct UnimplementedCoord<T>(PhantomData<T>);

impl<T> PartialEq for UnimplementedCoord<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> CoordTrait for UnimplementedCoord<T> {
    type T = T;

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }

    fn nth_or_panic(&self, _n: usize) -> Self::T {
        unimplemented!()
    }

    fn x(&self) -> Self::T {
        unimplemented!()
    }

    fn y(&self) -> Self::T {
        unimplemented!()
    }
}
