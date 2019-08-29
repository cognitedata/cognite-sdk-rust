use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorWrapper {
    pub error: ApiErrorMessage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorMessage {
    pub code: u32,
    pub message: String,
}

pub struct Error {
    kind: Kind,
}

impl Error {
    pub fn new(kind: Kind) -> Error {
        Error { kind }
    }

    pub fn new_reqwest_error_with_kind(external: ::reqwest::Error, kind: Kind) -> Error {
        let external_kind = ExternalKind::Reqwest(external, Some(Box::new(kind)));
        Error {
            kind: Kind::ExternalLib(external_kind),
        }
    }
}

/// A `Result` alias where the `Err` case is `cognite::Error`.
pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error").field("kind", &self.kind).finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::ExternalLib(e) => match e {
                ExternalKind::Reqwest(exk, _k) => fmt::Display::fmt(exk, f),
                ExternalKind::SerdeJson(exk, _k) => fmt::Display::fmt(exk, f),
            },
            Kind::BadRequest(e) => f.write_str(e),
            Kind::Unauthorized(e) => f.write_str(e),
            Kind::Forbidden(e) => f.write_str(e),
            Kind::NotFound(e) => f.write_str(e),
            Kind::Conflict(e) => f.write_str(e),
            Kind::UnprocessableEntity(e) => f.write_str(e),
            Kind::Http(e) => f.write_str(e),
            Kind::EnvironmentVariableMissing(e) => f.write_str(e),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match &self.kind {
            Kind::ExternalLib(e) => match e {
                ExternalKind::Reqwest(exk, _) => exk.description(),
                ExternalKind::SerdeJson(exk, _) => exk.description(),
            },
            Kind::BadRequest(e) => e,
            Kind::Unauthorized(e) => e,
            Kind::Forbidden(e) => e,
            Kind::NotFound(e) => e,
            Kind::Conflict(e) => e,
            Kind::UnprocessableEntity(e) => e,
            Kind::Http(e) => e,
            Kind::EnvironmentVariableMissing(e) => e,
        }
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

#[derive(Debug)]
pub enum Kind {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    UnprocessableEntity(String),
    Http(String),
    EnvironmentVariableMissing(String),
    ExternalLib(ExternalKind),
}

#[derive(Debug)]
pub enum ExternalKind {
    Reqwest(::reqwest::Error, Option<Box<Kind>>),
    SerdeJson(::serde_json::Error, Option<Box<Kind>>),
}

impl From<::reqwest::Error> for Kind {
    fn from(err: ::reqwest::Error) -> Kind {
        Kind::ExternalLib(ExternalKind::Reqwest(err, None))
    }
}
impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::new(Kind::from(err))
    }
}

impl From<::serde_json::Error> for Kind {
    fn from(err: ::serde_json::Error) -> Kind {
        Kind::ExternalLib(ExternalKind::SerdeJson(err, None))
    }
}
impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::new(Kind::from(err))
    }
}
