use super::{
  ApiClient, 
  Params,
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

  pub fn list_all(&self, params : Option<Vec<Params>>) -> Vec<TimeSerie> {
    let time_series_response_json = self.api_client.get("timeseries", params).unwrap();
    let time_series_response : TimeSerieResponseWrapper = serde_json::from_str(&time_series_response_json).unwrap();
    let time_series = time_series_response.data.items;
    time_series
  }

  pub fn retrieve(&self, time_serie_id : u64, params : Option<Vec<Params>>) -> TimeSerie {
    let time_serie_response_json = self.api_client.get(&format!("timeseries/{}", time_serie_id), params).unwrap();
    let mut time_serie_response : TimeSerieResponseWrapper = serde_json::from_str(&time_serie_response_json).unwrap();
    let time_serie = time_serie_response.data.items.pop().unwrap();
    time_serie
  }

  pub fn retrieve_multiple(&self, time_serie_ids : Vec<u64>) -> Vec<TimeSerie> {
    let request_body = format!("{{\"items\":{} }}", serde_json::to_string(&time_serie_ids).unwrap());
    let time_series_response_json = self.api_client.post("timeseries/byids", &request_body).unwrap();
    let time_series_response : TimeSerieResponseWrapper = serde_json::from_str(&time_series_response_json).unwrap();
    let time_series = time_series_response.data.items;
    time_series
  }

  pub fn search(&self, params : Option<Vec<Params>>) -> Vec<TimeSerie> {
    let time_series_response_json = self.api_client.get("timeseries/search", params).unwrap();
    let time_series_response : TimeSerieResponseWrapper = serde_json::from_str(&time_series_response_json).unwrap();
    let time_series = time_series_response.data.items;
    time_series
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