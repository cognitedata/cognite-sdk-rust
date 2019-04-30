use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointListResponseWrapper {
  pub data : DatapointItems
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointItems {
  pub items : Vec<DatapointsResponse>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsResponse {
  pub name : String,
  pub datapoints : Vec<Datapoint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointResponseWrapper {
  pub data : DatapointResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointResponse {
  pub items : Vec<Datapoint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Datapoint {
  pub timestamp : u64,
  pub value : Value,
  pub average : Option<u64>,
  pub max : Option<u64>,
  pub min : Option<u64>,
  pub count : Option<u64>,
  pub sum : Option<u64>,
  pub interpolation : Option<u64>,
  pub step_interpolation : Option<u64>,
  pub continous_variance : Option<u64>,
  pub discrete_variance : Option<u64>,
  pub total_variance : Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddDatapoint {
  pub timestamp : u64,
  pub value : Value,
}

impl From<&Datapoint> for AddDatapoint {
  fn from(datapoint : &Datapoint) -> AddDatapoint {
      AddDatapoint { 
        timestamp : datapoint.timestamp,
        value : datapoint.value.clone(),
      }
  }
}
