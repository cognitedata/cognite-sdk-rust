use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::instances::{InstanceId, NodeOrEdgeCreate};

use super::{
    common::{CogniteDescribable, CogniteSourceable},
    CogniteExtendable, WithInstance, WithView,
};

/// Represents a series of data points in time order..
pub type CogniteTimeseries = CogniteExtendable<Timeseries>;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "lowercase")]
/// Specifies the data type of the data points.
pub enum TimeSeriesType {
    /// Indicates that timeseries type is a string.
    String,
    #[default]
    /// Indicates that timeseries type is a number.
    Numeric,
}

impl WithView for CogniteTimeseries {
    const SPACE: &'static str = "cdf_cdm";
    const EXTERNAL_ID: &'static str = "CogniteTimeSeries";
    const VERSION: &'static str = "v1";
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// The properties of the file object.
pub struct Timeseries {
    #[serde(flatten)]
    /// Descriptions of the instance.
    pub description: CogniteDescribable,
    #[serde(flatten)]
    /// Source system.
    pub source: CogniteSourceable,
    /// Defines whether the time series is a step series or not.
    pub is_step: bool,
    /// Unit as specified in the source system.
    pub source_unit: String,
    /// Direct relation to the unit of the time series.
    pub unit: Option<InstanceId>,
    /// List of assets to which this file relates.
    pub assets: Option<Vec<InstanceId>>,
    /// List of activities associated with this time series.
    pub activities: Option<Vec<InstanceId>>,
    /// Type of datapoints the time series contains.
    pub r#type: TimeSeriesType,
}

impl Timeseries {
    /// Create a new timeseries instance.
    pub fn new(is_step: bool) -> Self {
        Self {
            is_step,
            ..Default::default()
        }
    }
}

impl From<CogniteTimeseries> for NodeOrEdgeCreate<Timeseries> {
    fn from(value: CogniteTimeseries) -> NodeOrEdgeCreate<Timeseries> {
        value.instance()
    }
}
