use cognite::models::*;
use cognite::*;
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, Rng};

#[cfg(test)]

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

pub fn get_path(base_url: &str, project: &str, endpoint: &str) -> String {
    format!("{}/api/v1/projects/{}/{}", base_url, project, endpoint)
}

pub async fn create_space(client: &CogniteClient, space_id: &str) -> Vec<Space> {
    let new_space = SpaceCreate {
        space: space_id.to_string(),
        description: Some("Some description".to_owned()),
        name: Some("Test space".to_owned()),
    };
    client.models.spaces.create(&[new_space]).await.unwrap()
}

pub async fn delete_space(client: &CogniteClient, space_id: &str) -> Vec<SpaceId> {
    client
        .models
        .spaces
        .delete(&[SpaceId {
            space: space_id.to_string(),
        }])
        .await
        .unwrap()
        .items
}
