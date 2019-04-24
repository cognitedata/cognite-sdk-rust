use super::{
  ApiClient, 
  Params,
  Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieResponseWrapper {
  data : TimeSerieListResponse
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerieListResponse {
  items : Vec<TimeSerie>,
  previous_cursor : Option<String>,
  next_cursor : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSerie {
  name: String,
  is_string: bool,
  unit: Option<String>,
  asset_id: u64,
  is_step: bool,
  description: String,
  security_categories: Option<Vec<u64>>,
  id: u64,
  created_time: u64,
  last_updated_time: u64
}

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

  pub fn create(&self, time_series : Vec<TimeSerie>) -> TimeSerie {
    unimplemented!();
  }

  pub fn update(&self, time_series : Vec<TimeSerie>) -> Vec<TimeSerie> {
    unimplemented!();
  }

  pub fn delete(&self, time_serie_ids : Vec<u64>) -> () {
    unimplemented!();
  }
}