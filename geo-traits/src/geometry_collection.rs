use core::marker::PhantomData;

use crate::iterator::GeometryCollectionIterator;
use crate::{Dimensions, GeometryTrait, UnimplementedGeometry};

/// A trait for accessing data from a generic GeometryCollection.
///
/// A GeometryCollection is a collection of [Geometry][GeometryTrait] types.
pub trait GeometryCollectionTrait: Sized {
    /// The coordinate type of this geometry
    type T;

    /// The type of each underlying geometry, which implements [GeometryTrait]
    type GeometryType<'a>: 'a + GeometryTrait<T = Self::T>
    where
        Self: 'a;

    /// The dimension of this geometry
    fn dim(&self) -> Dimensions;

    /// An iterator over the geometries in this GeometryCollection
    fn geometries(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::GeometryType<'_>> {
        GeometryCollectionIterator::new(self, 0, self.num_geometries())
    }

    /// The number of geometries in this GeometryCollection
    fn num_geometries(&self) -> usize;

    /// Access to a specified geometry in this GeometryCollection
    /// Will return None if the provided index is out of bounds
    fn geometry(&self, i: usize) -> Option<Self::GeometryType<'_>> {
        if i >= self.num_geometries() {
            None
        } else {
            unsafe { Some(self.geometry_unchecked(i)) }
        }
    }

    /// Access to a specified geometry in this GeometryCollection
    ///
    /// # Safety
    ///
    /// Accessing an index out of bounds is UB.
    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_>;
}

/// An empty struct that implements [GeometryCollectionTrait].
///
/// This can be used as the `GeometryCollectionType` of the `GeometryTrait` by implementations that
/// don't have a GeometryCollection concept
pub struct UnimplementedGeometryCollection<T>(PhantomData<T>);

impl<T> GeometryCollectionTrait for UnimplementedGeometryCollection<T> {
    type T = T;
    type GeometryType<'a>
        = UnimplementedGeometry<Self::T>
    where
        Self: 'a;

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }

    fn num_geometries(&self) -> usize {
        unimplemented!()
    }

    unsafe fn geometry_unchecked(&self, _i: usize) -> Self::GeometryType<'_> {
        unimplemented!()
    }
}
