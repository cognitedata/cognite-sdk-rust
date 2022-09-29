use reqwest::Client;
use std::env;
use std::sync::Arc;

use super::{ApiClient, Error, Result};
use crate::api::core::sequences::Sequences;
use crate::error::Kind;
use crate::{
    assets::Assets, datasets::DataSets, events::Events, extpipes::ExtPipeRuns, extpipes::ExtPipes,
    files::Files, iam::ApiKeys, iam::Groups, iam::ServiceAccounts, labels::Labels, login::Login,
    raw::Raw, relationships::Relationships, time_series::TimeSeries,
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

pub struct CogniteClient {
    pub api_client: Arc<ApiClient>,

    pub assets: Assets,
    pub events: Events,
    pub files: Files,
    pub time_series: TimeSeries,
    pub service_accounts: ServiceAccounts,
    pub api_keys: ApiKeys,
    pub groups: Groups,
    pub raw: Raw,
    pub data_sets: DataSets,
    pub labels: Labels,
    pub relationships: Relationships,
    pub ext_pipes: ExtPipes,
    pub ext_pipe_runs: ExtPipeRuns,
    pub sequences: Sequences,
}

static COGNITE_API_KEY: &str = "COGNITE_API_KEY";
static COGNITE_BASE_URL: &str = "COGNITE_BASE_URL";
static COGNITE_PROJECT_NAME: &str = "COGNITE_PROJECT";
static COGNITE_CLIENT_ID: &str = "COGNITE_CLIENT_ID";
static COGNITE_CLIENT_SECRET: &str = "COGNITE_CLIENT_SECRET";
static COGNITE_TOKEN_URL: &str = "COGNITE_TOKEN_URL";
static COGNITE_RESOURCE: &str = "COGNITE_RESOURCE";
static COGNITE_AUDIENCE: &str = "COGNITE_AUDIENCE";
static COGNITE_SCOPES: &str = "COGNITE_SCOPES";

impl CogniteClient {
    pub fn new(app_name: &str) -> Result<Self> {
        let api_key = env_or_error!(COGNITE_API_KEY);
        let api_base_url = env_or!(COGNITE_BASE_URL, "https://api.cognitedata.com/".to_string());
        let project_name = env_or_error!(COGNITE_PROJECT_NAME);

        CogniteClient::new_from(&api_key, &api_base_url, &project_name, app_name)
    }

    pub fn new_oidc(app_name: &str) -> Result<Self> {
        let api_base_url = env_or!(COGNITE_BASE_URL, "https://api.cognitedata.com/".to_string());
        let project_name = env_or_error!(COGNITE_PROJECT_NAME);
        let config = AuthenticatorConfig {
            client_id: env_or_error!(COGNITE_CLIENT_ID),
            token_url: env_or_error!(COGNITE_TOKEN_URL),
            secret: env_or_error!(COGNITE_CLIENT_SECRET),
            resource: env_or_none!(COGNITE_RESOURCE),
            audience: env_or_none!(COGNITE_AUDIENCE),
            scopes: env_or_none!(COGNITE_SCOPES),
        };

        CogniteClient::new_from_oidc(&api_base_url, config, &project_name, app_name)
    }

    fn new_internal(api_client: ApiClient) -> Result<Self> {
        let ac = Arc::new(api_client);
        Ok(CogniteClient {
            api_client: ac.clone(),

            assets: Assets::new(ac.clone()),
            api_keys: ApiKeys::new(ac.clone()),
            events: Events::new(ac.clone()),
            files: Files::new(ac.clone()),
            groups: Groups::new(ac.clone()),
            service_accounts: ServiceAccounts::new(ac.clone()),
            time_series: TimeSeries::new(ac.clone()),
            raw: Raw::new(ac.clone()),
            data_sets: DataSets::new(ac.clone()),
            labels: Labels::new(ac.clone()),
            relationships: Relationships::new(ac.clone()),
            ext_pipes: ExtPipes::new(ac.clone()),
            ext_pipe_runs: ExtPipeRuns::new(ac.clone()),
            sequences: Sequences::new(ac),
        })
    }

    pub async fn new_with_login_from(
        api_key: &str,
        api_base_url: &str,
        app_name: &str,
    ) -> Result<Self> {
        // Get project name associated to API KEY
        let login_api_client = ApiClient::new(api_base_url, api_key, app_name, Client::new());
        let login = Login::new(Arc::new(login_api_client));
        let login_status = match login.status().await {
            Ok(status) => status,
            Err(e) => return Err(e),
        };

        let project_name = login_status.project;

        CogniteClient::new_from(api_key, api_base_url, &project_name, app_name)
    }

    pub fn new_from_oidc(
        api_base_url: &str,
        config: AuthenticatorConfig,
        project_name: &str,
        app_name: &str,
    ) -> Result<Self> {
        let client = Client::new();
        let authenticator = Authenticator::new(config);
        let api_base_path = format!("{}/api/{}/projects/{}", api_base_url, "v1", project_name);
        let api_client = ApiClient::new_oidc(&api_base_path, authenticator, app_name, client);

        Self::new_internal(api_client)
    }

    pub fn new_from(
        api_key: &str,
        api_base_url: &str,
        project_name: &str,
        app_name: &str,
    ) -> Result<Self> {
        let client = Client::new();
        let api_base_path = format!("{}/api/{}/projects/{}", api_base_url, "v1", project_name);
        let api_client = ApiClient::new(&api_base_path, api_key, app_name, client);

        Self::new_internal(api_client)
    }
}
