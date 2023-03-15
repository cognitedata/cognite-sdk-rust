use cognite::{ClientConfig, CogniteClient};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, Rng};

pub fn get_client() -> CogniteClient {
    CogniteClient::new_oidc(
        "rust_sdk_test",
        Some(ClientConfig {
            max_retries: 5,
            ..Default::default()
        }),
    )
    .unwrap()
}

pub fn get_client_for_mocking(api_base_url: &str, project_name: &str) -> CogniteClient {
    CogniteClient::new_custom_auth(
        api_base_url,
        project_name,
        cognite::AuthHeaderManager::AuthTicket("my_ticket".to_string()),
        "rust_sdk_test",
        Some(ClientConfig {
            max_retries: 5,
            ..Default::default()
        }),
    )
    .unwrap()
}

pub fn get_path(base_url: &str, project: &str, endpoint: &str) -> String {
    format!("{}/api/v1/projects/{}/{}", base_url, project, endpoint)
}

pub static PREFIX: Lazy<String> = Lazy::new(|| {
    format!(
        "rust-sdk-test-{}",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>()
    )
});
