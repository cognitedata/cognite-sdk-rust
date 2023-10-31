use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

pub mod asset;
pub mod datapoint;
pub mod event;
pub mod files;
pub mod sequences;
pub mod time_serie;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CoreSortOrder {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CoreSortNulls {
    First,
    Last,
    Auto,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoreSortItem {
    pub property: Vec<String>,
    pub order: Option<CoreSortOrder>,
    pub nulls: Option<CoreSortNulls>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum GeoLocation {
    Feature(Feature),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    geometry: GeoLocationGeometry,
    properties: Option<HashMap<String, Value>>,
}

type Point = [f64; 2];
type LineString = Vec<Point>;
type LinearRing = Vec<Point>;
type Polygon = Vec<LinearRing>;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum GeoLocationGeometry {
    Point(GeometryItem<Point>),
    LineString(GeometryItem<LineString>),
    Polygon(GeometryItem<Polygon>),
    MultiPoint(GeometryItem<Vec<Point>>),
    MultiLineString(GeometryItem<Vec<LineString>>),
    MultiPolygon(GeometryItem<Vec<Polygon>>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeometryItem<T> {
    pub coordinates: T,
}
