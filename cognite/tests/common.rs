#[cfg(test)]
use cognite::ClientConfig;
use cognite::CogniteClient;
use once_cell::sync::Lazy;
use prost_types::Duration;
use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::Semaphore;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_path(base_url: &str, project: &str, endpoint: &str) -> String {
    format!("{}/api/v1/projects/{}/{}", base_url, project, endpoint)
}

#[allow(dead_code)]
pub static CDM_CONCURRENCY_PERMITS: Semaphore = Semaphore::const_new(2);
