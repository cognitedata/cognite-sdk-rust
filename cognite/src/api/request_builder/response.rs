use std::future::Future;
use std::marker::PhantomData;

use prost::Message;
use serde::de::DeserializeOwned;

use reqwest::Response;

use crate::{CondSend, CondSync, Result};

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
    ) -> impl Future<Output = Result<Self::Output>> + CondSend + CondSync;
}

// Uses fn() -> T since that is covariant, and allows us to have a Send future
// without T being Send, technically.
/// Response handler for parsing a payload as JSON.
pub struct JsonResponseHandler<T>(PhantomData<fn() -> T>);

impl<T> Default for JsonResponseHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> JsonResponseHandler<T> {
    /// Create a new JSON response handler.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: DeserializeOwned> ResponseHandler for JsonResponseHandler<T> {
    type Output = T;
    const ACCEPT_HEADER: &'static str = "application/json";
    async fn handle_response(self, response: Response) -> Result<Self::Output> {
        Ok(response.json().await?)
    }
}

/// Response handler for parsing a payload as Protobuf.
pub struct ProtoResponseHandler<T>(PhantomData<fn() -> T>);

impl<T> Default for ProtoResponseHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ProtoResponseHandler<T> {
    /// Create a new protobuf response handler.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Message + Default + CondSend + CondSync> ResponseHandler for ProtoResponseHandler<T> {
    type Output = T;
    const ACCEPT_HEADER: &'static str = "application/protobuf";
    async fn handle_response(self, response: Response) -> Result<Self::Output> {
        let bytes = response.bytes().await?;
        Ok(T::decode(bytes)?)
    }
}

/// Response handler for just returning the raw response.
#[derive(Default)]
pub struct RawResponseHandler;

impl ResponseHandler for RawResponseHandler {
    type Output = Response;
    const ACCEPT_HEADER: &'static str = "*/*";
    async fn handle_response(self, response: Response) -> Result<Self::Output> {
        Ok(response)
    }
}

/// Response handler for ignoring the response payload on success.
#[derive(Default)]
pub struct NoResponseHandler;

impl ResponseHandler for NoResponseHandler {
    type Output = ();
    const ACCEPT_HEADER: &'static str = "*/*";
    async fn handle_response(self, _response: Response) -> Result<Self::Output> {
        Ok(())
    }
}
