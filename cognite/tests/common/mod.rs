use cognite::CogniteClient;
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, Rng};

#[cfg(test)]
pub fn get_client() -> CogniteClient {
    use cognite::ClientConfig;

    CogniteClient::new_oidc(
        "rust_sdk_test",
        Some(ClientConfig {
            max_retries: 5,
            ..Default::default()
        }),
    )
    .unwrap()
}

#[cfg(test)]
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