use super::{ApiClient};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieResponse {
  data : TimeSerieListResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieListResponse {
  items : Vec<TimeSerie>,
  previous_cursor : String,
  next_cursor : String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerie {
  name: String,
  is_string: bool,
  unit: String,
  asset_id: u64,
  is_step: bool,
  description: String,
  security_categories: Vec<u64>,
  id: u64,
  created_time: u64,
  last_updated_time: u64
}

pub struct TimeSeries {
  api_client : ApiClient,
}

impl TimeSeries {
  pub fn new(api_client : ApiClient) -> TimeSeries {
    TimeSeries {
      api_client : api_client
    }
  }
}