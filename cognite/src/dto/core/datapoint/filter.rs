use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Identity;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Aggregate {
    Average,
    Max,
    Min,
    Count,
    Sum,
    Interpolation,
    StepInterpolation,
    TotalVariation,
    ContinuousVariance,
    DiscreteVariance,
}

impl From<&str> for Aggregate {
    fn from(val: &str) -> Aggregate {
        serde_json::from_str(val).unwrap()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsFilter {
    pub items: Vec<DatapointsQuery>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub limit: Option<u32>,
    pub aggregates: Option<Vec<Aggregate>>,
    pub granularity: Option<String>,
    pub include_outside_points: Option<bool>,
    pub ignore_unknown_ids: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsQuery {
    #[serde(flatten)]
    pub id: Identity,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub limit: Option<u32>,
    pub aggregates: Option<Vec<String>>,
    pub granularity: Option<String>,
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
pub struct LatestDatapointsQuery {
    pub before: String,
    #[serde(flatten)]
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
pub struct DeleteDatapointsQuery {
    pub inclusive_begin: i64,
    pub exclusive_end: i64,
    #[serde(rename = "id", flatten)]
    pub id: Identity,
}

impl DeleteDatapointsQuery {
    pub fn new(
        time_serie_id: Identity,
        inclusive_begin: i64,
        exclusive_end: i64,
    ) -> DeleteDatapointsQuery {
        DeleteDatapointsQuery {
            id: time_serie_id,
            inclusive_begin,
            exclusive_end,
        }
    }
}
