// This file is adapted from reqwest-retry, which was a bit too opinionated for our use.
// https://github.com/TrueLayer/reqwest-middleware

use async_trait::async_trait;
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Middleware, Next, Result};
use std::time::Duration;
use task_local_extensions::Extensions;

pub struct CustomRetryMiddleware {
    max_retries: u32,
    max_delay_ms: u64,
}

#[async_trait]
impl Middleware for CustomRetryMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        self.execute_with_retry(req, next, extensions).await
    }
}

impl CustomRetryMiddleware {
    pub fn new(max_retries: u32, max_delay_ms: u64) -> Self {
        Self {
            max_retries: max_retries.min(10),
            max_delay_ms,
        }
    }

    async fn execute_with_retry<'a>(
        &'a self,
        req: Request,
        next: Next<'a>,
        ext: &'a mut Extensions,
    ) -> Result<Response> {
        let mut n_past_retries = 0;
        loop {
            let duplicate_request = match req.try_clone() {
                Some(x) => x,
                None => return next.run(req, ext).await,
            };

            let result = next.clone().run(duplicate_request, ext).await;

            // We classify the response which will return None if not
            // errors were returned.
            break match Retryable::from_reqwest_response(&result) {
                Some(retryable)
                    if retryable == Retryable::Transient && n_past_retries < self.max_retries =>
                {
                    // If the response failed and the error type was transient
                    // we can safely try to retry the request.
                    let mut retry_delay = 125u64 * 2u64.pow(n_past_retries);
                    if retry_delay > self.max_delay_ms {
                        retry_delay = self.max_delay_ms;
                    }
                    futures_timer::Delay::new(Duration::from_millis(retry_delay)).await;
                    n_past_retries += 1;
                    continue;
                }
                Some(_) | None => result,
            };
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Retryable {
    /// The failure was due to something tha might resolve in the future.
    Transient,
    /// Unresolvable error.
    Fatal,
}

impl Retryable {
    /// Try to map a `reqwest` response into `Retryable`.
    ///
    /// Returns `None` if the response object does not contain any errors.
    ///
    pub fn from_reqwest_response(
        res: &reqwest_middleware::Result<reqwest::Response>,
    ) -> Option<Self> {
        match res {
            Ok(success) => {
                let status = success.status();
                if status.is_server_error() {
                    Some(Retryable::Transient)
                } else if status.is_client_error()
                    && status != StatusCode::REQUEST_TIMEOUT
                    && status != StatusCode::TOO_MANY_REQUESTS
                {
                    Some(Retryable::Fatal)
                } else if status.is_success() {
                    None
                } else if status == StatusCode::REQUEST_TIMEOUT
                    || status == StatusCode::TOO_MANY_REQUESTS
                {
                    Some(Retryable::Transient)
                } else {
                    Some(Retryable::Fatal)
                }
            }
            Err(error) => match error {
                reqwest_middleware::Error::Middleware(_) => Some(Retryable::Fatal),
                reqwest_middleware::Error::Reqwest(error) => {
                    if error.is_timeout() || error.is_connect() {
                        Some(Retryable::Transient)
                    } else if error.is_body()
                        || error.is_decode()
                        || error.is_builder()
                        || error.is_redirect()
                        || error.is_request()
                    {
                        Some(Retryable::Fatal)
                    } else {
                        // We omit checking if error.is_status() since we check that already.
                        // However, if Response::error_for_status is used the status will still
                        // remain in the response object.
                        None
                    }
                }
            },
        }
    }
}