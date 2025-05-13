use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{AdvancedFilter, PropertyIdentifier};

use super::LastUpdatedTimeFilter;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Object used to define the fixed bounds of a histogram.
pub struct Bounds<T> {
    /// Lower bound of the histogram.
    pub min: Option<T>,
    /// Upper bound of the histogram.
    pub max: Option<T>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A simple aggregate on a property.
pub struct SimpleAggregate {
    /// Property to aggregate over.
    pub property: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An aggregate that returns unique values of a property.
pub struct UniqueValuesAggregate {
    /// Property to aggregate over.
    pub property: Vec<String>,
    /// Nested aggregates, to further group the unique values.
    pub aggregates: Option<HashMap<String, RecordsAggregate>>,
    /// The number of top buckets returned. The default limit is 10 items.
    pub size: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// This aggregate is used to create a histogram of numeric values
/// from the specified property.
pub struct NumberHistogramAggregate {
    /// Property to aggregate over.
    pub property: Vec<String>,
    /// To limit the range of buckets in the histogram.
    /// It is particularly useful in the case of open data ranges
    /// that can result in a very large number of buckets.
    /// One or both bounds must be specified.
    pub hard_bounds: Option<Bounds<f64>>,
    /// The interval between each bucket.
    pub interval: f64,
    /// Nested aggregates, to further group the histogram values.
    pub aggregates: Option<HashMap<String, RecordsAggregate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Calendar interval between each bucket.
pub enum CalendarInterval {
    #[serde(rename = "1y")]
    /// 1 year
    Year,
    #[serde(rename = "1q")]
    /// 1 quarter
    Quarter,
    #[serde(rename = "1M")]
    /// 1 month
    Month,
    #[serde(rename = "1w")]
    /// 1 week
    Week,
    #[serde(rename = "1d")]
    /// 1 day
    Day,
    #[serde(rename = "1h")]
    /// 1 hour
    Hour,
    #[serde(rename = "1m")]
    /// 1 minute
    Minute,
    #[serde(rename = "1s")]
    /// 1 second
    Second,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The interval between each bucket in a time histogram.
pub enum TimeHistogramInterval {
    /// Calendar interval
    CalendarInterval(CalendarInterval),
    /// Fixed interval, as a duration, e.g. 3m, 400h, 25d, etc.
    FixedInterval(String),
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A time histogram aggregator function. This function will generate a histogram
/// from the values of the specified property. It uses the specified
/// calendar or fixed interval.
pub struct TimeHistogramAggregate {
    /// Property to aggregate over.
    pub property: Vec<String>,
    #[serde(flatten)]
    /// The interval between each bucket.
    pub interval: TimeHistogramInterval,
    /// To limit the range of buckets in the histogram.
    /// It is particularly useful in the case of open data ranges
    /// that can result in a very large number of buckets.
    /// One or both bounds must be specified.
    pub hard_bounds: Option<Bounds<String>>,
    /// Nested aggregates, to further group the histogram values.
    pub aggregates: Option<HashMap<String, RecordsAggregate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]

/// The moving function to use in the moving function aggregate.
pub enum MovingFunction {
    #[serde(rename = "MovingFunctions.max")]
    /// The maximum value in the window.
    Max,
    #[serde(rename = "MovingFunctions.min")]
    /// The minimum value in the window.
    Min,
    #[serde(rename = "MovingFunctions.sum")]
    /// The sum of the values in the window.
    Sum,
    #[serde(rename = "MovingFunctions.unweightedAvg")]
    /// The unweighted average of the values in the window.
    UnweightedAvg,
    #[serde(rename = "MovingFunctions.linearWeightedAvg")]
    /// The linear weighted average of the values in the window.
    LinearWeightedAvg,
    /// Some other function.
    #[serde(untagged)]
    Other(String),
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Given an ordered series of data, the Moving Function aggregation will slide
/// a window across the data and allow the user to specify a function
/// that is executed on each window of data. A number of common functions
/// are predefined such as min/max, moving averages, etc.
/// Customer defined functions are not allowed now.
/// The aggregate must be embedded inside of a numberHistogram or
/// timeHistogram aggregate. It can be embedded like any other metric aggregate.
pub struct MovingFunctionAggregate {
    /// The path to the buckets to use for the moving function.
    /// Syntax is [AggregateName][MultiBucketKey]?(>[AggregateName])*.
    /// See documentation for more details.
    pub buckets_path: String,
    /// The size of window to slide accross the histogram.
    pub window: u32,
    /// The function that should be executed on each window of data.
    pub function: MovingFunction,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A multi-bucket aggregation where each bucket contains the records that matches
/// a filter in the filters list.
///
/// Note: only 30 seuch buckets are allowed accross all filter aggregates in a
/// single request.
pub struct FilterAggregate {
    /// List of filters to describe each bucket.
    pub filters: Vec<AdvancedFilter>,
    /// Nested aggregates, to further group the filter values.
    pub aggregates: Option<HashMap<String, RecordsAggregate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Aggregates on records.
pub enum RecordsAggregate {
    /// Average aggregate.
    Avg(SimpleAggregate),
    /// Count aggregate.
    Count(SimpleAggregate),
    /// Minimum aggregate.
    Min(SimpleAggregate),
    /// Maximum aggregate.
    Max(SimpleAggregate),
    /// Sum aggregate.
    Sum(SimpleAggregate),
    /// Unique values aggregate.
    UniqueValues(UniqueValuesAggregate),
    /// Number histogram aggregate.
    NumberHistogram(NumberHistogramAggregate),
    /// Time histogram aggregate.
    TimeHistogram(TimeHistogramAggregate),
    /// Moving function aggregate.
    MovingFunction(MovingFunctionAggregate),
    /// Filter aggregate.
    Filters(FilterAggregate),
}

impl RecordsAggregate {
    /// Create an average aggregate on the specified property.
    pub fn average(property: impl PropertyIdentifier) -> Self {
        RecordsAggregate::Avg(SimpleAggregate {
            property: property.into_identifier(),
        })
    }

    /// Create a count aggregate on the specified property.
    pub fn count(property: impl PropertyIdentifier) -> Self {
        RecordsAggregate::Count(SimpleAggregate {
            property: property.into_identifier(),
        })
    }

    /// Create a min aggregate on the specified property.
    pub fn min(property: impl PropertyIdentifier) -> Self {
        RecordsAggregate::Min(SimpleAggregate {
            property: property.into_identifier(),
        })
    }

    /// Create a max aggregate on the specified property.
    pub fn max(property: impl PropertyIdentifier) -> Self {
        RecordsAggregate::Max(SimpleAggregate {
            property: property.into_identifier(),
        })
    }

    /// Create a sum aggregate on the specified property.
    pub fn sum(property: impl PropertyIdentifier) -> Self {
        RecordsAggregate::Sum(SimpleAggregate {
            property: property.into_identifier(),
        })
    }

    /// Create a unique values aggregate on the specified property.
    ///
    /// * `size` is the maximum number of unique values to return, default is 10.
    /// * `aggregates` is a map of nested aggregates to further group the unique values.
    pub fn unique_values(
        property: impl PropertyIdentifier,
        size: Option<u32>,
        aggregates: Option<HashMap<String, RecordsAggregate>>,
    ) -> Self {
        RecordsAggregate::UniqueValues(UniqueValuesAggregate {
            property: property.into_identifier(),
            size,
            aggregates,
        })
    }

    /// Create a number histogram aggregate on the specified property.
    ///
    /// * `interval` is the interval between each bucket.
    /// * `hard_bounds` is the fixed bounds of the histogram.
    /// * `aggregates` is a map of nested aggregates to further group the histogram values.
    pub fn number_histogram(
        property: impl PropertyIdentifier,
        interval: f64,
        hard_bounds: Option<Bounds<f64>>,
        aggregates: Option<HashMap<String, RecordsAggregate>>,
    ) -> Self {
        RecordsAggregate::NumberHistogram(NumberHistogramAggregate {
            property: property.into_identifier(),
            hard_bounds,
            interval,
            aggregates,
        })
    }

    /// Create a time histogram aggregate on the specified property.
    ///
    /// * `interval` is the interval between each bucket.
    /// * `hard_bounds` is the fixed bounds of the histogram.
    /// * `aggregates` is a map of nested aggregates to further group the histogram values.
    pub fn time_histogram(
        property: impl PropertyIdentifier,
        interval: TimeHistogramInterval,
        hard_bounds: Option<Bounds<String>>,
        aggregates: Option<HashMap<String, RecordsAggregate>>,
    ) -> Self {
        RecordsAggregate::TimeHistogram(TimeHistogramAggregate {
            property: property.into_identifier(),
            interval,
            hard_bounds,
            aggregates,
        })
    }

    /// Create a moving function aggregate.
    ///
    /// * `buckets_path` is the path to the buckets to use for the moving function.
    /// * `window` is the size of window to slide across the histogram.
    /// * `function` is the function that should be executed on each window of data.
    pub fn moving_function(
        buckets_path: impl Into<String>,
        window: u32,
        function: MovingFunction,
    ) -> Self {
        RecordsAggregate::MovingFunction(MovingFunctionAggregate {
            buckets_path: buckets_path.into(),
            window,
            function,
        })
    }

    /// Create a filter aggregate.
    ///
    /// * `filters` is a list of filters to describe each bucket.
    /// * `aggregates` is a map of nested aggregates to further group the filter values.
    pub fn filters(
        filters: Vec<AdvancedFilter>,
        aggregates: Option<HashMap<String, RecordsAggregate>>,
    ) -> Self {
        RecordsAggregate::Filters(FilterAggregate {
            filters,
            aggregates,
        })
    }
}

/// Request for aggregates on records.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordsAggregateRequest {
    /// Matches records with the last updated time within the provided range.
    /// This attribute is mandatory, and the maximum interval it can define
    /// is limited by the stream settings.
    pub last_updated_time: LastUpdatedTimeFilter,
    /// A custom filter to apply to the records to include in the aggregate result.
    pub filter: Option<AdvancedFilter>,
    /// A dictionary of requested aggregates with client defined names/identifiers.
    pub aggregates: HashMap<String, RecordsAggregate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
/// The value of a multivalued aggregate.
pub enum AggregateValue {
    /// A string value
    String(String),
    /// An integer value
    Integer(i64),
    /// A float value
    Number(f64),
    /// A boolean value
    Boolean(bool),
}

impl From<String> for AggregateValue {
    fn from(value: String) -> Self {
        AggregateValue::String(value)
    }
}

impl<'a> From<&'a str> for AggregateValue {
    fn from(value: &'a str) -> Self {
        AggregateValue::String(value.to_string())
    }
}
impl From<i64> for AggregateValue {
    fn from(value: i64) -> Self {
        AggregateValue::Integer(value)
    }
}
impl From<f64> for AggregateValue {
    fn from(value: f64) -> Self {
        AggregateValue::Number(value)
    }
}
impl From<bool> for AggregateValue {
    fn from(value: bool) -> Self {
        AggregateValue::Boolean(value)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A bucket of unique values in an aggregate.
pub struct UniqueValuesBucket {
    /// The number of items with the given value.
    pub count: u64,
    /// The unique value of this bucket.
    pub value: AggregateValue,
    /// The nested aggregates for this bucket.
    pub aggregates: Option<HashMap<String, AggregateResult>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A bucket of a histogram aggregate.
pub struct HistogramBucket<T> {
    /// The number of values in this bucket.
    pub count: u64,
    /// The lower bound of this bucket.
    pub interval_start: T,
    /// The nested aggregates for this bucket.
    pub aggregates: Option<HashMap<String, AggregateResult>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A bucket of a filter aggregate.
pub struct FilterBucket {
    /// The number of values in this bucket.
    pub count: u64,
    /// The nested aggregates for this bucket.
    pub aggregates: Option<HashMap<String, AggregateResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
/// A bucket of an aggregate.
pub enum AggregateBuckets {
    /// A bucket of unique values.
    UniqueValues(Vec<UniqueValuesBucket>),
    /// A bucket of a histogram aggregate.
    NumberHistogram(Vec<HistogramBucket<f64>>),
    /// A bucket of a time histogram aggregate.
    TimeHistogram(Vec<HistogramBucket<String>>),
    /// A bucket of a filter aggregate.
    Filter(Vec<FilterBucket>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result of an aggregate.
pub enum AggregateResult {
    /// An average aggregate.
    Avg(f64),
    /// A count aggregate.
    Count(u64),
    /// A minimum aggregate.
    Min(f64),
    /// A maximum aggregate.
    Max(f64),
    /// A sum aggregate.
    Sum(f64),
    /// Buckets of a unique values or histogram aggregate.
    Buckets(AggregateBuckets),
    /// A moving function aggregate.
    MovingFunction(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result of a request for aggregates.
pub struct RecordsAggregateResult {
    /// Retrieved aggregates, by user defined names.
    pub aggregates: HashMap<String, AggregateResult>,
}
