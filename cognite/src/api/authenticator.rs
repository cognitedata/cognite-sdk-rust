use async_trait::async_trait;
use futures_locks::RwLock;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

type CustomAuthCallback = dyn Fn(&mut HeaderMap, &ClientWithMiddleware) + Send + Sync;

#[async_trait]
pub trait CustomAuthenticator {
    async fn set_headers(&self, headers: &mut HeaderMap, client: &ClientWithMiddleware);
}

pub enum AuthHeaderManager {
    OIDCToken(Authenticator),
    ApiKey(String),
    FixedToken(String),
    AuthTicket(String),
    Custom(Box<CustomAuthCallback>),
    CustomAsync(Box<dyn CustomAuthenticator + Send + Sync>),
}

impl AuthHeaderManager {
    pub async fn set_headers(
        &self,
        headers: &mut HeaderMap,
        client: &ClientWithMiddleware,
    ) -> Result<(), AuthenticatorError> {
        match self {
            AuthHeaderManager::OIDCToken(a) => {
                let token = a.get_token(client).await?;
                let auth_header_value =
                    HeaderValue::from_str(&format!("Bearer {token}")).map_err(|e| {
                        AuthenticatorError {
                            error: Some(format!("Failed to set authorization bearer token: {e}")),
                            error_description: None,
                            error_uri: None,
                        }
                    })?;
                headers.insert("Authorization", auth_header_value);
            }
            AuthHeaderManager::ApiKey(a) => {
                let api_key_header_value =
                    HeaderValue::from_str(a).map_err(|e| AuthenticatorError {
                        error: Some(format!("Failed to set api key: {e}")),
                        error_description: None,
                        error_uri: None,
                    })?;
                headers.insert("api-key", api_key_header_value);
            }
            AuthHeaderManager::FixedToken(token) => {
                let auth_header_value =
                    HeaderValue::from_str(&format!("Bearer {token}")).map_err(|e| {
                        AuthenticatorError {
                            error: Some(format!("Failed to set authorization bearer token: {e}")),
                            error_description: None,
                            error_uri: None,
                        }
                    })?;
                headers.insert("Authorization", auth_header_value);
            }
            AuthHeaderManager::AuthTicket(t) => {
                let auth_ticket_header_value =
                    HeaderValue::from_str(t).map_err(|e| AuthenticatorError {
                        error: Some(format!("Failed to set auth ticket: {e}")),
                        error_description: None,
                        error_uri: None,
                    })?;
                headers.insert("auth-ticket", auth_ticket_header_value);
            }
            AuthHeaderManager::Custom(c) => c(headers, client),
            AuthHeaderManager::CustomAsync(c) => c.set_headers(headers, client).await,
        }
        Ok(())
    }
}

pub struct AuthenticatorConfig {
    pub client_id: String,
    pub token_url: String,
    pub secret: String,
    pub resource: Option<String>,
    pub audience: Option<String>,
    pub scopes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthenticatorRequest {
    client_id: String,
    client_secret: String,
    resource: Option<String>,
    audience: Option<String>,
    scope: Option<String>,
    grant_type: String,
}

impl AuthenticatorRequest {
    fn new(config: AuthenticatorConfig) -> AuthenticatorRequest {
        AuthenticatorRequest {
            client_id: config.client_id,
            client_secret: config.secret,
            grant_type: "client_credentials".to_string(),
            resource: config.resource,
            audience: config.audience,
            scope: config.scopes,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthenticatorResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Serialize, Deserialize, Debug, Error)]
pub struct AuthenticatorError {
    pub error: Option<String>,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
}

impl Display for AuthenticatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#?}: {:#?}. {:#?}",
            &self.error, &self.error_description, &self.error_uri
        )
    }
}

struct AuthenticatorState {
    pub last_response: Option<AuthenticatorResponse>,
    last_request_time: u64,
}

impl AuthenticatorState {
    pub fn register_response(
        &mut self,
        response: AuthenticatorResponse,
        now: u64,
    ) -> Option<&AuthenticatorResponse> {
        self.last_response = Some(response);
        self.last_request_time = now;
        self.last_response.as_ref()
    }
}

pub struct Authenticator {
    req: AuthenticatorRequest,
    state: RwLock<AuthenticatorState>,
    token_url: String,
}

impl Authenticator {
    pub fn new(config: AuthenticatorConfig) -> Authenticator {
        Authenticator {
            token_url: config.token_url.clone(),
            req: AuthenticatorRequest::new(config),
            state: RwLock::new(AuthenticatorState {
                last_response: None,
                last_request_time: 0,
            }),
        }
    }

    async fn request_token(
        &self,
        client: &ClientWithMiddleware,
    ) -> Result<AuthenticatorResponse, AuthenticatorError> {
        match client.get(&self.token_url).form(&self.req).send().await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<AuthenticatorResponse>().await {
                    Ok(json) => Ok(json),
                    Err(_e) => Err(AuthenticatorError {
                        error: Some("Failed to serialize".to_string()),
                        error_description: None,
                        error_uri: None,
                    }),
                },
                _ => match response.json::<AuthenticatorError>().await {
                    Ok(json) => Err(json),
                    Err(_e) => Err(AuthenticatorError {
                        error: Some(
                            "Something went wrong, but the response error couldn't be serialized"
                                .to_string(),
                        ),
                        error_description: None,
                        error_uri: None,
                    }),
                },
            },
            Err(e) => Err(AuthenticatorError {
                error: Some(format!(
                    "Something went wrong when sending the request: {e}"
                )),
                error_description: None,
                error_uri: None,
            }),
        }
    }

    pub async fn get_token(
        &self,
        client: &ClientWithMiddleware,
    ) -> Result<String, AuthenticatorError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        {
            let state = &*self.state.read().await;
            if let Some(last) = &state.last_response {
                if state.last_request_time + last.expires_in > now + 60 {
                    return Ok(last.access_token.clone());
                }
            }
        }

        // If the token is expired, release the read lock and try to acquire a write lock.
        let mut write = self.state.write().await;

        // Need to check here too, in case we were blocked in this write lock by another thread
        // fetching the token.
        if let Some(last) = &write.last_response {
            if write.last_request_time + last.expires_in > now {
                return Ok(last.access_token.clone());
            }
        }

        match self.request_token(client).await {
            Ok(response) => {
                let response = write.register_response(response, now);
                Ok(response.unwrap().access_token.clone())
            }
            Err(e) => Err(e),
        }
    }
}
