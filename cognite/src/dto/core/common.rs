use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Sort order for sorting core resources.
pub enum CoreSortOrder {
    /// Ascending
    Asc,
    /// Descending.
    Desc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Where to sort nulls.
pub enum CoreSortNulls {
    /// Nulls first,
    First,
    /// Nulls last
    Last,
    /// Translates to `First` for `Asc` and `Last` for `Desc`.
    Auto,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Sort by a specific property
pub struct CoreSortItem {
    /// Property to sort by.
    pub property: Vec<String>,
    /// Sort order. Defaults to `Asc`.
    pub order: Option<CoreSortOrder>,
    /// Null behavior. Defaults to `Auto`.
    pub nulls: Option<CoreSortNulls>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
/// Geographic metadata.
pub enum GeoLocation {
    /// GeoJson Feature.
    Feature(Feature),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A geographical feature.
pub struct Feature {
    /// Represents the points, curves and surfaces in the coordinate space.
    pub geometry: GeoLocationGeometry,
    /// Additional properties in a String key -> Object value format.
    pub properties: Option<HashMap<String, Value>>,
}
/// Coordinates of a point in 2D space, described as an array of 2 numbers.
pub type Point = [f64; 2];
/// Coordinates of a line described by a list of two or more points.
/// Each point is defined as a pair of two numbers in an array,
/// representing coordinates of a point in 2D space.
pub type LineString = Vec<Point>;
/// A linear ring is the boundary of a surface or the boundary of a hole in a surface.
/// It is defined as a list consisting of 4 or more Points,
/// where the first and last Point is equivalent.
pub type LinearRing = Vec<Point>;
/// List of one or more linear rings representing a shape.
pub type Polygon = Vec<LinearRing>;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
/// Geometry variants.
pub enum GeoLocationGeometry {
    /// Points in 2D space.
    Point(GeometryItem<Point>),
    /// A list of two or more points.
    LineString(GeometryItem<LineString>),
    /// A collection of linear rings that form a polygon.
    Polygon(GeometryItem<Polygon>),
    /// A set of multiple points.
    MultiPoint(GeometryItem<Vec<Point>>),
    /// A set of multiple lines.
    MultiLineString(GeometryItem<Vec<LineString>>),
    /// A set of multiple polygons.
    MultiPolygon(GeometryItem<Vec<Polygon>>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A geometry item.
pub struct GeometryItem<T> {
    /// Some form of coordinates.
    pub coordinates: T,
}
