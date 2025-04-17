// Macros for implementing the generic algorithms for user defined concrete geo types
// that implements geo-traits. These macros should be applied to the user defined geo
// types before applying generic algorithms to the type.

#[macro_export]
macro_rules! impl_geo_trait_coord {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, CoordTraitMarker);
        impl_bounding_rect!($t, $geo_type, CoordTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_point {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, PointTraitMarker);
        impl_bounding_rect!($t, $geo_type, PointTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_line_string {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, LineStringTraitMarker);
        impl_bounding_rect!($t, $geo_type, LineStringTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_polygon {
    (<$t:ident>, $geo_type:ty) => {
        impl_area_float!($t, $geo_type, PolygonTraitMarker);
        impl_bounding_rect!($t, $geo_type, PolygonTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_multi_point {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, MultiPointTraitMarker);
        impl_bounding_rect!($t, $geo_type, MultiPointTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_multi_line_string {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, MultiLineStringTraitMarker);
        impl_bounding_rect!($t, $geo_type, MultiLineStringTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_multi_polygon {
    (<$t:ident>, $geo_type:ty) => {
        impl_area_float!($t, $geo_type, MultiPolygonTraitMarker);
        impl_bounding_rect!($t, $geo_type, MultiPolygonTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_geometry_collection {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, GeometryCollectionTraitMarker);
        impl_bounding_rect!($t, $geo_type, GeometryCollectionTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_geometry {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, GeometryTraitMarker);
        impl_bounding_rect!($t, $geo_type, GeometryTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_line {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, LineTraitMarker);
        impl_bounding_rect!($t, $geo_type, LineTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_triangle {
    (<$t:ident>, $geo_type:ty) => {
        impl_area_float!($t, $geo_type, TriangleTraitMarker);
        impl_bounding_rect!($t, $geo_type, TriangleTraitMarker);
    };
}

#[macro_export]
macro_rules! impl_geo_trait_rect {
    (<$t:ident>, $geo_type:ty) => {
        impl_area!($t, $geo_type, RectTraitMarker);
        impl_bounding_rect!($t, $geo_type, RectTraitMarker);
    };
}
