use crate::api::ApiClient;
use crate::api::params::{Params};
use crate::error::{Result};
use crate::dto::datapoint::*;

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

  pub fn insert_in_time_serie_by_id(&self, time_serie_id : u64, datapoints : &[Datapoint]) -> Result<()> {
    let add_datapoints : Vec<AddDatapoint> = datapoints.iter().map(| d | AddDatapoint::from(d)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&add_datapoints).unwrap());
    match self.api_client.post::<::serde_json::Value>(&format!("timeseries/{}/data", time_serie_id), &request_body){
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }

  pub fn insert_in_time_serie_by_name(&self, time_serie_name : &str, datapoints : &[Datapoint]) -> Result<()> {
    let add_datapoints : Vec<AddDatapoint> = datapoints.iter().map(| d | AddDatapoint::from(d)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&add_datapoints).unwrap());
    match self.api_client.post::<::serde_json::Value>(&format!("timeseries/data/{}", time_serie_name), &request_body){
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }

  pub fn delete_single_in_time_serie_by_id(&self, time_serie_id : u64, timestamp : u128) -> Result<()> {
    let params = vec!(Params::DatapointsDelete_Timestamp(timestamp));
    match self.api_client.delete_with_params::<::serde_json::Value>(&format!("timeseries/{}/data/deletesingle", time_serie_id), Some(params)){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete_single_in_time_serie_by_name(&self, time_serie_name : &str, timestamp : u128) -> Result<()> {
    let params = vec!(Params::DatapointsDelete_Timestamp(timestamp));
    match self.api_client.delete_with_params::<::serde_json::Value>(&format!("timeseries/data/{}/deletesognle", time_serie_name), Some(params)){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete_in_time_serie_by_id(&self, time_serie_id : u64, from : u128, to : u128) -> Result<()> {
    let params = vec!(Params::DatapointsDelete_TimestampInclusiveBegin(from), Params::DatapointsDelete_TimestampExclusideEnd(to));
    match self.api_client.delete_with_params::<::serde_json::Value>(&format!("timeseries/{}/data/deleterange", time_serie_id), Some(params)){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete_in_time_serie_by_name(&self, time_serie_name : &str, from : u128, to : u128) -> Result<()> {
    let params = vec!(Params::DatapointsDelete_TimestampInclusiveBegin(from), Params::DatapointsDelete_TimestampExclusideEnd(to));
    match self.api_client.delete_with_params::<::serde_json::Value>(&format!("timeseries/data/{}/deleterange", time_serie_name), Some(params)){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}