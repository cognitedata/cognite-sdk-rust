use std::collections::HashMap;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT, USER_AGENT};
use reqwest::Error;

use super::{Params};

pub struct ApiClient {
  api_base_url : String,
  api_key : String,
  client : Client,
}

impl ApiClient {
  pub fn new(api_base_url : String, api_key : String) -> ApiClient {
    ApiClient { 
      api_base_url : api_base_url,
      api_key : api_key,
      client : Client::new(),
    }
  }

  pub fn convert_params_to_tuples(&self, params : Option<Vec<Params>>) -> Vec<(String, String)> {
    let http_params : Vec<(String, String)> = match params {
        Some(list_all_params) => {
          let json_string = serde_json::to_string(&list_all_params).unwrap();
          let params_list : Vec<HashMap<String, String>> = serde_json::from_str(&json_string).unwrap();
          let param_tuples : Vec<(String, String)> = params_list.iter().map(| m | {
            let mut key : String = String::from("");
            let mut value : String = String::from("");
            for (k, v) in m {
              key = k.to_owned();
              value = v.to_owned();
            }
            (key, value)
          }).collect();
          param_tuples
        },
        None => vec!(),
    };
    http_params
  }

  pub fn get(&self, path : &str, params : Option<Vec<Params>>) -> Result<String, Error> {
    let http_params : Vec<(String, String)> = self.convert_params_to_tuples(params);

    let url = format!("{}/{}", self.api_base_url, path);
    let mut headers = HeaderMap::new();
    let api_key_header_value = HeaderValue::from_str(&self.api_key).expect("failed to set api key");
    headers.insert("api-key", api_key_header_value);
    headers.insert(USER_AGENT, HeaderValue::from_static("user-agent-goes-here"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
     
    match self.client
                .get(&url)
                .headers(headers)
                .query(&http_params)
                .send() {
      Ok(mut response) => {
        response.text()
      },
      Err(e) => {
        // do error handling
        Err(e)
      }
    }
  }

  pub fn post(&self, path : &str, body : &str) -> Result<String, Error> {
    let url = format!("{}/{}", self.api_base_url, path);
    let mut headers = HeaderMap::new();
    let api_key_header_value = HeaderValue::from_str(&self.api_key).expect("failed to set api key");
    headers.insert("api-key", api_key_header_value);
    headers.insert(USER_AGENT, HeaderValue::from_static("user-agent-goes-here"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    let response = self.client
                          .post(&url)
                          .headers(headers)
                          .body(String::from(body))
                          .send()?
                          .text()?;
    Ok(response)
  }

  pub fn put(&self, path : &str, body : &str) -> Result<String, Error> {
    let url = format!("{}/{}", self.api_base_url, path);
    let mut headers = HeaderMap::new();
    let api_key_header_value = HeaderValue::from_str(&self.api_key).expect("failed to set api key");
    headers.insert("api-key", api_key_header_value);
    headers.insert(USER_AGENT, HeaderValue::from_static("user-agent-goes-here"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    let response = self.client
                          .put(&url)
                          .headers(headers)
                          .body(String::from(body))
                          .send()?
                          .text()?;
    Ok(response)
  }

  pub fn delete(&self, path : &str) -> Result<String, Error> {
    let url = format!("{}/{}", self.api_base_url, path);
    let mut headers = HeaderMap::new();
    let api_key_header_value = HeaderValue::from_str(&self.api_key).expect("failed to set api key");
    headers.insert("api-key", api_key_header_value);
    headers.insert(USER_AGENT, HeaderValue::from_static("user-agent-goes-here"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    let response = self.client
                          .delete(&url)
                          .headers(headers)
                          .send()?
                          .text()?;
    Ok(response)
  }

}