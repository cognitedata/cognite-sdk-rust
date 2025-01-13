mod filter;
#[allow(clippy::all)]
#[allow(missing_docs)]
#[path = "datapoint/generated/com.cognite.v1.timeseries.proto.rs"]
mod proto;
mod status_code;

use std::convert::TryFrom;

pub use self::filter::*;
pub use self::proto::data_point_insertion_item::DatapointType as InsertDatapointType;
pub use self::proto::data_point_insertion_item::TimeSeriesReference;
pub use self::proto::data_point_list_item::DatapointType as ListDatapointType;
pub use self::proto::*;
pub use self::status_code::*;

use serde::{de::Error, Deserialize, Serialize};
use serde_json::Value;

use crate::Identity;
use crate::IdentityOrInstance;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
/// Enumeration over different types of retrieved data points.
pub enum DatapointsEnumType {
    /// Datapoints with double precision floating point values.
    NumericDatapoints(Vec<DatapointDouble>),
    /// Datapoints with string values.
    StringDatapoints(Vec<DatapointString>),
    /// Aggregate data points.
    AggregateDatapoints(Vec<DatapointAggregate>),
}

impl From<Vec<DatapointDouble>> for DatapointsEnumType {
    fn from(value: Vec<DatapointDouble>) -> Self {
        Self::NumericDatapoints(value)
    }
}

impl From<Vec<DatapointString>> for DatapointsEnumType {
    fn from(value: Vec<DatapointString>) -> Self {
        Self::StringDatapoints(value)
    }
}

impl From<Vec<DatapointAggregate>> for DatapointsEnumType {
    fn from(value: Vec<DatapointAggregate>) -> Self {
        Self::AggregateDatapoints(value)
    }
}

impl DatapointsEnumType {
    /// Get self as numeric datapoints, or none if a different type.
    pub fn numeric(self) -> Option<Vec<DatapointDouble>> {
        match self {
            Self::NumericDatapoints(x) => Some(x),
            _ => None,
        }
    }
    /// Get self as string datapoints, or none if a different type.
    pub fn string(self) -> Option<Vec<DatapointString>> {
        match self {
            Self::StringDatapoints(x) => Some(x),
            _ => None,
        }
    }
    /// Get self as aggregate datapoints, or none if a different type.
    pub fn aggregate(self) -> Option<Vec<DatapointAggregate>> {
        match self {
            Self::AggregateDatapoints(x) => Some(x),
            _ => None,
        }
    }
}

/* #[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A data point status code.
pub struct StatusCode {
    /// Status code numeric representation.
    pub code: Option<i64>,
    /// Status code symbol.
    pub symbol: Option<String>,
}

impl StatusCode {
    /// Create a new status code from a given symbol.
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: Some(symbol.into()),
            code: None,
        }
    }

    /// Create a new status code from a numeric code.
    pub fn new_code(code: i64) -> Self {
        Self {
            code: Some(code),
            symbol: None,
        }
    }
} */

impl From<Status> for StatusCode {
    fn from(value: Status) -> Self {
        if value.code != 0 {
            StatusCode::try_from(value.code).unwrap_or(StatusCode::Invalid)
        } else if !value.symbol.is_empty() {
            StatusCode::try_parse(&value.symbol).unwrap_or(StatusCode::Invalid)
        } else {
            StatusCode::Good
        }
    }
}

impl From<StatusCode> for Status {
    fn from(code: StatusCode) -> Status {
        Status {
            code: code.bits() as i64,
            symbol: String::new(),
        }
    }
}

mod cdf_double_serde {
    use core::f64;

    use serde::{de::Visitor, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<f64>, D::Error> {
        struct CdfDoubleVisitor;

        impl Visitor<'_> for CdfDoubleVisitor {
            type Value = Option<f64>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "double, null, Infinity, or NaN")
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Some(v))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "Infinity" => Ok(Some(f64::INFINITY)),
                    "-Infinity" => Ok(Some(f64::NEG_INFINITY)),
                    "NaN" => Ok(Some(f64::NAN)),
                    r => Err(E::custom(format!("Failed to parse double value from string. Got {r} expected Infinity, -Infinity, or NaN")))
                }
            }
        }

        deserializer.deserialize_any(CdfDoubleVisitor)
    }

    pub fn serialize<S: Serializer>(value: &Option<f64>, ser: S) -> Result<S::Ok, S::Error> {
        match value {
            None => ser.serialize_none(),
            Some(r) if r.is_nan() => ser.serialize_str("NaN"),
            Some(f64::INFINITY) => ser.serialize_str("Infinity"),
            Some(f64::NEG_INFINITY) => ser.serialize_str("-Infinity"),
            Some(r) => ser.serialize_f64(*r),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A datapoint with double precision floating point value.
pub struct DatapointDouble {
    /// Timestamp in milliseconds since epoch.
    pub timestamp: i64,
    /// Datapoint value.
    #[serde(with = "cdf_double_serde")]
    pub value: Option<f64>,
    /// Datapoint status code.
    pub status: Option<StatusCode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A datapoint with string value.
pub struct DatapointString {
    /// Timestamp in milliseconds since epoch.
    pub timestamp: i64,
    /// Datapoint value.
    pub value: Option<String>,
    /// Datapoint status code.
    pub status: Option<StatusCode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// An aggregate data point.
pub struct DatapointAggregate {
    /// Timestamp in milliseconds since epoch.
    pub timestamp: i64,
    /// Average of values in aggregate.
    pub average: f64,
    /// Max value in aggregate.
    pub max: f64,
    /// Min value in aggregate.
    pub min: f64,
    /// Number of values in aggregate.
    pub count: f64,
    /// Sum of values in aggregate.
    pub sum: f64,
    /// Interpolated value.
    pub interpolation: f64,
    /// Step-interpolated value.
    pub step_interpolation: f64,
    /// The variance of the underlying function when assuming linear or step behavior between data points.
    pub continuous_variance: f64,
    /// The variance of the discrete set of data points, no weighting for density of points in time.
    pub discrete_variance: f64,
    /// The sum of absolute differences between neighboring data points in a period.
    pub total_variation: f64,
    /// The number of data points in the aggregate period that have a Good status code.
    /// Uncertain does not count, irrespective of treatUncertainAsBad parameter.
    pub count_good: f64,
    /// The number of data points in the aggregate period that have an Uncertain status code.
    pub count_uncertain: f64,
    /// The number of data points in the aggregate period that have a Bad status code.
    /// Uncertain does not count, irrespective of treatUncertainAsBad parameter.
    pub count_bad: f64,
    /// The duration the aggregate is defined and marked as good (regardless of ignoreBadDataPoints parameter).
    /// Measured in milliseconds. Equivalent to duration that the previous data point is good and in range.
    pub duration_good: f64,
    /// The duration the aggregate is defined and marked as uncertain (regardless of ignoreBadDataPoints parameter).
    /// Measured in milliseconds. Equivalent to duration that the previous data point is uncertain and in range.
    pub duration_uncertain: f64,
    /// The duration the aggregate is defined but marked as bad (regardless of ignoreBadDataPoints parameter).
    /// Measured in milliseconds. Equivalent to duration that the previous data point is bad and in range.
    pub duration_bad: f64,
}

impl From<NumericDatapoint> for DatapointDouble {
    fn from(dp: NumericDatapoint) -> DatapointDouble {
        DatapointDouble {
            timestamp: dp.timestamp,
            value: if dp.null_value { None } else { Some(dp.value) },
            status: dp.status.map(|s| s.into()),
        }
    }
}

impl From<DatapointDouble> for NumericDatapoint {
    fn from(dp: DatapointDouble) -> NumericDatapoint {
        NumericDatapoint {
            timestamp: dp.timestamp,
            null_value: dp.value.is_none(),
            value: dp.value.unwrap_or_default(),
            status: dp.status.map(|s| s.into()),
        }
    }
}

impl From<StringDatapoint> for DatapointString {
    fn from(dp: StringDatapoint) -> DatapointString {
        DatapointString {
            timestamp: dp.timestamp,
            value: if dp.null_value { None } else { Some(dp.value) },
            status: dp.status.map(|s| s.into()),
        }
    }
}

impl From<DatapointString> for StringDatapoint {
    fn from(dp: DatapointString) -> StringDatapoint {
        StringDatapoint {
            timestamp: dp.timestamp,
            null_value: dp.value.is_none(),
            value: dp.value.unwrap_or_default(),
            status: dp.status.map(|s| s.into()),
        }
    }
}

impl From<AggregateDatapoint> for DatapointAggregate {
    fn from(dp: AggregateDatapoint) -> DatapointAggregate {
        DatapointAggregate {
            timestamp: dp.timestamp,
            average: dp.average,
            max: dp.max,
            min: dp.min,
            count: dp.count,
            sum: dp.sum,
            interpolation: dp.interpolation,
            step_interpolation: dp.step_interpolation,
            continuous_variance: dp.continuous_variance,
            discrete_variance: dp.discrete_variance,
            total_variation: dp.total_variation,
            count_good: dp.count_good,
            count_uncertain: dp.count_uncertain,
            count_bad: dp.count_bad,
            duration_good: dp.duration_good,
            duration_uncertain: dp.duration_uncertain,
            duration_bad: dp.duration_bad,
        }
    }
}

#[derive(Debug)]
/// Response to a request for datapoints.
pub struct DatapointsListResponse {
    /// List of datapoint responses.
    pub items: Vec<DatapointsResponse>,
}

#[derive(Debug)]
/// Response for a single timeseries when listing datapoints.
pub struct DatapointsResponse {
    /// Time series internal ID.
    pub id: i64,
    /// Time series external ID.
    pub external_id: Option<String>,
    /// Retrieved datapoints.
    pub datapoints: DatapointsEnumType,
    /// The physical unit of the time series (free-text field).
    /// Omitted if data points were converted to a different unit.
    pub unit: Option<String>,
    /// The physical unit of the time series as represented in the unit catalog.
    /// Replaced with target unit if data points were converted.
    pub unit_external_id: Option<String>,
    /// Time series `is_step` property value.
    pub is_step: bool,
    /// Whether this is a string time series.
    pub is_string: bool,
    /// The cursor to get the next page of results (if available).
    /// nextCursor will be omitted when the next aggregate datapoint
    /// is after the end of the interval. Increase start/end to fetch more data.
    pub next_cursor: Option<String>,
}

#[derive(Debug)]
/// Result for retrieving a latest datapoint from CDF.
pub enum LatestDatapoint {
    /// Numeric datapoint.
    Numeric(DatapointDouble),
    /// String datapoint.
    String(DatapointString),
}

impl LatestDatapoint {
    /// Get the value of this as a numeric datapoint.
    pub fn numeric(&self) -> Option<&DatapointDouble> {
        match self {
            Self::Numeric(d) => Some(d),
            _ => None,
        }
    }

    /// Get the value of this as a string datapoint.
    pub fn string(&self) -> Option<&DatapointString> {
        match self {
            Self::String(d) => Some(d),
            _ => None,
        }
    }
}

#[derive(Debug)]
/// Response to a request retrieving latest datapoints for a single time series.
pub struct LatestDatapointsResponse {
    /// Time series internal ID.
    pub id: i64,
    /// Time series external ID.
    pub external_id: Option<String>,
    /// Retrieved datapoints.
    pub datapoint: Option<LatestDatapoint>,
    /// The physical unit of the time series (free-text field).
    /// Omitted if data points were converted to a different unit.
    pub unit: Option<String>,
    /// The physical unit of the time series as represented in the unit catalog.
    /// Replaced with target unit if data points were converted.
    pub unit_external_id: Option<String>,
    /// Time series `is_step` property value.
    pub is_step: bool,
    /// Whether this is a string time series.
    pub is_string: bool,
    /// The cursor to get the next page of results (if available).
    /// nextCursor will be omitted when the next aggregate datapoint
    /// is after the end of the interval. Increase start/end to fetch more data.
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DatapointsResponsePartial {
    id: i64,
    external_id: Option<String>,
    datapoints: Value,
    unit: Option<String>,
    unit_external_id: Option<String>,
    #[serde(default)]
    is_step: bool,
    #[serde(default)]
    is_string: bool,
    next_cursor: Option<String>,
}

impl<'de> Deserialize<'de> for LatestDatapointsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let r = DatapointsResponsePartial::deserialize(deserializer)?;
        let dps = r.datapoints;
        let dps = if matches!(dps, Value::Null) {
            None
        } else if let Value::Array(v) = dps {
            match v.into_iter().next() {
                Some(v) => {
                    if r.is_string {
                        Some(LatestDatapoint::String(serde_json::from_value(v).map_err(
                            |e| {
                                D::Error::custom(format!(
                                    "Failed to deserialize string datapoint: {e:?}"
                                ))
                            },
                        )?))
                    } else {
                        Some(LatestDatapoint::Numeric(
                            serde_json::from_value(v).map_err(|e| {
                                D::Error::custom(format!(
                                    "Failed to deserialize numeric datapoint: {e:?}"
                                ))
                            })?,
                        ))
                    }
                }
                None => None,
            }
        } else {
            None
        };

        Ok(Self {
            id: r.id,
            external_id: r.external_id,
            datapoint: dps,
            unit: r.unit,
            unit_external_id: r.unit_external_id,
            is_step: r.is_step,
            is_string: r.is_string,
            next_cursor: r.next_cursor,
        })
    }
}

#[derive(Debug, Clone)]
/// Add datapoints to a time series.
pub struct AddDatapoints {
    /// ID or external ID of time series to insert into.
    pub id: IdentityOrInstance,
    /// Data points to insert.
    pub datapoints: DatapointsEnumType,
}

impl AddDatapoints {
    /// Create a new batch of data points to insert.
    ///
    /// # Arguments
    ///
    /// * `id` - Internal ID of time series to insert into.
    /// * `datapoints` - Datapoints to insert.
    pub fn new(id: i64, datapoints: DatapointsEnumType) -> AddDatapoints {
        AddDatapoints {
            id: IdentityOrInstance::Identity(Identity::Id { id }),
            datapoints,
        }
    }
    /// Create a new batch of data points to insert.
    ///
    /// # Arguments
    ///
    /// * `external_id` - External ID of time series to insert into.
    /// * `datapoints` - Datapoints to insert.
    pub fn new_external_id(external_id: &str, datapoints: DatapointsEnumType) -> AddDatapoints {
        AddDatapoints {
            id: IdentityOrInstance::Identity(Identity::ExternalId {
                external_id: external_id.to_string(),
            }),
            datapoints,
        }
    }
}

impl From<Identity> for TimeSeriesReference {
    fn from(idt: Identity) -> TimeSeriesReference {
        match idt {
            Identity::Id { id } => TimeSeriesReference::Id(id),
            Identity::ExternalId {
                external_id: ext_id,
            } => TimeSeriesReference::ExternalId(ext_id),
        }
    }
}

impl From<IdentityOrInstance> for TimeSeriesReference {
    fn from(idt: IdentityOrInstance) -> TimeSeriesReference {
        match idt {
            IdentityOrInstance::Identity(Identity::Id { id }) => TimeSeriesReference::Id(id),
            IdentityOrInstance::Identity(Identity::ExternalId {
                external_id: ext_id,
            }) => TimeSeriesReference::ExternalId(ext_id),
            IdentityOrInstance::InstanceId { instance_id } => {
                TimeSeriesReference::InstanceId(instance_id.into())
            }
        }
    }
}

impl TryFrom<TimeSeriesReference> for Identity {
    type Error = ();

    fn try_from(idt: TimeSeriesReference) -> Result<Identity, ()> {
        match idt {
            TimeSeriesReference::Id(id) => Ok(Identity::Id { id }),
            TimeSeriesReference::ExternalId(ext_id) => Ok(Identity::ExternalId {
                external_id: ext_id,
            }),
            TimeSeriesReference::InstanceId(_) => Err(()),
        }
    }
}

impl From<crate::dto::data_modeling::instances::InstanceId> for InstanceId {
    fn from(value: crate::dto::data_modeling::instances::InstanceId) -> Self {
        Self {
            external_id: value.external_id,
            space: value.space,
        }
    }
}

impl From<InstanceId> for crate::dto::data_modeling::instances::InstanceId {
    fn from(value: InstanceId) -> Self {
        Self {
            external_id: value.external_id,
            space: value.space,
        }
    }
}

impl From<TimeSeriesReference> for IdentityOrInstance {
    fn from(value: TimeSeriesReference) -> Self {
        match value {
            TimeSeriesReference::Id(id) => IdentityOrInstance::Identity(Identity::Id { id }),
            TimeSeriesReference::ExternalId(external_id) => {
                IdentityOrInstance::Identity(Identity::ExternalId { external_id })
            }
            TimeSeriesReference::InstanceId(instance_id) => IdentityOrInstance::InstanceId {
                instance_id: instance_id.into(),
            },
        }
    }
}

impl From<DataPointListItem> for DatapointsResponse {
    fn from(req: DataPointListItem) -> DatapointsResponse {
        DatapointsResponse {
            id: req.id,
            external_id: if req.external_id.is_empty() {
                None
            } else {
                Some(req.external_id)
            },
            unit: if req.unit.is_empty() {
                None
            } else {
                Some(req.unit)
            },
            is_step: req.is_step,
            is_string: req.is_string,
            datapoints: match req.datapoint_type {
                Some(dps) => match dps {
                    data_point_list_item::DatapointType::NumericDatapoints(num_dps) => {
                        DatapointsEnumType::NumericDatapoints(
                            num_dps
                                .datapoints
                                .into_iter()
                                .map(DatapointDouble::from)
                                .collect(),
                        )
                    }
                    data_point_list_item::DatapointType::StringDatapoints(str_dps) => {
                        DatapointsEnumType::StringDatapoints(
                            str_dps
                                .datapoints
                                .into_iter()
                                .map(DatapointString::from)
                                .collect(),
                        )
                    }
                    data_point_list_item::DatapointType::AggregateDatapoints(aggr_dps) => {
                        DatapointsEnumType::AggregateDatapoints(
                            aggr_dps
                                .datapoints
                                .into_iter()
                                .map(DatapointAggregate::from)
                                .collect(),
                        )
                    }
                },
                None => DatapointsEnumType::NumericDatapoints(Vec::<DatapointDouble>::new()),
            },
            unit_external_id: if req.unit_external_id.is_empty() {
                None
            } else {
                Some(req.unit_external_id)
            },
            next_cursor: if req.next_cursor.is_empty() {
                None
            } else {
                Some(req.next_cursor)
            },
        }
    }
}

impl From<AddDatapoints> for DataPointInsertionItem {
    fn from(req: AddDatapoints) -> DataPointInsertionItem {
        DataPointInsertionItem {
            time_series_reference: Some(TimeSeriesReference::from(req.id)),
            datapoint_type: match req.datapoints {
                DatapointsEnumType::NumericDatapoints(dps) => Some(
                    self::proto::data_point_insertion_item::DatapointType::NumericDatapoints(
                        NumericDatapoints {
                            datapoints: dps.into_iter().map(NumericDatapoint::from).collect(),
                        },
                    ),
                ),
                DatapointsEnumType::StringDatapoints(dps) => Some(
                    self::proto::data_point_insertion_item::DatapointType::StringDatapoints(
                        StringDatapoints {
                            datapoints: dps.into_iter().map(StringDatapoint::from).collect(),
                        },
                    ),
                ),
                DatapointsEnumType::AggregateDatapoints(_) => {
                    panic!("Cannot insert aggregate datapoints")
                }
            },
        }
    }
}

impl From<Vec<AddDatapoints>> for DataPointInsertionRequest {
    fn from(items: Vec<AddDatapoints>) -> DataPointInsertionRequest {
        DataPointInsertionRequest {
            items: items
                .into_iter()
                .map(DataPointInsertionItem::from)
                .collect(),
        }
    }
}

impl From<DataPointListResponse> for DatapointsListResponse {
    fn from(resp: DataPointListResponse) -> DatapointsListResponse {
        DatapointsListResponse {
            items: resp
                .items
                .into_iter()
                .map(DatapointsResponse::from)
                .collect(),
        }
    }
}
