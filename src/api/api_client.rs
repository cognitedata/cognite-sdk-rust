use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::collections::HashMap;

use crate::dto::params::Params;
use crate::error::{ApiErrorWrapper, Error, Kind, Result};

pub struct ApiClient {
    api_base_url: String,
    api_key: String,
    app_name: String,
    client: Client,
}

const SDK_VERSION: &str = concat!("rust-sdk-v", env!("CARGO_PKG_VERSION"));

impl ApiClient {
    pub fn new(api_base_url: &str, api_key: &str, app_name: &str) -> ApiClient {
        ApiClient {
            api_base_url: String::from(api_base_url),
            api_key: String::from(api_key),
            app_name: String::from(app_name),
            client: Client::new(),
        }
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let api_key_header_value =
            HeaderValue::from_str(&self.api_key).expect("failed to set api key");
        headers.insert("api-key", api_key_header_value);
        headers.insert("x-cdp-sdk", HeaderValue::from_str(SDK_VERSION).expect(""));
        headers.insert(
            "x-cdp-app",
            HeaderValue::from_str(&self.app_name).expect(""),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("user-agent-goes-here"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<T> {
        match request_builder.send().await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<T>().await {
                    Ok(json) => Ok(json),
                    Err(e) => Err(Error::from(e)),
                },
                StatusCode::CREATED => match response.json::<T>().await {
                    Ok(json) => Ok(json),
                    Err(e) => Err(Error::from(e)),
                },
                StatusCode::BAD_REQUEST => match response.json::<ApiErrorWrapper>().await {
                    Ok(error_message) => {
                        Err(Error::new(Kind::BadRequest(error_message.error.message)))
                    }
                    Err(e) => Err(Error::new_reqwest_error_with_kind(
                        e,
                        Kind::BadRequest("400".to_owned()),
                    )),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ApiErrorWrapper>().await {
                    Ok(error_message) => {
                        Err(Error::new(Kind::Unauthorized(error_message.error.message)))
                    }
                    Err(e) => Err(Error::new_reqwest_error_with_kind(
                        e,
                        Kind::Unauthorized("401".to_owned()),
                    )),
                },
                StatusCode::FORBIDDEN => match response.json::<ApiErrorWrapper>().await {
                    Ok(error_message) => {
                        Err(Error::new(Kind::Forbidden(error_message.error.message)))
                    }
                    Err(e) => Err(Error::new_reqwest_error_with_kind(
                        e,
                        Kind::Forbidden("403".to_owned()),
                    )),
                },
                StatusCode::NOT_FOUND => match response.json::<ApiErrorWrapper>().await {
                    Ok(error_message) => {
                        Err(Error::new(Kind::NotFound(error_message.error.message)))
                    }
                    Err(e) => Err(Error::new_reqwest_error_with_kind(
                        e,
                        Kind::NotFound("404".to_owned()),
                    )),
                },
                StatusCode::CONFLICT => match response.json::<ApiErrorWrapper>().await {
                    Ok(error_message) => {
                        Err(Error::new(Kind::Conflict(error_message.error.message)))
                    }
                    Err(e) => Err(Error::new_reqwest_error_with_kind(
                        e,
                        Kind::Conflict("409".to_owned()),
                    )),
                },
                StatusCode::UNPROCESSABLE_ENTITY => {
                    match response.json::<ApiErrorWrapper>().await {
                        Ok(error_message) => Err(Error::new(Kind::UnprocessableEntity(
                            error_message.error.message,
                        ))),
                        Err(e) => Err(Error::new_reqwest_error_with_kind(
                            e,
                            Kind::UnprocessableEntity("422".to_owned()),
                        )),
                    }
                }
                s => {
                    let error_message = format!(
                        "Received API response {} with result: {:?}",
                        s,
                        response.text().await
                    );
                    Err(Error::new(Kind::Http(error_message)))
                }
            },
            Err(e) => Err(Error::from(e)),
        }
    }

    fn convert_params_to_tuples(&self, params: Option<Vec<Params>>) -> Vec<(String, String)> {
        let http_params: Vec<(String, String)> = match params {
            Some(list_all_params) => {
                let json_string = serde_json::to_string(&list_all_params).unwrap();
                let params_list: Vec<HashMap<String, String>> =
                    serde_json::from_str(&json_string).unwrap();
                let param_tuples: Vec<(String, String)> = params_list
                    .iter()
                    .map(|m| {
                        let mut key: String = String::from("");
                        let mut value: String = String::from("");
                        for (k, v) in m {
                            key = k.to_owned();
                            value = v.to_owned();
                        }
                        (key, value)
                    })
                    .collect();
                param_tuples
            }
            None => vec![],
        };
        http_params
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.get_with_params::<T>(path, None).await
    }

    pub async fn get_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: Option<Vec<Params>>,
    ) -> Result<T> {
        let http_params: Vec<(String, String)> = self.convert_params_to_tuples(params);

        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self.client.get(&url).headers(headers).query(&http_params);
        self.send_request(request_builder).await
    }

    pub async fn post<D, S>(&self, path: &str, object: &S) -> Result<D>
    where
        D: DeserializeOwned,
        S: Serialize,
    {
        let json = match serde_json::to_string(object) {
            Ok(json) => json,
            Err(e) => return Err(Error::from(e)),
        };
        self.post_json(path, &json).await
    }

    pub async fn post_json<T: DeserializeOwned>(&self, path: &str, body: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self
            .client
            .post(&url)
            .headers(headers)
            .body(String::from(body));
        self.send_request(request_builder).await
    }

    pub async fn put<D, S>(&self, path: &str, object: &S) -> Result<D>
    where
        D: DeserializeOwned,
        S: Serialize,
    {
        let json = match serde_json::to_string(object) {
            Ok(json) => json,
            Err(e) => return Err(Error::from(e)),
        };
        self.put_json(path, &json).await
    }

    pub async fn put_json<T: DeserializeOwned>(&self, path: &str, body: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self
            .client
            .put(&url)
            .headers(headers)
            .body(String::from(body));
        self.send_request(request_builder).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.delete_with_params::<T>(path, None).await
    }

    pub async fn delete_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: Option<Vec<Params>>,
    ) -> Result<T> {
        let http_params: Vec<(String, String)> = self.convert_params_to_tuples(params);
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self
            .client
            .delete(&url)
            .headers(headers)
            .query(&http_params);
        self.send_request::<T>(request_builder).await
    }
}
