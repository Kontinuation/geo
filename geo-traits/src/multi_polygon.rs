use core::marker::PhantomData;

use crate::iterator::MultiPolygonIterator;
use crate::polygon::UnimplementedPolygon;
use crate::{Dimensions, PolygonTrait};

/// A trait for accessing data from a generic MultiPolygon.
///
/// # Semantics
///
/// The _interior_ and the _boundary_ are the union of the interior and the boundary
/// of the constituent polygons.
///
/// # Validity
///
/// - The interiors of no two constituent polygons may intersect.
///
/// - The boundaries of two (distinct) constituent polygons may only intersect at finitely many points.
///
/// Note that the validity is not enforced, but expected by the operations and predicates
/// that operate on it.
pub trait MultiPolygonTrait: Sized {
    /// The coordinate type of this geometry
    type T;

    /// The type of each underlying Polygon, which implements [PolygonTrait]
    type PolygonType<'a>: 'a + PolygonTrait<T = Self::T>
    where
        Self: 'a;

    /// The dimension of this geometry
    fn dim(&self) -> Dimensions;

    /// An iterator over the Polygons in this MultiPolygon
    fn polygons(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::PolygonType<'_>> {
        MultiPolygonIterator::new(self, 0, self.num_polygons())
    }

    /// The number of polygons in this MultiPolygon
    fn num_polygons(&self) -> usize;

    /// Access to a specified polygon in this MultiPolygon
    /// Will return None if the provided index is out of bounds
    fn polygon(&self, i: usize) -> Option<Self::PolygonType<'_>> {
        if i >= self.num_polygons() {
            None
        } else {
            unsafe { Some(self.polygon_unchecked(i)) }
        }
    }

    /// Access to a specified polygon in this MultiPolygon
    ///
    /// # Safety
    ///
    /// Accessing an index out of bounds is UB.
    unsafe fn polygon_unchecked(&self, i: usize) -> Self::PolygonType<'_>;
}

/// An empty struct that implements [MultiPolygonTrait].
///
/// This can be used as the `MultiPolygonType` of the `GeometryTrait` by implementations that don't
/// have a MultiPolygon concept
pub struct UnimplementedMultiPolygon<T>(PhantomData<T>);

impl<T> MultiPolygonTrait for UnimplementedMultiPolygon<T> {
    type T = T;
    type PolygonType<'a>
        = UnimplementedPolygon<Self::T>
    where
        Self: 'a;

    fn dim(&self) -> Dimensions {
        unimplemented!()
    }

    fn num_polygons(&self) -> usize {
        unimplemented!()
    }

    unsafe fn polygon_unchecked(&self, _i: usize) -> Self::PolygonType<'_> {
        unimplemented!()
    }
}
