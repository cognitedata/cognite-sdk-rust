use super::{
  ApiClient,
  Params,
};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointListResponseWrapper {
  data : DatapointListResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointListResponse {
  items : Vec<DatapointsResponse>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointResponseWrapper {
  data : DatapointResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointResponse {
  items : Vec<Datapoint>,
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
  timestamp : u64,
  value : Value,
  average : Option<u64>,
  max : Option<u64>,
  min : Option<u64>,
  count : Option<u64>,
  sum : Option<u64>,
  interpolation : Option<u64>,
  step_interpolation : Option<u64>,
  continous_variance : Option<u64>,
  discrete_variance : Option<u64>,
  total_variance : Option<u64>,
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

  pub fn retrieve_from_time_serie_by_id(&self, time_serie_id : u64, params : Option<Vec<Params>>) -> Vec<Datapoint> {
    let datapoints_response_json = self.api_client.get(&format!("timeseries/{}/data", time_serie_id), params).unwrap();
    let mut datapoints_response : DatapointListResponseWrapper = serde_json::from_str(&datapoints_response_json).unwrap();
    let datapoints = datapoints_response.data.items.pop().unwrap();
    datapoints.datapoints
  }

  pub fn retrieve_from_time_serie_by_name(&self, time_serie_name : &str, params : Option<Vec<Params>>) -> Vec<Datapoint> {
    let datapoints_response_json = self.api_client.get(&format!("timeseries/data/{}", time_serie_name), params).unwrap();
    let mut datapoints_response : DatapointListResponseWrapper = serde_json::from_str(&datapoints_response_json).unwrap();
    let datapoints = datapoints_response.data.items.pop().unwrap();
    datapoints.datapoints
  }

  pub fn retrieve_latest_from_time_serie_by_name(&self, time_serie_name : &str, params : Option<Vec<Params>>) -> Datapoint {
    let datapoint_response_json = self.api_client.get(&format!("timeseries/latest/{}", time_serie_name), params).unwrap();
    let mut datapoint_response : DatapointResponseWrapper = serde_json::from_str(&datapoint_response_json).unwrap();
    let datapoint = datapoint_response.data.items.pop().unwrap();
    datapoint
  }

  pub fn insert_in_time_serie_by_id(&self, time_serie_id : String, datapoints : Vec<Datapoint>) -> Vec<Datapoint> {
    unimplemented!();
  }

  pub fn insert_in_time_serie_by_name(&self, time_serie_name : String, datapoints : Vec<Datapoint>) -> Vec<Datapoint> {
    unimplemented!();
  }

  pub fn delete_single_in_time_serie_by_id(&self, time_serie_id : u64, timestamp : u64) -> () {
    unimplemented!();
  }

  pub fn delete_single_in_time_serie_by_name(&self, time_serie_name : String, timestamp : u64) -> () {
    unimplemented!();
  }

  pub fn delete_in_time_serie_by_id(&self, time_serie_id : u64, from : u64, to : u64) -> () {
    unimplemented!();
  }

  pub fn delete_in_time_serie_by_name(&self, time_serie_name : String, from : u64, to : u64) -> () {
    unimplemented!();
  }
}