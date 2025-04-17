// Macros for implementing the generic algorithms for user defined concrete geo types
// that implements geo-traits. These macros should be applied to the user defined geo
// types before applying generic algorithms to the type.

#[macro_export]
macro_rules! impl_geo_traits_for_coord {
    (<$t:ident>, $geo_type:ty) => {
        impl_bounding_rect!(<$t>, $geo_type, CoordTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_bounding_rect!($t, $geo_type, CoordTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_point {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!(<$t>, $geo_type, PointTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, PointTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area!($t, $geo_type, PointTraitMarker);
        impl_bounding_rect!($t, $geo_type, PointTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_line_string {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!(<$t>, $geo_type, LineStringTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, LineStringTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area!($t, $geo_type, LineStringTraitMarker);
        impl_bounding_rect!($t, $geo_type, LineStringTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_polygon {
    (<$t:ident>, $geo_type:ty) => {
        impl_area_float!(<$t>, $geo_type, PolygonTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, PolygonTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area_float!($t, $geo_type, PolygonTraitMarker);
        impl_bounding_rect!($t, $geo_type, PolygonTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_multi_point {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!(<$t>, $geo_type, MultiPointTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, MultiPointTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area!($t, $geo_type, MultiPointTraitMarker);
        impl_bounding_rect!($t, $geo_type, MultiPointTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_multi_line_string {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!(<$t>, $geo_type, MultiLineStringTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, MultiLineStringTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area!($t, $geo_type, MultiLineStringTraitMarker);
        impl_bounding_rect!($t, $geo_type, MultiLineStringTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_multi_polygon {
    (<$t:ident>, $geo_type:ty) => {
        impl_area_float!(<$t>, $geo_type, MultiPolygonTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, MultiPolygonTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area_float!($t, $geo_type, MultiPolygonTraitMarker);
        impl_bounding_rect!($t, $geo_type, MultiPolygonTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_geometry_collection {
    (<$t:ident>, $geo_type:ty) => {
        impl_area_float!(<$t>, $geo_type, GeometryCollectionTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, GeometryCollectionTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area!($t, $geo_type, GeometryCollectionTraitMarker);
        impl_bounding_rect!($t, $geo_type, GeometryCollectionTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_geometry {
    (<$t:ident>, $geo_type:ty) => {
        impl_area_float!(<$t>, $geo_type, GeometryTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, GeometryTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area!($t, $geo_type, GeometryTraitMarker);
        impl_bounding_rect!($t, $geo_type, GeometryTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_line {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!(<$t>, $geo_type, LineTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, LineTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area!($t, $geo_type, LineTraitMarker);
        impl_bounding_rect!($t, $geo_type, LineTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_triangle {
    (<$t:ident>, $geo_type:ty) => {
        impl_area_float!(<$t>, $geo_type, TriangleTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, TriangleTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area_float!($t, $geo_type, TriangleTraitMarker);
        impl_bounding_rect!($t, $geo_type, TriangleTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_traits_for_rect {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!(<$t>, $geo_type, RectTraitMarker);
        impl_bounding_rect!(<$t>, $geo_type, RectTraitMarker);
    };
    ($t:ty, $geo_type:ty) => {
        impl_area!($t, $geo_type, RectTraitMarker);
        impl_bounding_rect!($t, $geo_type, RectTraitMarker);
    };
}
