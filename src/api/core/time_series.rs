use crate::api::ApiClient;
use crate::error::{Result};
use crate::dto::params::{Params};
use crate::dto::core::time_serie::*;
use crate::dto::items::Items;

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

  pub fn create(&self, time_series : &[TimeSerie]) -> Result<Vec<TimeSerie>> {
    let add_time_series : Vec<AddTimeSerie> = time_series.iter().map(| ts | AddTimeSerie::from(ts)).collect();
    let add_time_series_items = Items::from(&add_time_series);
    match self.api_client.post("timeseries", &add_time_series_items){
      Ok(result) => {
        let time_series_response : TimeSerieListResponse = result;
        let time_series = time_series_response.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, time_serie_filter : TimeSerieFilter, time_serie_search : TimeSerieSearch) -> Result<Vec<TimeSerie>> {
    let filter : Search = Search::new(time_serie_filter, time_serie_search, None);
    match self.api_client.post("timeseries/search", &filter){
      Ok(result) => {
        let time_series_response : TimeSerieListResponse = result;
        let time_series = time_series_response.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, time_serie_ids : &[u64]) -> Result<Vec<TimeSerie>> {
    let id_list : Vec<TimeSerieId> = time_serie_ids.iter().map(| ts_id | TimeSerieId::from(*ts_id)).collect();
    let id_items = Items::from(&id_list);
    match self.api_client.post("timeseries/byids", &id_items){
      Ok(result) => {
        let time_series_response : TimeSerieListResponse = result;
        let time_series = time_series_response.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, time_series : &[TimeSerie]) -> Result<Vec<TimeSerie>> {
    let patch_time_series : Vec<PatchTimeSerie> = time_series.iter().map(| a | PatchTimeSerie::from(a)).collect();
    let patch_time_series_items = Items::from(&patch_time_series);
    match self.api_client.post("timeseries/update", &patch_time_series_items){
      Ok(result) => {
        let time_series_response : TimeSerieListResponse = result;
        let time_series = time_series_response.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, time_serie_ids : &[u64]) -> Result<()> {
    let id_list : Vec<TimeSerieId> = time_serie_ids.iter().map(| ts_id | TimeSerieId::from(*ts_id)).collect();
    let id_items = Items::from(&id_list);
    match self.api_client.post::<::serde_json::Value, Items>("timeseries/delete", &id_items){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}