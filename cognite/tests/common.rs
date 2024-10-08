use std::{future::Future, time::Duration};

#[cfg(test)]
use cognite::ClientConfig;
use cognite::CogniteClient;
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
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

#[allow(dead_code)]
pub trait Retry<'a, T, E>: FnMut() -> <Self as Retry<'a, T, E>>::Fut {
    type Fut: Future<Output = Result<T, E>>;
}

impl<'a, T, E, F, Fut> Retry<'a, T, E> for F
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    type Fut = Fut;
}

pub struct Backoff {
    pub jitter: f64,
    pub initial_backoff: f64,
    next_backoff: f64,
    max_backoff: f64,
    multiplier: f64,
}

impl Default for Backoff {
    fn default() -> Self {
        Self {
            jitter: 0.5,
            initial_backoff: 0.,
            next_backoff: Default::default(),
            max_backoff: 2.,
            multiplier: 2.,
        }
    }
}

impl Iterator for Backoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        let jitter: f64 = thread_rng().gen_range(-self.jitter..self.jitter);
        let mut backoff = (self.next_backoff + jitter).min(self.max_backoff);
        backoff = backoff.max(0.);
        self.next_backoff = (self.next_backoff * self.multiplier).min(self.max_backoff);
        Some(Duration::from_secs_f64(backoff))
    }
}

// #[allow(dead_code)]
// pub async fn retry_backoff<T, E>(mut callback: impl for<'a> Retry<'a, T, E>) -> Option<T> {
//     let mut jitter: f64 = 0.5;
//     let mut next_backoff: f64 = 0.;
//     let max_backoff: f64 = 2.;
//     let multiplier: f64 = 2.;
//
//     for _ in 0..10 {
//         match callback().await {
//             Ok(res) => return Some(res),
//             Err(_) => {
//                 jitter = thread_rng().gen_range(-jitter..jitter);
//                 let mut backoff = (next_backoff + jitter).min(max_backoff);
//                 backoff = backoff.max(0.0);
//                 tokio::time::sleep(Duration::from_secs_f64(backoff)).await;
//                 next_backoff = (next_backoff * multiplier).min(max_backoff);
//                 continue;
//             }
//         }
//     }
//     None
// }
