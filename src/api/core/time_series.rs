use crate::api::ApiClient;
use crate::error::{Result};
use crate::dto::params::{Params};
use crate::dto::core::time_serie::*;

pub struct TimeSeries {
  api_client : ApiClient,
}

impl TimeSeries {
  pub fn new(api_client : ApiClient) -> TimeSeries {
    TimeSeries {
      api_client : api_client
    }
  }

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Result<Vec<TimeSerie>> {
    match self.api_client.get_with_params::<TimeSerieListResponse>("timeseries", params){
      Ok(time_series_response) => {
        let time_series = time_series_response.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, time_series : &[TimeSerie]) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(time_series).unwrap());
    match self.api_client.post::<::serde_json::Value>("timeseries", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, time_serie_filter : TimeSerieFilter, time_serie_search : TimeSerieSearch) -> Result<Vec<TimeSerie>> {
    let filter : Search = Search::new(time_serie_filter, time_serie_search, None);
    match self.api_client.post::<TimeSerieListResponse>("timeseries/search", &serde_json::to_string(&filter).unwrap()){
      Ok(time_series_response) => {
        let time_series = time_series_response.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, time_serie_ids : &[u64]) -> Result<Vec<TimeSerie>> {
    let id_list : Vec<TimeSerieId> = time_serie_ids.iter().map(| ts_id | TimeSerieId::from(*ts_id)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&id_list).unwrap());
    match self.api_client.post::<TimeSerieListResponse>("timeseries/byids", &request_body){
      Ok(time_series_response) => {
        let time_series = time_series_response.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, time_series : &[TimeSerie]) -> Result<Vec<TimeSerie>> {
    let patch_time_series : Vec<PatchTimeSerie> = time_series.iter().map(| a | PatchTimeSerie::from(a)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&patch_time_series).unwrap());
    println!("{:?}", request_body);
    match self.api_client.post::<TimeSerieListResponse>("timeseries/update", &request_body){
      Ok(time_series_response) => {
        let time_series = time_series_response.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, time_serie_ids : &[u64]) -> Result<()> {
    let id_list : Vec<TimeSerieId> = time_serie_ids.iter().map(| ts_id | TimeSerieId::from(*ts_id)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&id_list).unwrap());
    match self.api_client.post::<::serde_json::Value>("timeseries/delete", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}