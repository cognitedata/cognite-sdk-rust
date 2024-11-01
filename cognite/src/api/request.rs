use std::future::Future;
use std::marker::PhantomData;

use http::header::ACCEPT;
use http::header::CONTENT_TYPE;
use http::header::USER_AGENT;
use http::HeaderName;
use http::HeaderValue;
use prost::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::reqwest::{IntoUrl, Response};
use crate::ApiClient;
use crate::Error;

use crate::Result;

/// Trait for a type that produces a typed response from a successful
/// HTTP response message.
pub trait ResponseHandler {
    /// Output type.
    type Output;
    /// Accept header added to request.
    const ACCEPT_HEADER: &'static str;

    /// Consume the response and produce an instance of `Output`
    ///
    /// # Arguments
    ///
    /// * `response` - Response from API.
    fn handle_response(
        self,
        response: Response,
    ) -> impl Future<Output = Result<Self::Output>> + Send + Sync;
}

// Uses fn() -> T since that is covariant, and allows us to have a Send future
// without T being Send, technically.
/// Response handler for parsing a payload as JSON.
pub struct JsonResponseHandler<T>(PhantomData<fn() -> T>);

impl<T: DeserializeOwned> ResponseHandler for JsonResponseHandler<T> {
    type Output = T;
    const ACCEPT_HEADER: &'static str = "application/json";
    async fn handle_response(self, response: Response) -> Result<Self::Output> {
        Ok(response.json().await?)
    }
}

/// Response handler for parsing a payload as Protobuf.
pub struct ProtoResponseHandler<T>(PhantomData<fn() -> T>);

impl<T: Message + Default + Send + Sync> ResponseHandler for ProtoResponseHandler<T> {
    type Output = T;
    const ACCEPT_HEADER: &'static str = "application/protobuf";
    async fn handle_response(self, response: Response) -> Result<Self::Output> {
        let bytes = response.bytes().await?;
        Ok(T::decode(bytes)?)
    }
}

/// Response handler for just returning the raw response.
pub struct RawResponseHandler;

impl ResponseHandler for RawResponseHandler {
    type Output = Response;
    const ACCEPT_HEADER: &'static str = "*/*";
    async fn handle_response(self, response: Response) -> Result<Self::Output> {
        Ok(response)
    }
}

/// Response handler for ignoring the response payload on success.
pub struct NoResponseHandler;

impl ResponseHandler for NoResponseHandler {
    type Output = ();
    const ACCEPT_HEADER: &'static str = "*/*";
    async fn handle_response(self, _response: Response) -> Result<Self::Output> {
        Ok(())
    }
}

/// Generic request builder. Used to construct custom requests towards CDF.
pub struct RequestBuilder<'a, T = ()> {
    inner: crate::reqwest_middleware::RequestBuilder,
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
    pub fn body(mut self, body: impl Into<crate::reqwest::Body>) -> Self {
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
        R: FnOnce(
            crate::reqwest_middleware::RequestBuilder,
        ) -> crate::reqwest_middleware::RequestBuilder,
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
        self.accept(JsonResponseHandler(PhantomData))
    }

    /// Expect the response for a successful request to be `T` encoded as protobuf.
    pub fn accept_protobuf<T: Message + Default + Send + Sync>(
        self,
    ) -> RequestBuilder<'a, ProtoResponseHandler<T>> {
        self.accept(ProtoResponseHandler(PhantomData))
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

impl<'a, T: ResponseHandler> RequestBuilder<'a, T> {
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
