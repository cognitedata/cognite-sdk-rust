use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware};
use std::env;
use std::sync::Arc;

use super::{ApiClient, Error, Result};
use crate::api::core::sequences::SequencesResource;
use crate::api::data_modeling::Models;
use crate::api::iam::groups::GroupsResource;
use crate::api::iam::sessions::SessionsResource;
use crate::auth::AuthenticatorMiddleware;
use crate::retry::CustomRetryMiddleware;
use crate::AuthHeaderManager;
use crate::{
    assets::AssetsResource, datasets::DataSetsResource, events::EventsResource,
    extpipes::ExtPipeRunsResource, extpipes::ExtPipesResource, files::Files,
    labels::LabelsResource, raw::RawResource, relationships::RelationshipsResource,
    time_series::TimeSeriesResource,
};

use crate::api::authenticator::{Authenticator, AuthenticatorConfig};

macro_rules! env_or_error {
    ($e: expr) => {
        match env::var($e) {
            Ok(el) => el,
            Err(err) => {
                let error_message =
                    format!("{} is not defined in the environment. Error: {}", $e, err);
                return Err(Error::EnvironmentVariableMissing(error_message));
            }
        }
    };
}

macro_rules! env_or {
    ($e: expr, $d: expr) => {
        match env::var($e) {
            Ok(el) => el,
            Err(_) => $d,
        }
    };
}

macro_rules! env_or_none {
    ($e: expr) => {
        match env::var($e) {
            Ok(el) => Some(el),
            Err(_) => None,
        }
    };
}

#[derive(Default, Clone, Debug)]
/// Configuration object for a cognite client.
pub struct ClientConfig {
    /// Maximum number of retries per request.
    pub max_retries: u32,
    /// Maximum delay between retries.
    pub max_retry_delay_ms: Option<u64>,
    /// Request timeout in milliseconds.
    pub timeout_ms: Option<u64>,
    /// Initial delay for exponential backoff, defaults to 125 milliseconds.
    pub initial_delay_ms: Option<u64>,
}

/// Client object for the CDF API.
pub struct CogniteClient {
    /// Reference to an API client, which can let you make
    /// your own requests to the CDF API.
    pub api_client: Arc<ApiClient>,

    /// CDF assets resource.
    pub assets: AssetsResource,
    /// CDF events resource.
    pub events: EventsResource,
    /// CDF files resource.
    pub files: Files,
    /// CDF time series resource.
    pub time_series: TimeSeriesResource,
    /// CDF groups resource.
    pub groups: GroupsResource,
    /// CDF raw resource.
    pub raw: RawResource,
    /// CDF data sets resource.
    pub data_sets: DataSetsResource,
    /// CDF labels resource.
    pub labels: LabelsResource,
    /// CDF relationships resource.
    pub relationships: RelationshipsResource,
    /// CDF extraction pipelines resource.
    pub ext_pipes: ExtPipesResource,
    /// CDF extraction pipeline runs resource.
    pub ext_pipe_runs: ExtPipeRunsResource,
    /// CDF sequences resource.
    pub sequences: SequencesResource,
    /// CDF sessions resource.
    pub sessions: SessionsResource,
    /// CDF data modeling resource.
    pub models: Models,
}

static COGNITE_BASE_URL: &str = "COGNITE_BASE_URL";
static COGNITE_PROJECT_NAME: &str = "COGNITE_PROJECT";
static COGNITE_CLIENT_ID: &str = "COGNITE_CLIENT_ID";
static COGNITE_CLIENT_SECRET: &str = "COGNITE_CLIENT_SECRET";
static COGNITE_TOKEN_URL: &str = "COGNITE_TOKEN_URL";
static COGNITE_RESOURCE: &str = "COGNITE_RESOURCE";
static COGNITE_AUDIENCE: &str = "COGNITE_AUDIENCE";
static COGNITE_SCOPES: &str = "COGNITE_SCOPES";

impl CogniteClient {
    /// Create a new cogntite client, taking OIDC credentials from the environment.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The value used for the `x-cdp-app` header.
    /// * `config` - Optional configuration for retries.
    ///
    /// This uses the environment variables
    ///
    /// * `COGNITE_BASE_URL`
    /// * `COGNITE_PROJECT`
    /// * `COGNITE_CLIENT_ID`
    /// * `COGNITE_CLIENT_SECRET`
    /// * `COGNITE_TOKEN_URL`
    /// * `COGNITE_RESOURCE`
    /// * `COGNITE_AUDIENCE`
    /// * `COGNITE_SCOPES`
    pub fn new_oidc(app_name: &str, config: Option<ClientConfig>) -> Result<Self> {
        let api_base_url = env_or!(COGNITE_BASE_URL, "https://api.cognitedata.com/".to_string());
        let project_name = env_or_error!(COGNITE_PROJECT_NAME);
        let auth_config = AuthenticatorConfig {
            client_id: env_or_error!(COGNITE_CLIENT_ID),
            token_url: env_or_error!(COGNITE_TOKEN_URL),
            secret: env_or_error!(COGNITE_CLIENT_SECRET),
            resource: env_or_none!(COGNITE_RESOURCE),
            audience: env_or_none!(COGNITE_AUDIENCE),
            scopes: env_or_none!(COGNITE_SCOPES),
            default_expires_in: None,
        };

        CogniteClient::new_from_oidc(&api_base_url, auth_config, &project_name, app_name, config)
    }

    /// Create a new cognite client, using a user-provided authentication manager.
    ///
    /// # Arguments
    ///
    /// * `api_base_url` - Base URL for the API. For example `https://api.cognitedata.com`
    /// * `project_name` - Name of the CDF project to use.
    /// * `auth` - Authentication provider.
    /// * `app_name` - Value used for the `x-cdp-app` header.
    /// * `config` - Optional configuration for retries.
    pub fn new_custom_auth(
        api_base_url: &str,
        project_name: &str,
        auth: AuthHeaderManager,
        app_name: &str,
        config: Option<ClientConfig>,
    ) -> Result<Self> {
        let api_base_path = format!("{}/api/{}/projects/{}", api_base_url, "v1", project_name);
        let client = Self::get_client(config.unwrap_or_default(), auth, None, None)?;
        let api_client = ApiClient::new(&api_base_path, app_name, client.clone());

        Self::new_internal(api_client)
    }

    fn get_client(
        config: ClientConfig,
        authenticator: AuthHeaderManager,
        client: Option<Client>,
        middleware: Option<Vec<Arc<dyn Middleware>>>,
    ) -> Result<ClientWithMiddleware> {
        let client = if let Some(client) = client {
            client
        } else {
            #[allow(unused_mut)]
            let mut builder = Client::builder();
            // We can add more here later
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(timeout) = config.timeout_ms {
                builder = builder.timeout(std::time::Duration::from_millis(timeout));
            }

            builder.build()?
        };

        let mut builder = ClientBuilder::new(client);
        if config.max_retries > 0 {
            builder = builder.with(CustomRetryMiddleware::new(
                config.max_retries,
                config.max_retry_delay_ms.unwrap_or(5 * 60 * 1000),
                config.initial_delay_ms.unwrap_or(125),
            ));
        }
        builder = builder.with(AuthenticatorMiddleware::new(authenticator)?);
        if let Some(mw) = middleware {
            for ware in mw {
                builder = builder.with_arc(ware);
            }
        }
        Ok(builder.build())
    }

    fn new_from_builder(
        auth: AuthHeaderManager,
        config: ClientConfig,
        client: Option<Client>,
        app_name: String,
        project: String,
        base_url: String,
        middleware: Option<Vec<Arc<dyn Middleware>>>,
    ) -> Result<Self> {
        let api_base_path = format!("{}/api/{}/projects/{}", base_url, "v1", project);
        let client = Self::get_client(config, auth, client, middleware)?;
        let api_client = ApiClient::new(&api_base_path, &app_name, client.clone());
        Self::new_internal(api_client)
    }

    fn new_internal(api_client: ApiClient) -> Result<Self> {
        let ac = Arc::new(api_client);
        Ok(CogniteClient {
            api_client: ac.clone(),

            assets: AssetsResource::new(ac.clone()),
            events: EventsResource::new(ac.clone()),
            files: Files::new(ac.clone()),
            groups: GroupsResource::new(ac.clone()),
            time_series: TimeSeriesResource::new(ac.clone()),
            raw: RawResource::new(ac.clone()),
            data_sets: DataSetsResource::new(ac.clone()),
            labels: LabelsResource::new(ac.clone()),
            relationships: RelationshipsResource::new(ac.clone()),
            ext_pipes: ExtPipesResource::new(ac.clone()),
            ext_pipe_runs: ExtPipeRunsResource::new(ac.clone()),
            sequences: SequencesResource::new(ac.clone()),
            sessions: SessionsResource::new(ac.clone()),
            models: Models::new(ac),
        })
    }

    /// Create a new cognite client using provided OIDC credentials.
    ///
    /// # Arguments
    ///
    /// * `api_base_url` - Base URL for the API. For example `https://api.cognitedata.com`
    /// * `project_name` - Name of the CDF project to use.
    /// * `auth_config` - Configuration for creating an OIDC authenticator.
    /// * `app_name` - Value used for the `x-cdp-app` header.
    /// * `config` - Optional configuration for retries.
    pub fn new_from_oidc(
        api_base_url: &str,
        auth_config: AuthenticatorConfig,
        project_name: &str,
        app_name: &str,
        config: Option<ClientConfig>,
    ) -> Result<Self> {
        let authenticator = Authenticator::new(auth_config);
        let api_base_path = format!("{}/api/{}/projects/{}", api_base_url, "v1", project_name);
        let auth = AuthHeaderManager::OIDCToken(authenticator);
        let client = Self::get_client(config.unwrap_or_default(), auth, None, None)?;
        let api_client = ApiClient::new(&api_base_path, app_name, client.clone());

        Self::new_internal(api_client)
    }

    /// Create a builder with a fluent API for creating a cognite client.
    pub fn builder() -> Builder {
        Builder::default()
    }
}

/// Fluent API for configuring a client.
#[derive(Default)]
pub struct Builder {
    auth: Option<AuthHeaderManager>,
    config: Option<ClientConfig>,
    client: Option<Client>,
    app_name: Option<String>,
    project: Option<String>,
    base_url: Option<String>,
    custom_middleware: Option<Vec<Arc<dyn Middleware>>>,
}

impl Builder {
    /// Set a custom authenticator.
    ///
    /// # Arguments
    ///
    /// * `auth` - Authenticator to use.
    pub fn set_custom_auth(&mut self, auth: AuthHeaderManager) -> &mut Self {
        self.auth = Some(auth);
        self
    }

    /// Set an authenticator using OIDC client credentials.
    ///
    /// # Arguments
    ///
    /// * `auth` - Client credentials.
    pub fn set_oidc_credentials(&mut self, auth: AuthenticatorConfig) -> &mut Self {
        self.auth = Some(AuthHeaderManager::OIDCToken(Authenticator::new(auth)));
        self
    }

    /// Set the CDF project to connect to.
    ///
    /// # Arguments
    ///
    /// * `project` - CDF project
    pub fn set_project(&mut self, project: &str) -> &mut Self {
        self.project = Some(project.to_owned());
        self
    }

    /// Set the value of the `x-cdp-app` header.
    pub fn set_app_name(&mut self, app_name: &str) -> &mut Self {
        self.app_name = Some(app_name.to_owned());
        self
    }

    /// Set the reqwest client used internally. If your application
    /// connects to a large number of different CDF projects, or uses a large
    /// number of different sets of credentials. It is recommended to share
    /// a single reqwest client.
    ///
    /// # Arguments
    ///
    /// * `client` - reqwest client to use.
    pub fn set_internal_client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    /// Set configuration for retries.
    ///
    /// # Arguments
    ///
    /// * `config` - Client configuration.
    pub fn set_client_config(&mut self, config: ClientConfig) -> &mut Self {
        self.config = Some(config);
        self
    }

    /// Set the base URL used by the client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Cognite API base URL, for example `https://api.cognitedata.com`
    pub fn set_base_url(&mut self, base_url: &str) -> &mut Self {
        self.base_url = Some(base_url.to_owned());
        self
    }

    /// Add some custom middleware.
    ///
    /// # Arguments
    ///
    /// * `middleware` - A reference to some reqwest middleware.
    pub fn with_custom_middleware(&mut self, middleware: Arc<dyn Middleware>) -> &mut Self {
        match &mut self.custom_middleware {
            Some(x) => x.push(middleware),
            None => self.custom_middleware = Some(vec![middleware]),
        }
        self
    }

    /// Create a cognite client. This may fail if not all required parameters are provided.
    pub fn build(self) -> Result<CogniteClient> {
        let auth = self
            .auth
            .ok_or_else(|| Error::Config("Some form of auth is required".to_string()))?;
        let config = self.config.unwrap_or_default();
        let client = self.client;
        let app_name = self
            .app_name
            .ok_or_else(|| Error::Config("App name is required".to_string()))?;
        let project = self
            .project
            .ok_or_else(|| Error::Config("Project is required".to_string()))?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.cognitedata.com/".to_owned());

        CogniteClient::new_from_builder(
            auth,
            config,
            client,
            app_name,
            project,
            base_url,
            self.custom_middleware,
        )
    }
}
