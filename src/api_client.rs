use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT, USER_AGENT};
use reqwest::Error;

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

  pub fn get(&self, path : String) -> Result<String, Error> {
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

  pub fn post(&self, path : String, body : String) -> Result<String, Error> {
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
                          .body(body)
                          .send()?
                          .text()?;
    Ok(response)
  }

  pub fn put(&self, path : String, body : String) -> Result<String, Error> {
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
                          .body(body)
                          .send()?
                          .text()?;
    Ok(response)
  }

  pub fn delete(&self, path : String) -> Result<String, Error> {
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