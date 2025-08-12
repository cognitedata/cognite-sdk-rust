use std::{future::Future, sync::LazyLock, time::Duration};

#[cfg(test)]
use cognite::ClientConfig;
use cognite::CogniteClient;
use rand::{distr::Alphanumeric, rng, Rng};
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
pub static PREFIX: LazyLock<String> = LazyLock::new(|| {
    format!(
        "rust-sdk-test-{}",
        rand::rng()
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

#[allow(dead_code)]
pub trait Retry<'a, T, E>: FnMut() -> <Self as Retry<'a, T, E>>::Fut {
    type Fut: Future<Output = Result<T, E>>;
}

impl<T, E, F, Fut> Retry<'_, T, E> for F
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    type Fut = Fut;
}

pub struct Backoff {
    jitter: f64,
    next_backoff: f64,
    max_backoff: f64,
    multiplier: f64,
}

impl Default for Backoff {
    fn default() -> Self {
        Self {
            jitter: 0.5,
            next_backoff: 0.,
            max_backoff: 5.,
            multiplier: 3.,
        }
    }
}

impl Iterator for Backoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        let jitter: f64 = rng().random_range(-self.jitter..self.jitter);
        let mut backoff = (self.next_backoff + jitter).min(self.max_backoff);
        backoff = backoff.max(0.);
        self.next_backoff = (self.next_backoff * self.multiplier).min(self.max_backoff);
        Some(Duration::from_secs_f64(backoff))
    }
}
