use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsFilter {
  pub items : Vec<DatapointsQuery>,
  pub start : Option<String>,
  pub end : Option<String>, 
  pub limit : Option<u32>,
  pub aggregates : Option<Vec<String>>,
  pub granularity : Option<String>,
  pub include_outside_points : Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsQuery {
  #[serde(rename = "id")]
  pub time_serie_id : u64,
  pub start : Option<String>,
  pub end : Option<String>, 
  pub limit : Option<u32>,
  pub aggregates : Option<Vec<String>>,
  pub granularity : Option<String>,
  pub include_outside_points : Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LatestDatapointsQuery {
  pub before : String,
  #[serde(rename = "id")]
  pub time_serie_id : u64
}

impl LatestDatapointsQuery {
  pub fn new(time_serie_id : u64, before : &str) -> LatestDatapointsQuery {
    LatestDatapointsQuery {
      time_serie_id : time_serie_id,
      before : String::from(before),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatapointsQuery {
  pub inclusive_begin : u128,
  pub exclusive_end : u128,
  #[serde(rename = "id")]
  pub time_serie_id : u64
}

impl DeleteDatapointsQuery {
  pub fn new(time_serie_id : u64, inclusive_begin : u128, exclusive_end : u128) -> DeleteDatapointsQuery {
    DeleteDatapointsQuery {
      time_serie_id : time_serie_id,
      inclusive_begin : inclusive_begin,
      exclusive_end : exclusive_end,
    }
  }
}