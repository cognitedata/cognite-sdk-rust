use super::{ApiClient};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointResponse {
  data : DatapointListResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointListResponse {
  items : Vec<DatapointsResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsResponse {
  name : String,
  datapoints : Vec<Datapoint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Datapoint {
  timestamp : u128,
  value : String,
  average : u64,
  max : u64,
  min : u64,
  count : u64,
  sum : u64,
  interpolation : u64,
  step_interpolation : u64,
  continous_variance : u64,
  discrete_variance : u64,
  total_variance : u64,
}

pub struct Datapoints {
  api_client : ApiClient,
}

impl Datapoints {
  pub fn new(api_client : ApiClient) -> Datapoints {
    Datapoints {
      api_client : api_client
    }
  }
}