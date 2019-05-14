use std::collections::HashMap;
use serde::de::DeserializeOwned;
use reqwest::{
  Client,
  StatusCode,
  RequestBuilder,
};
use reqwest::header::{
  HeaderMap, 
  HeaderValue, 
  CONTENT_TYPE, 
  ACCEPT, 
  USER_AGENT
};

use crate::dto::params::{Params};
use crate::error::{
  Result,
  Error,
  Kind,
  ApiErrorWrapper,
};

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

  fn get_headers(&self) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let api_key_header_value = HeaderValue::from_str(&self.api_key).expect("failed to set api key");
    headers.insert("api-key", api_key_header_value);
    headers.insert("x-cdp-sdk", HeaderValue::from_str("rust-sdk-v0.1").expect("x-cdp-sdk"));
    headers.insert("x-cdp-app", HeaderValue::from_str("").expect("x-cdp-app"));
    headers.insert(USER_AGENT, HeaderValue::from_static("user-agent-goes-here"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers
  }

  fn send_request<T : DeserializeOwned>(&self, request : RequestBuilder) -> Result<T> {
    match request.send() {
      Ok(mut response) => {
        match response.status() {
          StatusCode::OK => {
            match response.json::<T>() {
              Ok(json) => Ok(json),
              Err(e) => Err(Error::from(e))
            }
          },
          StatusCode::BAD_REQUEST => {
            match response.json::<ApiErrorWrapper>() {
              Ok(error_message) => Err(Error::new(Kind::BadRequest(error_message.error.message))),
              Err(e) => Err(Error::from(e))
            }
          },
          StatusCode::UNAUTHORIZED => {
            match response.json::<ApiErrorWrapper>() {
              Ok(error_message) => Err(Error::new(Kind::Unauthorized(error_message.error.message))),
              Err(e) => Err(Error::from(e))
            }
          },
          StatusCode::FORBIDDEN => {
            match response.json::<ApiErrorWrapper>() {
              Ok(error_message) => Err(Error::new(Kind::Forbidden(error_message.error.message))),
              Err(e) => Err(Error::from(e))
            }
          },
          StatusCode::NOT_FOUND => {
            match response.json::<ApiErrorWrapper>() {
              Ok(error_message) => Err(Error::new(Kind::NotFound(error_message.error.message))),
              Err(e) => Err(Error::from(e))
            }
          },
          StatusCode::CONFLICT => {
            match response.json::<ApiErrorWrapper>() {
              Ok(error_message) => Err(Error::new(Kind::Conflict(error_message.error.message))),
              Err(e) => Err(Error::from(e))
            }
          },
          StatusCode::UNPROCESSABLE_ENTITY => {
            match response.json::<ApiErrorWrapper>() {
              Ok(error_message) => Err(Error::new(Kind::UnprocessableEntity(error_message.error.message))),
              Err(e) => Err(Error::from(e))
            }
          },
          s => {
            let error_message = format!("Received API response {} with result: {:?}", s, response.text());
            Err(Error::new(Kind::Http(error_message)))
          }
        }
      },
      Err(e) => {
        Err(Error::from(e))
      }
    }
  }

  fn convert_params_to_tuples(&self, params : Option<Vec<Params>>) -> Vec<(String, String)> {
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

  pub fn get<T : DeserializeOwned>(&self, path : &str) -> Result<T> {
    self.get_with_params::<T>(path, None)
  }

  pub fn get_with_params<T : DeserializeOwned>(&self, path : &str, params : Option<Vec<Params>>) -> Result<T> {
    let http_params : Vec<(String, String)> = self.convert_params_to_tuples(params);

    let url = format!("{}/{}", self.api_base_url, path);
    let headers : HeaderMap = self.get_headers();
    let request = self.client
                    .get(&url)
                    .headers(headers)
                    .query(&http_params);
    self.send_request(request)
  }

  pub fn post<T : DeserializeOwned>(&self, path : &str, body : &str) -> Result<T> {
    let url = format!("{}/{}", self.api_base_url, path);
    let headers : HeaderMap = self.get_headers();
    let request = self.client
                    .post(&url)
                    .headers(headers)
                    .body(String::from(body));
    self.send_request(request)
  }

  pub fn put<T : DeserializeOwned>(&self, path : &str, body : &str) -> Result<T> {
    let url = format!("{}/{}", self.api_base_url, path);
    let headers : HeaderMap = self.get_headers();
    let request = self.client
                          .put(&url)
                          .headers(headers)
                          .body(String::from(body));
    self.send_request(request)
  }

  pub fn delete<T : DeserializeOwned>(&self, path : &str) -> Result<T> {
    self.delete_with_params::<T>(path, None)
  }

  pub fn delete_with_params<T : DeserializeOwned>(&self, path : &str, params : Option<Vec<Params>>) -> Result<T> {
    let http_params : Vec<(String, String)> = self.convert_params_to_tuples(params);
    let url = format!("{}/{}", self.api_base_url, path);
    let headers : HeaderMap = self.get_headers();
    let request = self.client
                          .delete(&url)
                          .headers(headers)
                          .query(&http_params);
    self.send_request::<T>(request)
  }

}

