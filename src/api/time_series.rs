use crate::api::ApiClient;
use crate::api::params::{Params};
use crate::error::{Result};
use crate::dto::time_serie::*;

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
    match self.api_client.get::<TimeSerieResponseWrapper>("timeseries", params){
      Ok(time_series_response) => {
        let time_series = time_series_response.data.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve(&self, time_serie_id : u64, params : Option<Vec<Params>>) -> Result<TimeSerie> {
    match self.api_client.get::<TimeSerieResponseWrapper>(&format!("timeseries/{}", time_serie_id), params){
      Ok(mut time_serie_response) => {
        let time_serie = time_serie_response.data.items.pop().unwrap();
        Ok(time_serie)
      },
      Err(e) => Err(e)
    }
  }

  pub fn retrieve_multiple(&self, time_serie_ids : Vec<u64>) -> Result<Vec<TimeSerie>> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&time_serie_ids).unwrap());
    match self.api_client.post::<TimeSerieResponseWrapper>("timeseries/byids", &request_body){
      Ok(time_series_response) => {
        let time_series = time_series_response.data.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Result<Vec<TimeSerie>> {
    match self.api_client.get::<TimeSerieResponseWrapper>("timeseries/search", params){
      Ok(time_series_response) => {
        let time_series = time_series_response.data.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn create(&self, time_series : &[TimeSerie]) -> Result<()> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&time_series).unwrap());
    match self.api_client.post::<::serde_json::Value>("timeseries", &request_body){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

  pub fn update(&self, time_series : &[TimeSerie]) -> Result<Vec<TimeSerie>> {
    let patch_time_series : Vec<PatchTimeSerie> = time_series.iter().map(| a | PatchTimeSerie::new(a)).collect();
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&patch_time_series).unwrap());
    println!("{:?}", request_body);
    match self.api_client.post::<TimeSerieResponseWrapper>("timeseries/update", &request_body){
      Ok(time_series_response) => {
        let time_series = time_series_response.data.items;
        Ok(time_series)
      },
      Err(e) => Err(e)
    }
  }

  pub fn delete(&self, time_serie_name: &str) -> Result<()> {
    match self.api_client.delete::<::serde_json::Value>(&format!("timeseries/{}", time_serie_name)){
      Ok(_) => {
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}