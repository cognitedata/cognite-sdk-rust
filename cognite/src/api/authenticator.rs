use crate::dto::utils::MaybeStringU64;
use async_trait::async_trait;
use futures_locks::RwLock;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::{fmt::Display, sync::Arc};
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
#[derive(Clone)]
pub enum AuthHeaderManager {
    /// Authenticator that makes OIDC requests to obtain tokens.
    OIDCToken(Arc<Authenticator>),
    /// A fixed OIDC token
    FixedToken(String),
    /// An internal auth ticket.
    AuthTicket(String),
    /// A synchronous authentication method.
    Custom(Arc<CustomAuthCallback>),
    /// An async authentication method.
    CustomAsync(Arc<dyn CustomAuthenticator + Send + Sync>),
}

impl AuthHeaderManager {
    /// Set necessary headers in `headers`. This will sometimes request tokens from
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
    /// Optional default token expiry time, in seconds.
    /// If this is set, the authenticator will fall back on this if
    /// the identity provider returns a token response without `expires_in`.
    /// If this is not set, and `expires_in` is missing, the authenticator will return an error.
    pub default_expires_in: Option<u64>,
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
    expires_in: Option<MaybeStringU64>,
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
    last_token: Option<String>,
    current_token_expiry: Instant,
}

/// Result from getting a token, including expiry time.
pub struct AuthenticatorResult {
    /// The token string.
    token: String,
    /// The time when the token will expire.
    expiry: Instant,
}

/// Simple OIDC authenticator.
pub struct Authenticator {
    req: AuthenticatorRequest,
    state: RwLock<AuthenticatorState>,
    token_url: String,
    default_expires_in: Option<Duration>,
}

impl AuthenticatorResult {
    /// Get the token string.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Consume self and get the token string.
    pub fn into_token(self) -> String {
        self.token
    }

    /// Get the expiry time.
    pub fn expiry(&self) -> Instant {
        self.expiry
    }
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
            default_expires_in: config.default_expires_in.map(Duration::from_secs),
            req: AuthenticatorRequest::new(config),
            state: RwLock::new(AuthenticatorState {
                last_token: None,
                current_token_expiry: Instant::now(),
            }),
        }
    }

    async fn request_token(
        &self,
        client: &ClientWithMiddleware,
    ) -> Result<AuthenticatorResult, AuthenticatorError> {
        let response = client
            .post(&self.token_url)
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

        let start = Instant::now();

        let response = response.text().await.map_err(|e| {
            AuthenticatorError::internal_error(
                "Failed to receive response contents".to_owned(),
                Some(e.to_string()),
            )
        })?;

        if status != StatusCode::OK {
            return match serde_json::from_str(&response) {
                Ok(e) => Err(e),
                Err(e) => Err(AuthenticatorError::internal_error(
                    format!("Something went wrong (status: {status}), but the response error couldn't be deserialized. Raw response: {response}")
                    , Some(e.to_string())))
            };
        }

        let response: AuthenticatorResponse = serde_json::from_str(&response).map_err(|e| {
            AuthenticatorError::internal_error(
                "Failed to deserialize response from OAuth endpoint".to_string(),
                Some(e.to_string()),
            )
        })?;

        let token = response.access_token;
        let Some(expires_in) = response
            .expires_in
            // Subtract 60 as a buffer. We do retry on 401s, but it's best to renew the
            // token before it expires. If for whatever reason expires_in is less than 60,
            // we will just always renew before sending a request. We won't (hopefully)
            // get an infinite loop.
            .map(|m| Duration::from_secs(m.0.saturating_sub(60)))
            .or(self.default_expires_in)
        else {
            return Err(AuthenticatorError::internal_error(
                "Missing expires_in in response, and no default expiration configured".to_owned(),
                None,
            ));
        };

        Ok(AuthenticatorResult {
            token,
            expiry: start + expires_in,
        })
    }

    /// Get a token. This will only fetch a new token if it is about
    /// to expire (will expire in the next 60 seconds). This also
    /// returns when the next token will be requested. This is the time
    /// when the authenticator will refresh the token, so the actual
    /// expiry time minus 60 seconds.
    ///
    /// # Arguments
    ///
    /// * `client` - Reqwest client to use for requests to the IdP.
    pub async fn get_token_with_expiry(
        &self,
        client: &ClientWithMiddleware,
    ) -> Result<AuthenticatorResult, AuthenticatorError> {
        let now = Instant::now();
        {
            let state = &*self.state.read().await;
            if let Some(last) = &state.last_token {
                if state.current_token_expiry > now {
                    return Ok(AuthenticatorResult {
                        token: last.clone(),
                        expiry: state.current_token_expiry,
                    });
                }
            }
        }

        // If the token is expired, release the read lock and try to acquire a write lock.
        let mut write = self.state.write().await;

        // Need to check here too, in case we were blocked in this write lock by another thread
        // fetching the token.
        if let Some(last) = &write.last_token {
            if write.current_token_expiry > now {
                return Ok(AuthenticatorResult {
                    token: last.clone(),
                    expiry: write.current_token_expiry,
                });
            }
        }

        match self.request_token(client).await {
            Ok(response) => {
                write.current_token_expiry = response.expiry;
                write.last_token = Some(response.token.clone());
                Ok(response)
            }
            Err(e) => Err(e),
        }
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
        Ok(self.get_token_with_expiry(client).await?.token)
    }
}
