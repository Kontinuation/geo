// Geometry type markers

pub trait GeoTypeMarker {}

pub struct CoordTraitExtMarker;
pub struct PointTraitExtMarker;
pub struct LineStringTraitExtMarker;
pub struct PolygonTraitExtMarker;
pub struct MultiPointTraitExtMarker;
pub struct MultiLineStringTraitExtMarker;
pub struct MultiPolygonTraitExtMarker;
pub struct GeometryCollectionTraitExtMarker;
pub struct GeometryTraitExtMarker;
pub struct LineTraitExtMarker;
pub struct RectTraitExtMarker;
pub struct TriangleTraitExtMarker;

impl GeoTypeMarker for CoordTraitExtMarker {}
impl GeoTypeMarker for PointTraitExtMarker {}
impl GeoTypeMarker for LineStringTraitExtMarker {}
impl GeoTypeMarker for PolygonTraitExtMarker {}
impl GeoTypeMarker for MultiPointTraitExtMarker {}
impl GeoTypeMarker for MultiLineStringTraitExtMarker {}
impl GeoTypeMarker for MultiPolygonTraitExtMarker {}
impl GeoTypeMarker for GeometryCollectionTraitExtMarker {}
impl GeoTypeMarker for GeometryTraitExtMarker {}
impl GeoTypeMarker for LineTraitExtMarker {}
impl GeoTypeMarker for RectTraitExtMarker {}
impl GeoTypeMarker for TriangleTraitExtMarker {}

pub trait GeoTraitExtWithTypeMarker {
    type Marker: GeoTypeMarker;
}
