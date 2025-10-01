use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::time_series::TimestampOrRelative;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Query for synthetic time series data points.
pub struct SyntheticTimeSeriesQuery {
    /// query definition. For limits, see the
    /// [guide to synthetic time series](https://developer.cognite.com/dev/concepts/resource_types/synthetic_timeseries.html#limits).
    pub expression: String,
    /// Get datapoints starting from, and including, this time.
    pub start: Option<TimestampOrRelative>,
    /// Get datapoints up to, but excluding, this time.
    pub end: Option<TimestampOrRelative>,
    /// Return up to this number of datapoints.
    pub limit: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Synthetic data point.
pub struct SyntheticDataValue {
    /// Timestamp in milliseconds since epoch.
    pub timestamp: i64,
    /// Data point value.
    pub value: f64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Synthetic data error.
pub struct SyntheticDataError {
    /// Timestamp in milliseconds since epoch.
    pub timestamp: i64,
    /// Error that occured when computing synthetic data point.
    pub error: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Synthetic data point or error.
pub enum SyntheticDataPoint {
    /// A synthetic value.
    Value(SyntheticDataValue),
    /// A computation error.
    Error(SyntheticDataError),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Response when querying synthetic data points.
pub struct SyntheticQueryResponse {
    /// Whether the results are strings. Currently, this is always false.
    pub is_string: Option<bool>,
    /// List of computed data points.
    pub datapoints: Vec<SyntheticDataPoint>,
}
