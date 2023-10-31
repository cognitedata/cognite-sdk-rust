use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::time_series::TimestampOrRelative;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyntheticTimeSeriesQuery {
    pub expression: String,
    pub start: Option<TimestampOrRelative>,
    pub end: Option<TimestampOrRelative>,
    pub limit: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyntheticDataValue {
    pub timestamp: i64,
    pub value: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyntheticDataError {
    pub timestamp: i64,
    pub error: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum SyntheticDataPoint {
    Value(SyntheticDataValue),
    Error(SyntheticDataError),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyntheticQueryResponse {
    pub is_string: Option<bool>,
    pub datapoints: Vec<SyntheticDataPoint>,
}
