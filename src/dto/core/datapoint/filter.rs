use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsFilter {
    pub items: Vec<DatapointsQuery>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub limit: Option<u32>,
    pub aggregates: Option<Vec<String>>,
    pub granularity: Option<String>,
    pub include_outside_points: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsQuery {
    #[serde(rename = "id")]
    pub time_serie_id: u64,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub limit: Option<u32>,
    pub aggregates: Option<Vec<String>>,
    pub granularity: Option<String>,
    pub include_outside_points: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LatestDatapointsQuery {
    pub before: String,
    #[serde(rename = "id")]
    pub time_serie_id: u64,
}

impl LatestDatapointsQuery {
    pub fn new(time_serie_id: u64, before: &str) -> LatestDatapointsQuery {
        LatestDatapointsQuery {
            time_serie_id: time_serie_id,
            before: String::from(before),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatapointsQuery {
    pub inclusive_begin: i64,
    pub exclusive_end: i64,
    #[serde(rename = "id")]
    pub time_serie_id: u64,
}

impl DeleteDatapointsQuery {
    pub fn new(
        time_serie_id: u64,
        inclusive_begin: i64,
        exclusive_end: i64,
    ) -> DeleteDatapointsQuery {
        DeleteDatapointsQuery {
            time_serie_id: time_serie_id,
            inclusive_begin: inclusive_begin,
            exclusive_end: exclusive_end,
        }
    }
}
