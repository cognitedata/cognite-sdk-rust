use crate::api::authenticator::Authenticator;
use crate::AsParams;
use futures::Stream;
use prost::Message;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::{Body, Client, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::error::{ApiErrorWrapper, Error, Result};

pub struct ApiClient {
    api_base_url: String,
    api_key: Option<String>,
    app_name: String,
    client: Client,
    authenticator: Option<Authenticator>,
}

const SDK_VERSION: &str = concat!("rust-sdk-v", env!("CARGO_PKG_VERSION"));

impl ApiClient {
    pub fn new(api_base_url: &str, api_key: &str, app_name: &str, client: Client) -> ApiClient {
        ApiClient {
            api_base_url: String::from(api_base_url),
            api_key: Some(String::from(api_key)),
            app_name: String::from(app_name),
            client,
            authenticator: None,
        }
    }

    pub fn new_oidc(
        api_base_url: &str,
        auth: Authenticator,
        app_name: &str,
        client: Client,
    ) -> ApiClient {
        ApiClient {
            api_base_url: String::from(api_base_url),
            api_key: None,
            app_name: String::from(app_name),
            client,
            authenticator: Some(auth),
        }
    }

    async fn get_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        if let Some(api_key) = &self.api_key {
            let api_key_header_value =
                HeaderValue::from_str(api_key).expect("failed to set api key");
            headers.insert("api-key", api_key_header_value);
        }

        if let Some(authenticator) = &self.authenticator {
            let token = authenticator.get_token(&self.client).await?;
            let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", token))
                // Should not be possible
                .expect("Failed to set authorization bearer token");
            headers.insert("Authorization", auth_header_value);
        }

        headers.insert("x-cdp-sdk", HeaderValue::from_str(SDK_VERSION).expect(""));
        headers.insert(
            "x-cdp-app",
            HeaderValue::from_str(&self.app_name).expect(""),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("user-agent-goes-here"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        Ok(headers)
    }

    async fn handle_error(&self, response: Response) -> Error {
        let request_id = match response.headers().get("x-request-id") {
            Some(x) => match x.to_str() {
                Ok(rid) => Some(rid.to_string()),
                Err(_) => None,
            },
            None => None,
        };

        let status = response.status();

        match response.json::<ApiErrorWrapper>().await {
            Ok(error_message) => Error::new_from_cdf(status, error_message, request_id),
            Err(e) => Error::new_without_json(status, e.to_string(), request_id),
        }
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<T> {
        match request_builder.send().await {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED => {
                    match response.json::<T>().await {
                        Ok(json) => Ok(json),
                        Err(e) => Err(Error::from(e)),
                    }
                }
                _ => Err(self.handle_error(response).await),
            },
            Err(e) => Err(Error::from(e)),
        }
    }

    async fn send_request_proto<T: Message + Default>(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<T> {
        match request_builder.send().await {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED => {
                    match response.bytes().await {
                        Ok(buf) => match T::decode(buf) {
                            Ok(r) => Ok(r),
                            Err(e) => Err(Error::from(e)),
                        },
                        Err(e) => Err(Error::from(e)),
                    }
                }
                _ => Err(self.handle_error(response).await),
            },
            Err(e) => Err(Error::from(e)),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers().await?;
        let request_builder = self.client.get(&url).headers(headers);
        self.send_request(request_builder).await
    }

    pub async fn get_with_params<T: DeserializeOwned, R: AsParams>(
        &self,
        path: &str,
        params: Option<R>,
    ) -> Result<T> {
        let http_params = match params {
            Some(params) => params.to_tuples(),
            None => return self.get::<T>(path).await,
        };

        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers().await?;
        let request_builder = self.client.get(&url).headers(headers).query(&http_params);
        self.send_request(request_builder).await
    }

    pub async fn get_stream(
        &self,
        url: &str,
    ) -> Result<impl Stream<Item = std::result::Result<bytes::Bytes, reqwest::Error>>> {
        let headers: HeaderMap = self.get_headers().await?;
        let request_builder = self.client.get(url).headers(headers);
        match request_builder.send().await {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED => {
                    Ok(response.bytes_stream())
                }
                _ => Err(self.handle_error(response).await),
            },
            Err(e) => Err(Error::from(e)),
        }
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
        self.post_json(path, json).await
    }

    pub async fn post_json<T: DeserializeOwned>(&self, path: &str, body: String) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers().await?;
        let request_builder = self.client.post(&url).headers(headers).body(body);
        self.send_request(request_builder).await
    }

    pub async fn post_with_query<D: DeserializeOwned, S: Serialize, R: AsParams>(
        &self,
        path: &str,
        object: &S,
        params: Option<R>,
    ) -> Result<D> {
        let http_params = match params {
            Some(params) => params.to_tuples(),
            None => return self.post::<D, S>(path, object).await,
        };
        let json = match serde_json::to_string(object) {
            Ok(json) => json,
            Err(e) => return Err(Error::from(e)),
        };
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers().await?;
        let request_builder = self
            .client
            .post(&url)
            .headers(headers)
            .query(&http_params)
            .body(json);
        self.send_request(request_builder).await
    }

    pub async fn post_protobuf<D: DeserializeOwned, T: Message>(
        &self,
        path: &str,
        value: &T,
    ) -> Result<D> {
        let url = format!("{}/{}", self.api_base_url, path);
        let mut headers: HeaderMap = self.get_headers().await?;
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/protobuf"),
        );
        let request_builder = self
            .client
            .post(&url)
            .headers(headers)
            .body(value.encode_to_vec());
        self.send_request(request_builder).await
    }

    pub async fn post_expect_protobuf<D: Message + Default, S: Serialize>(
        &self,
        path: &str,
        object: &S,
    ) -> Result<D> {
        let url = format!("{}/{}", self.api_base_url, path);
        let mut headers: HeaderMap = self.get_headers().await?;
        headers.insert(ACCEPT, HeaderValue::from_static("application/protobuf"));
        let json = match serde_json::to_string(object) {
            Ok(json) => json,
            Err(e) => return Err(Error::from(e)),
        };

        let request_builder = self.client.post(&url).headers(headers).body(json);
        self.send_request_proto(request_builder).await
    }

    pub async fn put_stream<S>(&self, url: &str, mime_type: &str, stream: S) -> Result<()>
    where
        S: futures::TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        let mut headers: HeaderMap = self.get_headers().await?;
        headers.insert(CONTENT_TYPE, HeaderValue::from_str(mime_type)?);
        headers.insert("X-Upload-Content-Type", HeaderValue::from_str(mime_type)?);
        let request_builder = self
            .client
            .put(url)
            .headers(headers)
            .body(Body::wrap_stream(stream));
        self.send_request::<serde_json::Value>(request_builder)
            .await?;
        Ok(())
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
        let headers: HeaderMap = self.get_headers().await?;
        let request_builder = self
            .client
            .put(&url)
            .headers(headers)
            .body(String::from(body));
        self.send_request(request_builder).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers().await?;
        let request_builder = self.client.delete(&url).headers(headers);
        self.send_request::<T>(request_builder).await
    }

    pub async fn delete_with_params<T: DeserializeOwned, R: AsParams>(
        &self,
        path: &str,
        params: Option<R>,
    ) -> Result<T> {
        let http_params = match params {
            Some(params) => params.to_tuples(),
            None => return self.delete::<T>(path).await,
        };

        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers().await?;
        let request_builder = self
            .client
            .delete(&url)
            .headers(headers)
            .query(&http_params);
        self.send_request::<T>(request_builder).await
    }
}
