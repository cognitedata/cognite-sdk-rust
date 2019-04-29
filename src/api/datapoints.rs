use crate::api::ApiClient;
use crate::api::params::{Params};
use crate::error::{Result};

use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointListResponseWrapper {
  data : DatapointItems
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointItems {
  items : Vec<DatapointsResponse>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsResponse {
  name : String,
  datapoints : Vec<Datapoint>,
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

  pub fn retrieve_from_time_serie_by_id(&self, time_serie_id : u64, params : Option<Vec<Params>>) -> Result<Vec<Datapoint>> {
    match self.api_client.get::<DatapointListResponseWrapper>(&format!("timeseries/{}/data", time_serie_id), params){
      Ok(mut datapoints_response) => {
        let datapoints = datapoints_response.data.items.pop().unwrap();
        Ok(datapoints.datapoints)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve_from_time_serie_by_name(&self, time_serie_name : &str, params : Option<Vec<Params>>) -> Result<Vec<Datapoint>> {
    match self.api_client.get::<DatapointListResponseWrapper>(&format!("timeseries/data/{}", time_serie_name), params) {
      Ok(mut datapoints_response) => {
        let datapoints = datapoints_response.data.items.pop().unwrap();
        Ok(datapoints.datapoints)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve_latest_from_time_serie_by_name(&self, time_serie_name : &str, params : Option<Vec<Params>>) -> Result<Datapoint> {
    match self.api_client.get::<DatapointResponseWrapper>(&format!("timeseries/latest/{}", time_serie_name), params) {
      Ok(mut datapoint_response) => {
        let datapoint = datapoint_response.data.items.pop().unwrap();
        Ok(datapoint)
      },
      Err(e) => Err(e)
    }
  }

  pub fn insert_in_time_serie_by_id(&self, time_serie_id : String, datapoints : Vec<Datapoint>) -> Result<Vec<Datapoint>> {
    unimplemented!();
  }

  pub fn insert_in_time_serie_by_name(&self, time_serie_name : String, datapoints : Vec<Datapoint>) -> Result<Vec<Datapoint>> {
    unimplemented!();
  }

  pub fn delete_single_in_time_serie_by_id(&self, time_serie_id : u64, timestamp : u64) -> Result<()> {
    unimplemented!();
  }

  pub fn delete_single_in_time_serie_by_name(&self, time_serie_name : String, timestamp : u64) -> Result<()> {
    unimplemented!();
  }

  pub fn delete_in_time_serie_by_id(&self, time_serie_id : u64, from : u64, to : u64) -> Result<()> {
    unimplemented!();
  }

  pub fn delete_in_time_serie_by_name(&self, time_serie_name : String, from : u64, to : u64) -> Result<()> {
    unimplemented!();
  }
}