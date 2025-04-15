use geo_types::CoordNum;

/// GeoTraitTypeMarker is for implementing the same algorithm trait for multiple geo-traits
/// like PointTrait, LineStringTrait, PolygonTrait, etc. If we simply implement the algorithm trait
/// using the most straight forward way, we'll get a compile error because of conflicting
/// implementations.
///
/// For example, if we want to implement the `AreaTrait` for `PointTrait`, `LineStringTrait`,
/// `PolygonTrait`, etc, an ideal way is to write this:
///
/// ```rust
/// pub trait AreaTrait<T: CoordNum> {
///     fn area(&self) -> T;
/// }
///
/// impl<T: CoordNum, P: PointTrait<T = T>> AreaTrait<T> for P {
///     fn area(&self) -> T {
///         todo!()
///     }
/// }
///
/// impl<T: CoordNum, LS: LineStringTrait<T = T>> AreaTrait<T> for LS {
///     fn area(&self) -> T {
///         todo!()
///     }
/// }
/// ```
///
/// But this will get a compile error:
///
/// ```
/// error[E0119]: conflicting implementations of trait `AreaTrait<_>`
///   --> geo-generic-alg/src/algorithm/area.rs:14:1
///   |
/// 8  | impl<T: CoordNum, P: PointTrait<T = T>> AreaTrait<T> for P {
///   | ---------------------------------------------------------- first implementation here
/// ...
/// 14 | impl<T: CoordNum, LS: LineStringTrait<T = T>> AreaTrait<T> for LS {
///   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation
/// ```
///
///
pub trait GeoTraitTypeMarker {
    type T;
}

macro_rules! define_geo_trait_type_marker {
    ($name:ident, $type:ty) => {
        pub struct $name<T: CoordNum> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<T: CoordNum> GeoTraitTypeMarker for $name<T> {
            type T = T;
        }
    };
}

define_geo_trait_type_marker!(PointTraitMarker, PointTrait);
define_geo_trait_type_marker!(LineStringTraitMarker, LineStringTrait);
define_geo_trait_type_marker!(PolygonTraitMarker, PolygonTrait);
define_geo_trait_type_marker!(MultiPointTraitMarker, MultiPointTrait);
define_geo_trait_type_marker!(MultiLineStringTraitMarker, MultiLineStringTrait);
define_geo_trait_type_marker!(MultiPolygonTraitMarker, MultiPolygonTrait);
define_geo_trait_type_marker!(GeometryCollectionTraitMarker, GeometryCollectionTrait);
define_geo_trait_type_marker!(GeometryTraitMarker, GeometryTrait);
