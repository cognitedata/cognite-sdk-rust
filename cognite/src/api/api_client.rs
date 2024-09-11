use crate::reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT,
};
use crate::reqwest::{Body, Response, StatusCode};
use crate::reqwest_middleware::ClientWithMiddleware;
use crate::reqwest_middleware::RequestBuilder;
use crate::IntoParams;
use anyhow::anyhow;
use bytes::Bytes;
use futures::{TryStream, TryStreamExt};
use prost::Message;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::error::{Error, Result};

/// API client, used to query CDF.
pub struct ApiClient {
    api_base_url: String,
    app_name: String,
    client: ClientWithMiddleware,
}

const SDK_VERSION: &str = concat!("rust-sdk-v", env!("CARGO_PKG_VERSION"));

impl ApiClient {
    /// Create a new api client.
    ///
    /// # Arguments
    ///
    /// * `api_base_url` - Base URL for CDF. For example `https://api.cognitedata.com`
    /// * `app_name` - App name added to the `x-cdp-app` header.
    /// * `client` - Underlying reqwest client.
    pub fn new(api_base_url: &str, app_name: &str, client: ClientWithMiddleware) -> ApiClient {
        ApiClient {
            api_base_url: String::from(api_base_url),
            app_name: String::from(app_name),
            client,
        }
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

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

    async fn handle_error(&self, response: Response) -> Error {
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|x| x.to_str().ok())
            .map(|x| x.to_string());

        let status = response.status();

        match &response.text().await {
            Ok(s) => match serde_json::from_str(s) {
                Ok(error_message) => Error::new_from_cdf(status, error_message, request_id),
                Err(e) => Error::new_without_json(status, format!("{e}. Raw: {s}"), request_id),
            },
            Err(e) => Error::new_without_json(status, e.to_string(), request_id),
        }
    }

    async fn send_request_json<T: DeserializeOwned>(
        &self,
        mut request_builder: RequestBuilder,
    ) -> Result<T> {
        request_builder.extensions().insert(self.client.clone());
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

    async fn send_request_no_response(&self, mut request_builder: RequestBuilder) -> Result<()> {
        request_builder.extensions().insert(self.client.clone());
        match request_builder.send().await {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED => Ok(()),
                _ => Err(self.handle_error(response).await),
            },
            Err(e) => Err(Error::from(e)),
        }
    }

    async fn send_request_proto<T: Message + Default>(
        &self,
        mut request_builder: RequestBuilder,
    ) -> Result<T> {
        request_builder.extensions().insert(self.client.clone());
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

    /// Perform a get request to the given path, deserializing the result from JSON.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path, without leading slash.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self.client.get(&url).headers(headers);
        self.send_request_json(request_builder).await
    }

    /// Perform a get request to the given path, with a query given by `params`,
    /// then deserialize the result from JSON.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path, without leading slash.
    /// * `params` - Optional object converted to query parameters.
    pub async fn get_with_params<T: DeserializeOwned, R: IntoParams>(
        &self,
        path: &str,
        params: Option<R>,
    ) -> Result<T> {
        let http_params = match params {
            Some(params) => params.into_params(),
            None => return self.get::<T>(path).await,
        };

        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self.client.get(&url).headers(headers).query(&http_params);
        self.send_request_json(request_builder).await
    }

    /// Perform a get request to the given URL, returning a stream.
    ///
    /// # Arguments
    ///
    /// * `url` - Full URL to get stream from.
    pub async fn get_stream(
        &self,
        url: &str,
    ) -> Result<impl TryStream<Ok = bytes::Bytes, Error = crate::reqwest::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("x-cdp-sdk", HeaderValue::from_str(SDK_VERSION).expect(""));
        headers.insert(
            "x-cdp-app",
            HeaderValue::from_str(&self.app_name).expect(""),
        );
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
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

    /// Perform a post request to the given path, serializing `object` to JSON and sending it
    /// as the body, then deserialize the response from JSON.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path, without leading slash.
    /// * `object` - Object converted to JSON body.
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

    /// Perform a post request to the given path, assuming `body` is valid JSON, then
    /// deserialize the response from JSON.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path, without leading slash.
    /// * `body` - String body.
    pub async fn post_json<T: DeserializeOwned>(&self, path: &str, body: String) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self.client.post(&url).headers(headers).body(body);
        self.send_request_json(request_builder).await
    }

    /// Perform a post request to the given path, with query parameters given by `params`.
    /// Deserialize the response from JSON.
    ///
    /// * `path` - Request path, without leading slash.
    /// * `object` - Object converted to JSON body.
    /// * `params` - Object converted to query parameters.
    pub async fn post_with_query<D: DeserializeOwned, S: Serialize, R: IntoParams>(
        &self,
        path: &str,
        object: &S,
        params: Option<R>,
    ) -> Result<D> {
        let http_params = match params {
            Some(params) => params.into_params(),
            None => return self.post::<D, S>(path, object).await,
        };
        let json = match serde_json::to_string(object) {
            Ok(json) => json,
            Err(e) => return Err(Error::from(e)),
        };
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self
            .client
            .post(&url)
            .headers(headers)
            .query(&http_params)
            .body(json);
        self.send_request_json(request_builder).await
    }

    /// Perform a post request to the given path, posting `value` as protobuf.
    /// Expects JSON as response.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path without leading slash.
    /// * `value` - Protobuf value to post.
    pub async fn post_protobuf<D: DeserializeOwned, T: Message>(
        &self,
        path: &str,
        value: &T,
    ) -> Result<D> {
        let url = format!("{}/{}", self.api_base_url, path);
        let mut headers: HeaderMap = self.get_headers();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/protobuf"),
        );
        let request_builder = self
            .client
            .post(&url)
            .headers(headers)
            .body(value.encode_to_vec());
        self.send_request_json(request_builder).await
    }

    /// Perform a post request to the given path, send `object` as JSON in the body,
    /// then expect protobuf as response.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path without leading slash.
    /// * `object` - Object to convert to JSON and post.
    pub async fn post_expect_protobuf<D: Message + Default, S: Serialize>(
        &self,
        path: &str,
        object: &S,
    ) -> Result<D> {
        let url = format!("{}/{}", self.api_base_url, path);
        let mut headers: HeaderMap = self.get_headers();
        headers.insert(ACCEPT, HeaderValue::from_static("application/protobuf"));
        let json = match serde_json::to_string(object) {
            Ok(json) => json,
            Err(e) => return Err(Error::from(e)),
        };

        let request_builder = self.client.post(&url).headers(headers).body(json);
        self.send_request_proto(request_builder).await
    }

    /// Perform a put request with the data in `data`.
    ///
    /// # Arguments
    ///
    /// * `url` - URL to stream blob to.
    /// * `mime_type` - What to put in the `X-Upload_Content-Type` header.
    /// * `data` - Data to upload.
    pub async fn put_blob(&self, url: &str, mime_type: &str, data: impl Into<Bytes>) -> Result<()> {
        let mut headers: HeaderMap = self.get_headers();
        if !mime_type.is_empty() {
            headers.insert(CONTENT_TYPE, HeaderValue::from_str(mime_type)?);
            headers.insert("X-Upload-Content-Type", HeaderValue::from_str(mime_type)?);
        } else {
            headers.remove(CONTENT_TYPE);
        }
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));

        let bytes: Bytes = data.into();
        let request_builder = self.client.put(url).headers(headers).body(bytes);
        self.send_request_no_response(request_builder).await?;
        Ok(())
    }

    /// Perform a put request, streaming data to `url`.
    ///
    /// # Arguments
    ///
    /// * `url` - URL to stream blob to.
    /// * `mime_type` - What to put in the `X-Upload_Content-Type` header.
    /// * `stream` - Stream to upload.
    /// * `stream_chunked` - If `true`, use chunked streaming to upload the data.
    /// * `known_size` - Set the `Content-Length` header to this value.
    ///
    /// If `known_size` is `None` and `stream_chunked` is `true`, the request will be uploaded using
    /// special chunked streaming logic. Some backends do not support this.
    ///
    /// If `stream_chunked` is true and `known_size` is Some, this will include a content length header,
    /// it is highly recommended to set this whenever possible.
    ///
    /// # Warning
    /// If `stream_chunked` is false, this will collect the input stream into a memory, which can
    /// be _very_ expensive.
    ///
    pub async fn put_stream<S>(
        &self,
        url: &str,
        mime_type: &str,
        stream: S,
        stream_chunked: bool,
        known_size: Option<u64>,
    ) -> Result<()>
    where
        S: futures::TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
        bytes::Bytes: From<S::Ok>,
    {
        if stream_chunked {
            let mut headers: HeaderMap = self.get_headers();
            if !mime_type.is_empty() {
                headers.insert(CONTENT_TYPE, HeaderValue::from_str(mime_type)?);
                headers.insert("X-Upload-Content-Type", HeaderValue::from_str(mime_type)?);
            } else {
                headers.remove(CONTENT_TYPE);
            }
            headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
            if let Some(size) = known_size {
                headers.insert(CONTENT_LENGTH, HeaderValue::from_str(&size.to_string())?);
            }
            let request_builder = self
                .client
                .put(url)
                .headers(headers)
                .body(Body::wrap_stream(stream));
            self.send_request_no_response(request_builder).await?;
        } else {
            let body: Vec<S::Ok> = stream
                .try_collect()
                .await
                .map_err(|e| Error::StreamError(anyhow!(e.into())))?;
            let body: Vec<u8> = body
                .into_iter()
                .flat_map(Into::<bytes::Bytes>::into)
                .collect();
            self.put_blob(url, mime_type, body).await?;
        }

        Ok(())
    }

    /// Perform a put request to `path` with `object` as JSON, expecting JSON in return.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path without leading slash.
    /// * `object` - Object to send as JSON.
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

    /// Perform a put request to `path`, assuming `body` is JSON.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path without leading slash.
    /// * `body` - Request body JSON as string.
    pub async fn put_json<T: DeserializeOwned>(&self, path: &str, body: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self
            .client
            .put(&url)
            .headers(headers)
            .body(String::from(body));
        self.send_request_json(request_builder).await
    }

    /// Perform a delete request to `path`, expecting JSON as response.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path without leading slash.
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self.client.delete(&url).headers(headers);
        self.send_request_json::<T>(request_builder).await
    }

    /// Perform a delete request to `path`, with query parameters given by `params`.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path without leading slash.
    /// * `params` - Object converted to query parameters.
    pub async fn delete_with_params<T: DeserializeOwned, R: IntoParams>(
        &self,
        path: &str,
        params: Option<R>,
    ) -> Result<T> {
        let http_params = match params {
            Some(params) => params.into_params(),
            None => return self.delete::<T>(path).await,
        };

        let url = format!("{}/{}", self.api_base_url, path);
        let headers: HeaderMap = self.get_headers();
        let request_builder = self
            .client
            .delete(&url)
            .headers(headers)
            .query(&http_params);
        self.send_request_json::<T>(request_builder).await
    }

    /// Send an arbitrary HTTP request using the client. This will not parse the response,
    /// but will append authentication headers and retry with the same semantics as any
    /// normal API call.
    ///
    /// # Arguments
    ///
    /// * `request_builder` - Request to send.
    pub async fn send_request(&self, mut request_builder: RequestBuilder) -> Result<Response> {
        request_builder.extensions().insert(self.client.clone());

        Ok(request_builder.send().await?)
    }
}
