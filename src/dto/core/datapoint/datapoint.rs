use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsListResponse {
  pub items : Vec<DatapointsResponse>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointsResponse {
  #[serde(rename = "id")]
  pub time_serie_id : u64,
  pub external_id : Option<String>,
  pub datapoints : Vec<Datapoint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Datapoint {
  pub timestamp : i64,
  pub value : f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddDatapoints {
  #[serde(rename = "id")]
  pub time_serie_id : u64,
  pub datapoints : Vec<Datapoint>,
}

impl AddDatapoints {
  pub fn new(time_serie_id : u64, datapoints : Vec<Datapoint>) -> AddDatapoints {
    AddDatapoints {
        time_serie_id : time_serie_id,
        datapoints : datapoints,
      }
  }
}

