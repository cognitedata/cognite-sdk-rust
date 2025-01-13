// This file is adapted from reqwest-retry, which was a bit too opinionated for our use.
// https://github.com/TrueLayer/reqwest-middleware

use crate::reqwest::{Request, Response, StatusCode};
use crate::reqwest_middleware::{Middleware, Next, Result};
use crate::Extensions;
use async_trait::async_trait;
use rand::{thread_rng, Rng};
use std::time::Duration;

/// Middleware for retrying requests.
pub struct CustomRetryMiddleware {
    max_retries: u32,
    max_delay_ms: u64,
    initial_delay_ms: u64,
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
    /// Create a new retry middleware instance.
    pub fn new(max_retries: u32, max_delay_ms: u64, initial_delay_ms: u64) -> Self {
        Self {
            max_retries: max_retries.min(10),
            max_delay_ms,
            initial_delay_ms,
        }
    }

    async fn execute_with_retry<'a>(
        &'a self,
        req: Request,
        next: Next<'a>,
        ext: &'a mut Extensions,
    ) -> Result<Response> {
        let mut n_past_retries = 0;
        let mut last_req_401 = false;
        loop {
            let duplicate_request = match req.try_clone() {
                Some(x) => x,
                None => return next.run(req, ext).await,
            };

            let result = next.clone().run(duplicate_request, ext).await;

            // Check if the error can be retried.
            break match Retryable::from_reqwest_response(&result) {
                Some(retryable)
                    if (retryable == Retryable::Transient
                        || retryable == Retryable::Unauthorized && !last_req_401)
                        && n_past_retries < self.max_retries =>
                {
                    last_req_401 = retryable == Retryable::Unauthorized;
                    // If the response failed and the error type was transient
                    // we can safely try to retry the request.
                    let mut retry_delay = self.initial_delay_ms * 2u64.pow(n_past_retries);
                    if retry_delay > self.max_delay_ms {
                        retry_delay = self.max_delay_ms;
                    }
                    // Jitter so we land between initial * 2 ** attempt * 3/4 and initial * 2 ** attempt * 5/4
                    retry_delay =
                        retry_delay / 4 * 3 + thread_rng().gen_range(0..=(retry_delay / 2));
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
pub(crate) enum Retryable {
    /// The failure was due to something that might resolve in the future.
    Transient,
    /// Unresolvable error.
    Fatal,
    /// Unauthorized. This is _maybe_ resolvable, if the last request wasn't also a 401.
    Unauthorized,
}

impl Retryable {
    /// Try to map a `reqwest` response into `Retryable`.
    ///
    /// Returns `None` if the response object does not contain any errors.
    ///
    /// # Arguments
    ///
    /// * `res` - Request response.
    pub fn from_reqwest_response(
        res: &crate::reqwest_middleware::Result<crate::reqwest::Response>,
    ) -> Option<Self> {
        match res {
            Ok(success) => {
                let status = success.status();
                if status.is_success() {
                    None
                } else if status == StatusCode::UNAUTHORIZED {
                    Some(Retryable::Unauthorized)
                } else if status.is_server_error()
                    || status == StatusCode::REQUEST_TIMEOUT
                    || status == StatusCode::TOO_MANY_REQUESTS
                    || success
                        .headers()
                        .get("cdf-is-auto-retryable")
                        .and_then(|v| v.to_str().ok())
                        .is_some_and(|v| v == "true")
                {
                    Some(Retryable::Transient)
                } else {
                    Some(Retryable::Fatal)
                }
            }
            Err(error) => match error {
                crate::reqwest_middleware::Error::Middleware(_) => Some(Retryable::Fatal),
                crate::reqwest_middleware::Error::Reqwest(error) => {
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
