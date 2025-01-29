use crate::{AuthenticatorError, FromErrorDetail};
use reqwest::header::InvalidHeaderValue;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Wrapper around an error from CDF.
pub struct ApiErrorWrapper {
    /// Error message from CDF API.
    pub error: ApiErrorMessage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
/// Value that is either an integer or a string.
pub enum IntegerStringOrObject {
    /// 64 bit integer.
    Integer(i64),
    /// String.
    String(String),
    /// Object.
    Object(HashMap<String, Box<IntegerStringOrObject>>),
}

impl IntegerStringOrObject {
    /// Return self as integer, or none.
    pub fn integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }
    /// Return self as string, or none.
    pub fn string(&self) -> Option<&String> {
        match self {
            Self::Integer(_) => None,
            Self::String(s) => Some(s),
            _ => None,
        }
    }
    /// Return self as object, or none.
    pub fn object(&self) -> Option<&HashMap<String, Box<Self>>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Details about API errors.
pub struct ApiErrorDetail(pub Vec<HashMap<String, Box<IntegerStringOrObject>>>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// CDF error message.
pub struct ApiErrorMessage {
    /// HTTP error code.
    pub code: u32,
    /// Description of the error.
    pub message: String,
    /// List of missing items.
    pub missing: Option<ApiErrorDetail>,
    /// List of duplicated items.
    pub duplicated: Option<ApiErrorDetail>,
}

impl ApiErrorDetail {
    /// Get values of type `T` form self.
    pub fn get_values<T: FromErrorDetail>(&self) -> impl Iterator<Item = T> + '_ {
        self.iter().filter_map(|m| T::from_detail(m))
    }

    /// Get a value from `map` as integer.
    pub fn get_integer(
        map: &HashMap<String, Box<IntegerStringOrObject>>,
        key: &str,
    ) -> Option<i64> {
        map.get(key).and_then(|f| f.integer())
    }
    /// Get a value from `map` as string.
    pub fn get_string<'a>(
        map: &'a HashMap<String, Box<IntegerStringOrObject>>,
        key: &str,
    ) -> Option<&'a String> {
        map.get(key).and_then(|f| f.string())
    }

    /// Get an value from `map` as a nested object.
    pub fn get_object<'a>(
        map: &'a HashMap<String, Box<IntegerStringOrObject>>,
        key: &str,
    ) -> Option<&'a HashMap<String, Box<IntegerStringOrObject>>> {
        map.get(key).and_then(|f| f.object())
    }

    /// Iterate over elements.
    pub fn iter(&self) -> impl Iterator<Item = &HashMap<String, Box<IntegerStringOrObject>>> + '_ {
        self.0.iter()
    }
}

#[derive(Debug, Default)]
/// An Error from the CDF API.
pub struct CdfApiError {
    /// HTTP status code.
    pub code: u32,
    /// Error description.
    pub message: String,
    /// List of missing items.
    pub missing: Option<ApiErrorDetail>,
    /// List of duplicated items.
    pub duplicated: Option<ApiErrorDetail>,
    /// Request ID, if available.
    pub request_id: Option<String>,
}

impl fmt::Display for CdfApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}. RequestId: {}",
            &self.code,
            &self.message,
            self.request_id.as_deref().unwrap_or("")
        )
    }
}

impl CdfApiError {
    pub(crate) fn new(raw: ApiErrorMessage, request_id: Option<String>) -> Self {
        CdfApiError {
            code: raw.code,
            message: raw.message,
            missing: raw.missing,
            duplicated: raw.duplicated,
            request_id,
        }
    }
}

impl Error {
    fn new_from_code(code: StatusCode, err: CdfApiError) -> Error {
        match code {
            StatusCode::BAD_REQUEST => Error::BadRequest(err),
            StatusCode::UNAUTHORIZED => Error::Unauthorized(err),
            StatusCode::FORBIDDEN => Error::Forbidden(err),
            StatusCode::NOT_FOUND => Error::NotFound(err),
            StatusCode::CONFLICT => Error::Conflict(err),
            StatusCode::UNPROCESSABLE_ENTITY => Error::UnprocessableEntity(err),
            _ => Error::OtherApiError(err),
        }
    }

    pub(crate) fn new_from_cdf(
        code: StatusCode,
        err: ApiErrorWrapper,
        request_id: Option<String>,
    ) -> Error {
        let cdf_err = CdfApiError::new(err.error, request_id);
        Self::new_from_code(code, cdf_err)
    }

    pub(crate) fn new_without_json(
        code: StatusCode,
        err: String,
        request_id: Option<String>,
    ) -> Error {
        let err = CdfApiError {
            code: code.to_owned().as_u16().into(),
            message: err,
            request_id,
            ..Default::default()
        };
        Self::new_from_code(code, err)
    }
}

/// A `Result` alias where the `Err` case is `cognite::Error`.
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
/// Cognite SDK Error.
pub enum Error {
    #[error("Bad request (400): {0}")]
    /// A bad request error (400)
    BadRequest(CdfApiError),
    #[error("Unauthorized (401): {0}")]
    /// An unauthorized (401) error. When received from CDF this typically means
    /// that the token is not valid for any project in the tenant.
    Unauthorized(CdfApiError),
    #[error("Forbidden (403): {0}")]
    /// A forbidden (403) error. When received from CDF this typically means that
    /// the user lacks access to the requested resource.
    Forbidden(CdfApiError),
    #[error("Not Found (404): {0}")]
    /// A not found (404) error.
    NotFound(CdfApiError),
    #[error("Conflict (409): {0}")]
    /// A conflict (409) error.
    Conflict(CdfApiError),
    #[error("Unprocessable Entity (422): {0}")]
    /// An unprocessable entity (422) error.
    UnprocessableEntity(CdfApiError),
    #[error("Other Api Error: {0}")]
    /// A different CDF error not covered by the common variants in this enum.
    OtherApiError(CdfApiError),
    #[error("Environment Variable Missing: {0}")]
    /// Error returned due to a missing environment variable
    EnvironmentVariableMissing(String),
    #[error("Error from authenticator: {0}")]
    /// Error from the authenticator.
    Authenticator(AuthenticatorError),
    #[error("Invalid header value: {0}")]
    /// Error caused by an invalid header, for example one containing non-ascii symbols.
    InvalidHeader(#[from] InvalidHeaderValue),
    #[error("Error accessing file: {0}")]
    /// Error reading from a file.
    IOError(#[from] std::io::Error),
    #[error("Error collecting stream: {0}")]
    /// Error collecting a stream.
    StreamError(anyhow::Error),
    #[error("Error in middleware: {0}")]
    /// Error in middleware.
    Middleware(anyhow::Error),
    #[error("Error in configuration: {0}")]
    /// Error in configuration.
    Config(String),
    #[error("Unexpected request error: {0}")]
    /// Reqwest error
    Reqwest(#[from] reqwest::Error),
    /// Serde JSON error.
    #[error("Unexpected JSON error: {0}")]
    /// Serde JSON error.
    SerdeJson(#[from] ::serde_json::Error),
    #[error("Unexpected protobuf error: {0}")]
    /// Prost (protobuf deserializer) error
    Prost(#[from] ::prost::DecodeError),
    #[error("{0}")]
    /// Something else went wrong.
    Other(String),
}

impl From<reqwest_middleware::Error> for Error {
    fn from(err: reqwest_middleware::Error) -> Self {
        match err {
            reqwest_middleware::Error::Middleware(x) => Error::Middleware(x),
            reqwest_middleware::Error::Reqwest(x) => Self::from(x),
        }
    }
}
