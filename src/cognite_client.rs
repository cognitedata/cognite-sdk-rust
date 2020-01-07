use std::env;

use super::{
    ApiClient, ApiKeys, Assets, Error, Events, Files, Groups, Login, Result, ServiceAccounts,
    TimeSeries,
};
use crate::error::Kind;

pub struct CogniteClient {
    pub api_client: ApiClient,

    pub assets: Assets,
    pub events: Events,
    pub files: Files,
    pub time_series: TimeSeries,
    pub service_accounts: ServiceAccounts,
    pub api_keys: ApiKeys,
    pub groups: Groups,
}

static COGNITE_API_KEY: &str = "COGNITE_API_KEY";
static COGNITE_BASE_URL: &str = "COGNITE_BASE_URL";
static COGNITE_PROJECT_NAME: &str = "COGNITE_PROJECT_NAME";

impl CogniteClient {
    pub fn new(app_name: &str) -> Result<Self> {
        let api_key: String = match env::var(COGNITE_API_KEY) {
            Ok(api_key) => {
                println!("Environment variable: API KEY is set");
                api_key
            }
            Err(e) => {
                let error_message = format!(
                    "{} is not defined in the environment. Error: {}",
                    COGNITE_API_KEY, e
                );
                return Err(Error::new(Kind::EnvironmentVariableMissing(error_message)));
            }
        };

        let api_base_url: String = match env::var(COGNITE_BASE_URL) {
            Ok(base_url) => {
                println!("Environment variable: API BASE URL is {}", base_url);
                base_url
            }
            Err(e) => {
                let error_message = format!(
                    "{} is not defined in the environment. Error: {}",
                    COGNITE_BASE_URL, e
                );
                return Err(Error::new(Kind::EnvironmentVariableMissing(error_message)));
            }
        };

        let project_name: String = match env::var(COGNITE_PROJECT_NAME) {
            Ok(base_url) => {
                println!("Environment variable: PROJECT NAME is {}", base_url);
                base_url
            }
            Err(e) => {
                let error_message = format!(
                    "{} is not defined in the environment. Error: {}",
                    COGNITE_PROJECT_NAME, e
                );
                return Err(Error::new(Kind::EnvironmentVariableMissing(error_message)));
            }
        };

        CogniteClient::new_from(&api_key, &api_base_url, &project_name, app_name)
    }

    pub async fn new_with_login_from(
        api_key: &str,
        api_base_url: &str,
        app_name: &str,
    ) -> Result<Self> {
        // Get project name associated to API KEY
        let login_api_client = ApiClient::new(api_base_url, api_key, app_name);
        let login = Login::new(login_api_client);
        let login_status = match login.status().await {
            Ok(status) => status,
            Err(e) => return Err(e),
        };

        let project_name = login_status.project;
        println!("API PROJECT: {}", project_name);

        CogniteClient::new_from(api_key, api_base_url, &project_name, app_name)
    }

    pub fn new_from(
        api_key: &str,
        api_base_url: &str,
        project_name: &str,
        app_name: &str,
    ) -> Result<Self> {
        let api_client = ApiClient::new(api_base_url, api_key, app_name);

        let api_version = "v1".to_string();
        let api_base_path = format!(
            "{}/api/{}/projects/{}",
            api_base_url, api_version, project_name
        );
        let api_keys_api_client = ApiClient::new(&api_base_path, api_key, app_name);
        let assets_api_client = ApiClient::new(&api_base_path, api_key, app_name);
        let events_api_client = ApiClient::new(&api_base_path, api_key, app_name);
        let groups_api_client = ApiClient::new(&api_base_path, api_key, app_name);
        let files_api_client = ApiClient::new(&api_base_path, api_key, app_name);
        let service_accounts_api_client = ApiClient::new(&api_base_path, api_key, app_name);
        let time_series_api_client = ApiClient::new(&api_base_path, api_key, app_name);

        Ok(CogniteClient {
            api_client,

            assets: Assets::new(assets_api_client),
            api_keys: ApiKeys::new(api_keys_api_client),
            events: Events::new(events_api_client),
            files: Files::new(files_api_client),
            groups: Groups::new(groups_api_client),
            service_accounts: ServiceAccounts::new(service_accounts_api_client),
            time_series: TimeSeries::new(time_series_api_client),
        })
    }
}
