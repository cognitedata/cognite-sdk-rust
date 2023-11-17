use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::env;
use std::sync::Arc;
use std::time::Duration;

use super::{ApiClient, Error, Result};
use crate::api::core::sequences::SequencesResource;
use crate::api::data_modeling::Models;
use crate::api::iam::groups::GroupsResource;
use crate::api::iam::sessions::SessionsResource;
use crate::auth::AuthenticatorMiddleware;
use crate::error::Kind;
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
                return Err(Error::new(Kind::EnvironmentVariableMissing(error_message)));
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

#[derive(Default, Clone)]
pub struct ClientConfig {
    pub max_retries: u32,
    pub max_retry_delay_ms: Option<u64>,
    pub timeout_ms: Option<u64>,
}

pub struct CogniteClient {
    pub api_client: Arc<ApiClient>,

    pub assets: AssetsResource,
    pub events: EventsResource,
    pub files: Files,
    pub time_series: TimeSeriesResource,
    pub groups: GroupsResource,
    pub raw: RawResource,
    pub data_sets: DataSetsResource,
    pub labels: LabelsResource,
    pub relationships: RelationshipsResource,
    pub ext_pipes: ExtPipesResource,
    pub ext_pipe_runs: ExtPipeRunsResource,
    pub sequences: SequencesResource,
    pub sessions: SessionsResource,
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
        };

        CogniteClient::new_from_oidc(&api_base_url, auth_config, &project_name, app_name, config)
    }

    pub fn new_custom_auth(
        api_base_url: &str,
        project_name: &str,
        auth: AuthHeaderManager,
        app_name: &str,
        config: Option<ClientConfig>,
    ) -> Result<Self> {
        let api_base_path = format!("{}/api/{}/projects/{}", api_base_url, "v1", project_name);
        let api_client = ApiClient::new(
            &api_base_path,
            app_name,
            Self::get_client(config.unwrap_or_default(), auth, None)?,
        );

        Self::new_internal(api_client)
    }

    fn get_client(
        config: ClientConfig,
        authenticator: AuthHeaderManager,
        client: Option<Client>,
    ) -> Result<ClientWithMiddleware> {
        let client = if let Some(client) = client {
            client
        } else {
            let mut builder = Client::builder();
            // We can add more here later
            if let Some(timeout) = config.timeout_ms {
                builder = builder.timeout(Duration::from_millis(timeout));
            }

            builder.build()?
        };

        let mut builder = ClientBuilder::new(client);
        if config.max_retries > 0 {
            builder = builder.with(CustomRetryMiddleware::new(
                config.max_retries,
                config.max_retry_delay_ms.unwrap_or(5 * 60 * 1000),
            ));
        }
        builder = builder.with(AuthenticatorMiddleware::new(authenticator)?);
        Ok(builder.build())
    }

    fn new_from_builder(
        auth: AuthHeaderManager,
        config: ClientConfig,
        client: Option<Client>,
        app_name: String,
        project: String,
        base_url: String,
    ) -> Result<Self> {
        let api_base_path = format!("{}/api/{}/projects/{}", base_url, "v1", project);
        let api_client = ApiClient::new(
            &api_base_path,
            &app_name,
            Self::get_client(config, auth, client)?,
        );
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
        let api_client = ApiClient::new(
            &api_base_path,
            app_name,
            Self::get_client(config.unwrap_or_default(), auth, None)?,
        );

        Self::new_internal(api_client)
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Default)]
pub struct Builder {
    auth: Option<AuthHeaderManager>,
    config: Option<ClientConfig>,
    client: Option<Client>,
    app_name: Option<String>,
    project: Option<String>,
    base_url: Option<String>,
}

impl Builder {
    pub fn set_custom_auth(&mut self, auth: AuthHeaderManager) -> &mut Self {
        self.auth = Some(auth);
        self
    }

    pub fn set_oidc_credentials(&mut self, auth: AuthenticatorConfig) -> &mut Self {
        self.auth = Some(AuthHeaderManager::OIDCToken(Authenticator::new(auth)));
        self
    }

    pub fn set_project(&mut self, project: &str) -> &mut Self {
        self.project = Some(project.to_owned());
        self
    }

    pub fn set_app_name(&mut self, app_name: &str) -> &mut Self {
        self.app_name = Some(app_name.to_owned());
        self
    }

    pub fn set_internal_client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    pub fn set_client_config(&mut self, config: ClientConfig) -> &mut Self {
        self.config = Some(config);
        self
    }

    pub fn set_base_url(&mut self, base_url: &str) -> &mut Self {
        self.base_url = Some(base_url.to_owned());
        self
    }

    pub fn build(self) -> Result<CogniteClient> {
        let auth = self
            .auth
            .ok_or_else(|| Error::new(Kind::Config("Some form of auth is required".to_string())))?;
        let config = self.config.unwrap_or_default();
        let client = self.client;
        let app_name = self
            .app_name
            .ok_or_else(|| Error::new(Kind::Config("App name is required".to_string())))?;
        let project = self
            .project
            .ok_or_else(|| Error::new(Kind::Config("Project is required".to_string())))?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.cognitedata.com/".to_owned());

        CogniteClient::new_from_builder(auth, config, client, app_name, project, base_url)
    }
}
