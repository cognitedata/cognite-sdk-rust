use futures_locks::RwLock;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

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

#[derive(Serialize, Deserialize, Debug)]
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
        client: &Client,
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
                    "Something went wrong when sending the request: {}",
                    e
                )),
                error_description: None,
                error_uri: None,
            }),
        }
    }

    pub async fn get_token(&self, client: &Client) -> Result<String, AuthenticatorError> {
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
