use reqwest::header::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};

use prost::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::ApiClient;
use crate::Error;
use reqwest::{IntoUrl, Response};

use crate::Result;

use super::{
    JsonResponseHandler, NoResponseHandler, ProtoResponseHandler, RawResponseHandler,
    ResponseHandler,
};

/// Generic request builder. Used to construct custom requests towards CDF.
pub struct RequestBuilder<'a, T = ()> {
    inner: reqwest_middleware::RequestBuilder,
    client: &'a ApiClient,
    output: T,
}

impl<'a, T> RequestBuilder<'a, T> {
    /// Create a POST request to `url`.
    ///
    /// # Arguments
    ///
    /// * `client` - API Client
    /// * `url` - URL to send request to.
    pub fn post(client: &'a ApiClient, url: impl IntoUrl) -> RequestBuilder<'a, ()> {
        RequestBuilder {
            inner: client.client().post(url),
            client,
            output: (),
        }
    }

    /// Create a GET request to `url`.
    ///
    /// # Arguments
    ///
    /// * `client` - API Client
    /// * `url` - URL to send request to.
    pub fn get(client: &'a ApiClient, url: impl IntoUrl) -> RequestBuilder<'a, ()> {
        RequestBuilder {
            inner: client.client().get(url),
            client,
            output: (),
        }
    }

    /// Create a DELETE request to `url`.
    ///
    /// # Arguments
    ///
    /// * `client` - API Client
    /// * `url` - URL to send request to.
    pub fn delete(client: &'a ApiClient, url: impl IntoUrl) -> RequestBuilder<'a, ()> {
        RequestBuilder {
            inner: client.client().delete(url),
            client,
            output: (),
        }
    }

    /// Create a PUT request to `url`.
    ///
    /// # Arguments
    ///
    /// * `client` - API Client
    /// * `url` - URL to send request to.
    pub fn put(client: &'a ApiClient, url: impl IntoUrl) -> RequestBuilder<'a, ()> {
        RequestBuilder {
            inner: client.client().put(url),
            client,
            output: (),
        }
    }

    /// Add a header to the request.
    ///
    /// # Arguments
    ///
    /// * `key` - Header key
    /// * `value` - Header value
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        self.inner = self.inner.header(key, value);
        self
    }

    /// Add a query to the request.
    pub fn query<Q: Serialize + ?Sized>(mut self, query: &Q) -> Self {
        self.inner = self.inner.query(query);
        self
    }

    /// Add a JSON body to the request. This sets `CONTENT_TYPE`.
    pub fn json<B: Serialize + ?Sized>(mut self, body: &B) -> Result<Self> {
        self.inner = self.inner.header(
            CONTENT_TYPE,
            const { HeaderValue::from_static("application/json") },
        );
        Ok(self.body(serde_json::to_vec(body)?))
    }

    /// Add a body to the request. You will typically want to set `CONTENT_TYPE` yourself.
    pub fn body(mut self, body: impl Into<reqwest::Body>) -> Self {
        self.inner = self.inner.body(body);
        self
    }

    /// Add a protobuf message as body to the request. This sets `CONTENT_TYPE`
    pub fn protobuf<B: Message>(mut self, body: &B) -> Self {
        self.inner = self.inner.header(
            CONTENT_TYPE,
            const { HeaderValue::from_static("application/protobuf") },
        );
        self.body(body.encode_to_vec())
    }

    /// Modify the inner request builder.
    pub fn with_inner<
        R: FnOnce(reqwest_middleware::RequestBuilder) -> reqwest_middleware::RequestBuilder,
    >(
        mut self,
        m: R,
    ) -> Self {
        self.inner = m(self.inner);
        self
    }
}

impl<'a> RequestBuilder<'a, ()> {
    /// Expect the response for a successful request to be `T` encoded as JSON.
    pub fn accept_json<T: DeserializeOwned>(self) -> RequestBuilder<'a, JsonResponseHandler<T>> {
        self.accept(JsonResponseHandler::new())
    }

    /// Expect the response for a successful request to be `T` encoded as protobuf.
    pub fn accept_protobuf<T: Message + Default + Send + Sync>(
        self,
    ) -> RequestBuilder<'a, ProtoResponseHandler<T>> {
        self.accept(ProtoResponseHandler::new())
    }

    /// Ignore the response for a successful request.
    pub fn accept_nothing(self) -> RequestBuilder<'a, NoResponseHandler> {
        self.accept(NoResponseHandler)
    }

    /// Simply return the raw payload on a successful request.
    pub fn accept_raw(self) -> RequestBuilder<'a, RawResponseHandler> {
        self.accept(RawResponseHandler)
    }

    /// Set the response handler `T`.
    pub fn accept<T: ResponseHandler>(self, handler: T) -> RequestBuilder<'a, T> {
        RequestBuilder {
            inner: self
                .inner
                .header(ACCEPT, const { HeaderValue::from_static(T::ACCEPT_HEADER) }),
            client: self.client,
            output: handler,
        }
    }
}

async fn handle_error(response: Response) -> Error {
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

const SDK_USER_AGENT: &str = concat!("CogniteSdkRust/", env!("CARGO_PKG_VERSION"));
const SDK_VERSION: &str = concat!("rust-sdk-v", env!("CARGO_PKG_VERSION"));

impl<T: ResponseHandler> RequestBuilder<'_, T> {
    /// Send the request. This sets a few core headers, and converts any errors into
    /// [crate::Error]
    pub async fn send(mut self) -> Result<T::Output> {
        self.inner.extensions().insert(self.client.client().clone());

        self.inner = self
            .inner
            .header(
                USER_AGENT,
                const { HeaderValue::from_static(SDK_USER_AGENT) },
            )
            .header("x-cdp-sdk", const { HeaderValue::from_static(SDK_VERSION) })
            .header(
                "x-cdp-app",
                HeaderValue::from_str(self.client.app_name()).expect("Invalid app name"),
            );

        match self.inner.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    self.output.handle_response(response).await
                } else {
                    Err(handle_error(response).await)
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
