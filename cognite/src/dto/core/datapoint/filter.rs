use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Identity;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Datapoint aggregates. See [aggregates](https://developer.cognite.com/dev/concepts/aggregation/)
/// for more details.
pub enum Aggregate {
    /// Average over datpoint values.
    Average,
    /// Maximum datapoint value in the given range.
    Max,
    /// Minimum datapoint value in the given range.
    Min,
    /// Number of datapoints in the given range.
    Count,
    /// Sum of datapoints in the given range.
    Sum,
    /// The interpolated value at the start of each time range.
    Interpolation,
    /// The interpolated value at the start of each time range, treating time series as stepwise.
    StepInterpolation,
    /// The sum of absolute differences between neighboring data points in a period.
    TotalVariation,
    /// The variance of the underlying function when assuming linear or step behavior between data points.
    ContinuousVariance,
    /// The variance of the discrete set of data points, no weighting for density of points in time.
    DiscreteVariance,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
/// Either a timestamp in milliseconds since epoch, or a timestamp on the form
/// N[timeunit]-ago where timeunit is w,d,h,m,s. Example: '2d-ago'
/// gets datapoints that are up to 2 days old. You can also specify time
/// in milliseconds since epoch. Note that for aggregates, the start time is rounded
/// down to a whole granularity unit (in UTC timezone). Daily granularities (d) are
/// rounded to 0:00 AM; hourly granularities (h) to the start of the hour, etc.
pub enum TimestampOrRelative {
    /// Timestamp in milliseconds since epoch.
    Timestamp(i64),
    /// Relative timestamp.
    /// The format is 'now' or `N[timeunit]-ago` where timeunit is `w,d,h,m,s`.
    /// Example: `2d-ago` gets data that is up to two days old.
    /// You can also specify time in milliseconds since epoch.
    Relative(String),
}

impl From<&str> for TimestampOrRelative {
    fn from(value: &str) -> Self {
        Self::Relative(value.to_owned())
    }
}

impl From<i64> for TimestampOrRelative {
    fn from(value: i64) -> Self {
        Self::Timestamp(value)
    }
}

impl From<&str> for Aggregate {
    fn from(val: &str) -> Aggregate {
        serde_json::from_str(val).unwrap()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter on datapoints
pub struct DatapointsFilter {
    /// List of timeseries to query.
    pub items: Vec<DatapointsQuery>,
    /// Get datapoints from, and including, this time.
    pub start: Option<TimestampOrRelative>,
    /// Get datapoints up to, but excluding, this point in time.
    pub end: Option<TimestampOrRelative>,
    /// Returns up to this number of data points. The maximum is 100000
    /// non-aggregated data points and 10000 aggregated data points in
    /// total across all queries in a single request.
    pub limit: Option<u32>,
    /// Specify the aggregates to return.
    pub aggregates: Option<Vec<Aggregate>>,
    /// The time granularity size and unit to aggregate over.
    /// Valid entries are 'day, hour, minute, second', or short forms 'd, h, m, s',
    /// or a multiple of these indicated by a number as a prefix. For 'second' and 'minute',
    /// the multiple must be an integer between 1 and 120 inclusive; for 'hour'
    /// and 'day', the multiple must be an integer between 1 and 100000 inclusive.
    ///
    /// For example, a granularity '5m' means that aggregates are calculated over 5 minutes.
    /// This field is required if aggregates are specified.
    pub granularity: Option<String>,
    /// Whether to include the last data points before the requsted time period and the first
    /// one after.
    pub include_outside_points: Option<bool>,
    /// Ignore IDs and external IDs that are not found
    pub ignore_unknown_ids: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsQuery {
    #[serde(flatten)]
    /// ID or external ID of time series to retrieve data from.
    pub id: Identity,
    /// Get datapoints from, and including, this time.
    pub start: Option<TimestampOrRelative>,
    /// Get datapoints up to, but excluding, this point in time.
    pub end: Option<TimestampOrRelative>,
    /// Returns up to this number of data points. The maximum is 100000
    /// non-aggregated data points and 10000 aggregated data points in
    /// total across all queries in a single request.
    pub limit: Option<u32>,
    /// Specify the aggregates to return.
    pub aggregates: Option<Vec<String>>,
    /// The time granularity size and unit to aggregate over.
    /// Valid entries are 'day, hour, minute, second', or short forms 'd, h, m, s',
    /// or a multiple of these indicated by a number as a prefix. For 'second' and 'minute',
    /// the multiple must be an integer between 1 and 120 inclusive; for 'hour'
    /// and 'day', the multiple must be an integer between 1 and 100000 inclusive.
    ///
    /// For example, a granularity '5m' means that aggregates are calculated over 5 minutes.
    /// This field is required if aggregates are specified.
    pub granularity: Option<String>,
    /// Whether to include the last data points before the requsted time period and the first
    /// one after.
    pub include_outside_points: Option<bool>,
}

impl Default for DatapointsQuery {
    fn default() -> Self {
        Self {
            id: Identity::Id { id: 0 },
            start: Default::default(),
            end: Default::default(),
            limit: Default::default(),
            aggregates: Default::default(),
            granularity: Default::default(),
            include_outside_points: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Query for retrieving the latest datapoint in a time series.
pub struct LatestDatapointsQuery {
    /// Get data points before this time.
    /// The format is 'now' or `N[timeunit]-ago` where timeunit is `w,d,h,m,s`.
    /// Example: `2d-ago` gets data that is up to two days old.
    /// You can also specify time in milliseconds since epoch.
    pub before: String,
    #[serde(flatten)]
    /// ID or external ID of time series to retrieve data from.
    pub id: Identity,
}

impl LatestDatapointsQuery {
    pub fn new(time_serie_id: Identity, before: &str) -> LatestDatapointsQuery {
        LatestDatapointsQuery {
            id: time_serie_id,
            before: String::from(before),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Request for deleting a range of datapoints in a time series.
pub struct DeleteDatapointsQuery {
    /// Inclusive start time, in milliseconds since epoch.
    pub inclusive_begin: i64,
    /// Exclusive end time, in milliseconds since epoch.
    pub exclusive_end: i64,
    #[serde(rename = "id", flatten)]
    /// ID or external ID of time series to retrieve data from.
    pub id: Identity,
}

impl DeleteDatapointsQuery {
    /// Create a query for deleting data points.
    ///
    /// # Arguments
    ///
    /// * `id` - ID or external ID of time series to delete from.
    /// * `inclusive_begin` - Inclusive start time, in milliseconds since epoch.
    /// * `exclusive_end` - Exclusive end time, in milliseconds since epoch.
    pub fn new(id: Identity, inclusive_begin: i64, exclusive_end: i64) -> DeleteDatapointsQuery {
        DeleteDatapointsQuery {
            id,
            inclusive_begin,
            exclusive_end,
        }
    }
}
