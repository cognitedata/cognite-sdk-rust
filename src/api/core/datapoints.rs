use crate::api::ApiClient;
use crate::error::{Result};
use crate::dto::core::datapoint::*;

pub struct Datapoints {
  api_client : ApiClient,
}

impl Datapoints {
  pub fn new(api_client : ApiClient) -> Datapoints {
    Datapoints {
      api_client : api_client
    }
  }

  pub fn insert(&self, add_datapoints : &[AddDatapoints]) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(add_datapoints).unwrap());
    match self.api_client.post::<::serde_json::Value>("timeseries/data", &request_body){
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, datapoints_filter : DatapointsFilter) -> Result<Vec<DatapointsResponse>> {
    let request_body = serde_json::to_string(&datapoints_filter).unwrap();
    match self.api_client.post::<DatapointsListResponse>("timeseries/data/list", &request_body){
      Ok(datapoints_response) => {
        let datapoints = datapoints_response.items;
        Ok(datapoints)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve_latest(&self, time_serie_id : u64, before : &str) -> Result<DatapointsResponse> {
    let latest_datapoint_query : LatestDatapointsQuery = LatestDatapointsQuery::new(time_serie_id, before);
    let request_body = serde_json::to_string(&latest_datapoint_query).unwrap();
    match self.api_client.post::<DatapointsListResponse>("timeseries/data/latest", &request_body) {
      Ok(mut datapoint_response) => {
        let datapoint = datapoint_response.items.pop().unwrap();
        Ok(datapoint)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, time_serie_id : u64, inclusive_begin : u128, exclusive_end : u128) -> Result<()> {
    let delete_datapoint_query : DeleteDatapointsQuery = DeleteDatapointsQuery::new(time_serie_id, inclusive_begin, exclusive_end);
    let request_body = serde_json::to_string(&delete_datapoint_query).unwrap();
    match self.api_client.post::<::serde_json::Value>("timeseries/data/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

}