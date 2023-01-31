use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{to_query, AsParams, SetCursor};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Ready,
    Active,
    Cancelled,
    Revoked,
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

pub struct SessionQuery {
    pub status: Option<SessionStatus>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

impl SetCursor for SessionQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl AsParams for SessionQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = vec![];
        to_query("status", &self.status, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("limit", &self.limit, &mut params);
        params
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: i64,
    pub r#type: Option<String>,
    pub status: SessionStatus,
    pub creation_time: Option<i64>,
    pub expiration_time: Option<i64>,
    pub nonce: Option<String>,
    pub client_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum AddSession {
    ClientCredentials {
        client_id: String,
        client_secret: String,
    },
    TokenExchange {
        token_exchange: bool,
    },
}
