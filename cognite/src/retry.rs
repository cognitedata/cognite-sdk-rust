// This file is adapted from reqwest-retry, which was a bit too opinionated for our use.
// https://github.com/TrueLayer/reqwest-middleware

use async_trait::async_trait;
use http::Extensions;
use rand::{rng, RngExt};
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Middleware, Next, Result};
use std::time::Duration;

/// Middleware for retrying requests.
pub struct CustomRetryMiddleware {
    max_retries: u32,
    max_delay_ms: u64,
    initial_delay_ms: u64,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
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
                    retry_delay = retry_delay / 4 * 3 + rng().random_range(0..=(retry_delay / 2));
                    futures_timer::Delay::new(Duration::from_millis(retry_delay)).await;
                    n_past_retries += 1;
                    continue;
                }
                Some(_) | None => result,
            };
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
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
        res: &reqwest_middleware::Result<reqwest::Response>,
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
                reqwest_middleware::Error::Middleware(_) => Some(Retryable::Fatal),
                reqwest_middleware::Error::Reqwest(error) => {
                    #[cfg(not(target_arch = "wasm32"))]
                    let is_connect = error.is_connect();
                    #[cfg(target_arch = "wasm32")]
                    let is_connect = false;

                    if error.is_timeout() || is_connect {
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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    fn create_mock_response(status: StatusCode) -> reqwest::Response {
        http::Response::builder()
            .status(status)
            .body("")
            .unwrap()
            .into()
    }

    fn create_mock_response_with_header(
        status: StatusCode,
        header_name: &str,
        header_value: &str,
    ) -> reqwest::Response {
        http::Response::builder()
            .status(status)
            .header(header_name, header_value)
            .body("")
            .unwrap()
            .into()
    }

    #[test]
    fn test_retryable_success_response() {
        let response = create_mock_response(StatusCode::OK);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(Retryable::from_reqwest_response(&result), None);
    }

    #[test]
    fn test_retryable_unauthorized_401() {
        let response = create_mock_response(StatusCode::UNAUTHORIZED);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Unauthorized)
        );
    }

    #[test]
    fn test_retryable_rate_limit_429() {
        let response = create_mock_response(StatusCode::TOO_MANY_REQUESTS);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Transient)
        );
    }

    #[test]
    fn test_retryable_timeout_408() {
        let response = create_mock_response(StatusCode::REQUEST_TIMEOUT);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Transient)
        );
    }

    #[test]
    fn test_retryable_server_error_500() {
        let response = create_mock_response(StatusCode::INTERNAL_SERVER_ERROR);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Transient)
        );
    }

    #[test]
    fn test_retryable_server_error_502() {
        let response = create_mock_response(StatusCode::BAD_GATEWAY);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Transient)
        );
    }

    #[test]
    fn test_retryable_server_error_503() {
        let response = create_mock_response(StatusCode::SERVICE_UNAVAILABLE);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Transient)
        );
    }

    #[test]
    fn test_retryable_bad_request_400() {
        let response = create_mock_response(StatusCode::BAD_REQUEST);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Fatal)
        );
    }

    #[test]
    fn test_retryable_not_found_404() {
        let response = create_mock_response(StatusCode::NOT_FOUND);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Fatal)
        );
    }

    #[test]
    fn test_retryable_forbidden_403() {
        let response = create_mock_response(StatusCode::FORBIDDEN);
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Fatal)
        );
    }

    #[test]
    fn test_retryable_custom_header_makes_retryable() {
        let response = create_mock_response_with_header(
            StatusCode::BAD_REQUEST,
            "cdf-is-auto-retryable",
            "true",
        );
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Transient)
        );
    }

    #[test]
    fn test_retryable_custom_header_false_value_is_fatal() {
        let response = create_mock_response_with_header(
            StatusCode::BAD_REQUEST,
            "cdf-is-auto-retryable",
            "false",
        );
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Fatal)
        );
    }

    #[test]
    fn test_retryable_custom_header_overrides_server_error() {
        let response = create_mock_response_with_header(
            StatusCode::INTERNAL_SERVER_ERROR,
            "cdf-is-auto-retryable",
            "true",
        );
        let result: reqwest_middleware::Result<reqwest::Response> = Ok(response);
        
        // Server errors are already transient, but this tests the header is recognized
        assert_eq!(
            Retryable::from_reqwest_response(&result),
            Some(Retryable::Transient)
        );
    }

    #[test]
    fn test_retry_middleware_max_retries_capped_at_10() {
        let middleware = CustomRetryMiddleware::new(100, 5000, 100);
        assert_eq!(middleware.max_retries, 10);
    }

    #[test]
    fn test_retry_middleware_max_retries_normal() {
        let middleware = CustomRetryMiddleware::new(5, 5000, 100);
        assert_eq!(middleware.max_retries, 5);
    }
}
