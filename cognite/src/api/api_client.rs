use crate::reqwest::header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use crate::reqwest::{Body, Response};
use crate::reqwest_middleware::ClientWithMiddleware;
use crate::IntoParams;
use anyhow::anyhow;
use bytes::Bytes;
use futures::{TryStream, TryStreamExt};
use prost::Message;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::error::{Error, Result};

use super::request_builder::RequestBuilder;

/// API client, used to query CDF.
pub struct ApiClient {
    api_base_url: String,
    app_name: String,
    client: ClientWithMiddleware,
}

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

    /// Perform a get request to the given path, deserializing the result from JSON.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path, without leading slash.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        RequestBuilder::<()>::get(self, format!("{}/{}", self.api_base_url, path))
            .accept_json()
            .send()
            .await
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
        let mut b = RequestBuilder::<()>::get(self, format!("{}/{}", self.api_base_url, path))
            .accept_json();

        if let Some(params) = params {
            b = b.query(&params.into_params());
        }

        b.send().await
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
        let r = RequestBuilder::<()>::get(self, url)
            .accept_raw()
            .send()
            .await?;
        Ok(r.bytes_stream())
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
        RequestBuilder::<()>::post(self, format!("{}/{}", self.api_base_url, path))
            .json(object)?
            .accept_json()
            .send()
            .await
    }

    /// Create a request builder for a `GET` request to `path`.
    pub fn get_request(&self, path: &str) -> RequestBuilder<'_, ()> {
        RequestBuilder::<()>::get(self, format!("{}/{}", self.api_base_url, path))
    }

    /// Create a request builder for a `POST` request to `path`.
    pub fn post_request(&self, path: &str) -> RequestBuilder<'_, ()> {
        RequestBuilder::<()>::post(self, format!("{}/{}", self.api_base_url, path))
    }

    /// Create a request builder for a `PUT` request to `path`.
    pub fn put_request(&self, path: &str) -> RequestBuilder<'_, ()> {
        RequestBuilder::<()>::put(self, format!("{}/{}", self.api_base_url, path))
    }

    /// Create a request builder for a `Delete` request to `path`.
    pub fn delete_request(&self, path: &str) -> RequestBuilder<'_, ()> {
        RequestBuilder::<()>::delete(self, format!("{}/{}", self.api_base_url, path))
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
        let mut b = self.post_request(path).json(object)?;
        if let Some(params) = params {
            b = b.query(&params.into_params());
        }
        b.accept_json().send().await
    }

    /// Perform a post request to the given path, posting `value` as protobuf.
    /// Expects JSON as response.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path without leading slash.
    /// * `value` - Protobuf value to post.
    pub async fn post_protobuf<D: DeserializeOwned + Send + Sync, T: Message>(
        &self,
        path: &str,
        value: &T,
    ) -> Result<D> {
        self.post_request(path)
            .protobuf(value)
            .accept_json()
            .send()
            .await
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
        self.post_request(path)
            .json(object)?
            .accept_protobuf()
            .send()
            .await
    }

    /// Perform a put request with the data in `data`.
    ///
    /// # Arguments
    ///
    /// * `url` - URL to stream blob to.
    /// * `mime_type` - What to put in the `X-Upload_Content-Type` header.
    /// * `data` - Data to upload.
    pub async fn put_blob(&self, url: &str, mime_type: &str, data: impl Into<Bytes>) -> Result<()> {
        let bytes: Bytes = data.into();
        let mut b = RequestBuilder::<()>::put(self, url)
            .body(bytes)
            .accept_nothing();
        if !mime_type.is_empty() {
            b = b
                .header(CONTENT_TYPE, HeaderValue::from_str(mime_type)?)
                .header("X-Upload-Content-Type", HeaderValue::from_str(mime_type)?);
        }
        b.send().await
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
        let mut b = RequestBuilder::<()>::put(self, url).accept_nothing();
        if !mime_type.is_empty() {
            b = b
                .header(CONTENT_TYPE, HeaderValue::from_str(mime_type)?)
                .header("X-Upload-Content-Type", HeaderValue::from_str(mime_type)?);
        }

        if stream_chunked {
            if let Some(size) = known_size {
                b = b.header(CONTENT_LENGTH, HeaderValue::from_str(&size.to_string())?);
            }
            b = b.body(Body::wrap_stream(stream));
        } else {
            let body: Vec<S::Ok> = stream
                .try_collect()
                .await
                .map_err(|e| Error::StreamError(anyhow!(e.into())))?;
            let body: Vec<u8> = body
                .into_iter()
                .flat_map(Into::<bytes::Bytes>::into)
                .collect();
            b = b.body(body);
        }

        b.send().await
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
        self.put_request(path)
            .json(object)?
            .accept_json()
            .send()
            .await
    }

    /// Perform a delete request to `path`, expecting JSON as response.
    ///
    /// # Arguments
    ///
    /// * `path` - Request path without leading slash.
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.delete_request(path).accept_json().send().await
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
        let mut b = self.delete_request(path).accept_json();
        if let Some(params) = params {
            b = b.query(&params.into_params());
        }
        b.send().await
    }

    /// Send an arbitrary HTTP request using the client. This will not parse the response,
    /// but will append authentication headers and retry with the same semantics as any
    /// normal API call.
    ///
    /// # Arguments
    ///
    /// * `request_builder` - Request to send.
    pub async fn send_request(
        &self,
        mut request_builder: crate::reqwest_middleware::RequestBuilder,
    ) -> Result<Response> {
        request_builder.extensions().insert(self.client.clone());

        Ok(request_builder.send().await?)
    }

    /// Get the inner HTTP client.
    pub fn client(&self) -> &ClientWithMiddleware {
        &self.client
    }

    /// Get the configured app name.
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    /// Get the configured API base URL.
    pub fn api_base_url(&self) -> &str {
        &self.api_base_url
    }
}
