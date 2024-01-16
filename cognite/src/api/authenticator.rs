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

/// Type of closure for a synchronous auth callback.
type CustomAuthCallback =
    dyn Fn(&mut HeaderMap, &ClientWithMiddleware) -> Result<(), AuthenticatorError> + Send + Sync;

#[async_trait]
/// Trait for a custom authenticator. This should set the necessary headers in `headers` before each
/// request. Note that this may be called from multiple places in parallel.
pub trait CustomAuthenticator {
    /// Set the required headers for authentication. This may use the provided
    /// `client` to perform a request, if necessary. This will be called frequently, so
    /// make sure it only makes external requests when needed.
    ///
    /// # Arguments
    ///
    /// * `headers` - Header map to modify.
    /// * `client` - Client used to perform any external authentication requests.
    async fn set_headers(
        &self,
        headers: &mut HeaderMap,
        client: &ClientWithMiddleware,
    ) -> Result<(), AuthenticatorError>;
}

/// Enumeration of the possible authentication methods available.
pub enum AuthHeaderManager {
    /// Authenticator that makes OIDC requests to obtain tokens.
    OIDCToken(Authenticator),
    /// A fixed OIDC token
    FixedToken(String),
    /// An internal auth ticket.
    AuthTicket(String),
    /// A synchronous authentication method.
    Custom(Box<CustomAuthCallback>),
    /// An async authentication method.
    CustomAsync(Box<dyn CustomAuthenticator + Send + Sync>),
}

impl AuthHeaderManager {
    /// Set necesary headers in `headers`. This will sometimes request tokens from
    /// the identity provider.
    ///
    /// # Arguments
    ///
    /// * `headers` - Request header collection.
    /// * `client` - Reqwest client used to send authentication requests, if necessary.
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
                        AuthenticatorError::internal_error(
                            "Failed to set authorization bearer token".to_string(),
                            Some(e.to_string()),
                        )
                    })?;
                headers.insert("Authorization", auth_header_value);
            }
            AuthHeaderManager::FixedToken(token) => {
                let auth_header_value =
                    HeaderValue::from_str(&format!("Bearer {token}")).map_err(|e| {
                        AuthenticatorError::internal_error(
                            "Failed to set authorization bearer token".to_string(),
                            Some(e.to_string()),
                        )
                    })?;
                headers.insert("Authorization", auth_header_value);
            }
            AuthHeaderManager::AuthTicket(t) => {
                let auth_ticket_header_value = HeaderValue::from_str(t).map_err(|e| {
                    AuthenticatorError::internal_error(
                        "Failed to set auth ticket".to_string(),
                        Some(e.to_string()),
                    )
                })?;
                headers.insert("auth-ticket", auth_ticket_header_value);
            }
            AuthHeaderManager::Custom(c) => c(headers, client)?,
            AuthHeaderManager::CustomAsync(c) => c.set_headers(headers, client).await?,
        }
        Ok(())
    }
}

/// Configuration for authentication using the OIDC authenticator
pub struct AuthenticatorConfig {
    /// Service principal client ID.
    pub client_id: String,
    /// IdP token URL.
    pub token_url: String,
    /// Service principal client secret.
    pub secret: String,
    /// Optional resource.
    pub resource: Option<String>,
    /// Optional audience.
    pub audience: Option<String>,
    /// Optional space separate list of scopes.
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
/// Error from an authenticator request.
pub struct AuthenticatorError {
    /// Error message
    pub error: String,
    /// Detailed error description.
    pub error_description: Option<String>,
    /// Error URI.
    pub error_uri: Option<String>,
}

impl AuthenticatorError {
    /// Create an authenticator error from message and description.
    ///
    /// # Arguments
    ///
    /// * `error` - Short error message
    /// * `error_description` - Detailed error description.
    pub fn internal_error(error: String, error_description: Option<String>) -> Self {
        Self {
            error,
            error_description,
            error_uri: None,
        }
    }
}

impl Display for AuthenticatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error,)?;
        if let Some(error_description) = &self.error_description {
            write!(f, ": {error_description}")?;
        }
        if let Some(error_uri) = &self.error_uri {
            write!(f, " ({error_uri})")?;
        }
        Ok(())
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

/// Simple OIDC authenticator.
pub struct Authenticator {
    req: AuthenticatorRequest,
    state: RwLock<AuthenticatorState>,
    token_url: String,
}

impl Authenticator {
    /// Create a new authenticator with given config.
    ///
    /// # Arguments
    ///
    /// * `config` - Authenticator configuration.
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
        let response = client
            .get(&self.token_url)
            .form(&self.req)
            .send()
            .await
            .map_err(|e| {
                AuthenticatorError::internal_error(
                    "Something went wrong when sending the request".to_string(),
                    Some(e.to_string()),
                )
            })?;

        let status = response.status();

        if status != StatusCode::OK {
            return Err(
                response.json::<AuthenticatorError>().await.map_err(|e| {
                    AuthenticatorError::internal_error(
                         format!("Something went wrong (status: {status}), but the response error couldn't be deserialized"),
                        Some(e.to_string()),
                    )
                })?,
            );
        }

        response.json::<AuthenticatorResponse>().await.map_err(|e| {
            AuthenticatorError::internal_error(
                "Failed to deserialize".to_string(),
                Some(e.to_string()),
            )
        })
    }

    /// Get a token. This will only fetch a new token if it is about
    /// to expire (will expire in the next 60 seconds).
    ///
    /// # Arguments
    ///
    /// * `client` - Reqwest client to use for requests to the IdP.
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
