use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{to_query, IntoParams, SetCursor};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
/// Session status.
pub enum SessionStatus {
    /// Session is ready to be activated.
    Ready,
    /// Session is active.
    Active,
    /// Session has been cancelled.
    Cancelled,
    /// Session has been revoked.
    Revoked,
    /// Access to the IdP has been lost.
    AccessLost,
}

impl Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ready => write!(f, "ready"),
            Self::Active => write!(f, "active"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Revoked => write!(f, "revoked"),
            Self::AccessLost => write!(f, "access_lost"),
        }
    }
}

#[derive(Clone, Debug, Default)]
/// Query for listing sessions.
pub struct SessionQuery {
    /// Filter by session status.
    pub status: Option<SessionStatus>,
    /// Cursor for pagination.
    pub cursor: Option<String>,
    /// Maximum number of sessions to return, default 25, maximum 1000.
    pub limit: Option<u32>,
}

impl SetCursor for SessionQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl IntoParams for SessionQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = vec![];
        to_query("status", &self.status, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("limit", &self.limit, &mut params);
        params
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// A CDF Session.
pub struct Session {
    /// Session internal ID.
    pub id: i64,
    /// Session type.
    pub r#type: Option<String>,
    /// Session status.
    pub status: SessionStatus,
    /// Time this session was created, in milliseconds since epoch.
    pub creation_time: Option<i64>,
    /// Time when this session will expire, in milliseconds since epoch.
    pub expiration_time: Option<i64>,
    /// Session nonce, used by the CDF service to activate the session.
    pub nonce: Option<String>,
    /// Client ID.
    pub client_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged, rename_all = "camelCase")]
/// Create a new session.
pub enum AddSession {
    /// Create a session using client credentials.
    ClientCredentials {
        /// Client ID.
        client_id: String,
        /// Client secret.
        client_secret: String,
    },
    /// Create a session using token exchange.
    TokenExchange {
        /// Set to `true`.
        token_exchange: bool,
    },
}
