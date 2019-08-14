use crate::api::ApiClient;
use crate::error::{Result, Error, Kind};
use crate::dto::core::datapoint::*;
use crate::dto::items::Items;

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
    let add_datapoints_items = Items::from(add_datapoints);
    self.api_client.post::<::serde_json::Value, Items>("timeseries/data", &add_datapoints_items)?;
    Ok(())
  }

  pub fn retrieve(&self, datapoints_filter : DatapointsFilter) -> Result<Vec<DatapointsResponse>> {
    let datapoints_response : DatapointsListResponse = self.api_client.post("timeseries/data/list", &datapoints_filter)?;
    Ok(datapoints_response.items)
  }

  pub fn retrieve_latest(&self, time_serie_id : u64, before : &str) -> Result<DatapointsResponse> {
    let latest_datapoint_query : LatestDatapointsQuery = LatestDatapointsQuery::new(time_serie_id, before);
    let mut datapoints_response : DatapointsListResponse = self.api_client.post("timeseries/data/latest", &latest_datapoint_query)?;
    if let Some(datapoint) = datapoints_response.items.pop() {
      return Ok(datapoint);
    }
    Err(Error::new(Kind::NotFound("Datapoint not found".to_owned())))
  }

  pub fn delete(&self, time_serie_id : u64, inclusive_begin : u128, exclusive_end : u128) -> Result<()> {
    let delete_datapoint_query : DeleteDatapointsQuery = DeleteDatapointsQuery::new(time_serie_id, inclusive_begin, exclusive_end);
    self.api_client.post::<::serde_json::Value, DeleteDatapointsQuery>("timeseries/data/delete", &delete_datapoint_query)?;
    Ok(())
  }

}