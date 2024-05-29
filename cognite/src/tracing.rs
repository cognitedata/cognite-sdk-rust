use std::time::{Duration, Instant};

use async_trait::async_trait;

use crate::reqwest::{Method, Request, Response, Url};
use crate::reqwest_middleware::{Middleware, Next, Result};
use crate::Extensions;

/// Trait for tracing requests going through the SDK.
pub trait RequestTracer {
    /// Observe a completed request.
    fn observe_request(
        &self,
        url: Url,
        method: Method,
        result: &Result<Response>,
        duration: Duration,
    );
}

/// Middleware for tracing requests sent with the SDK to a custom tracer.
pub struct TracingMiddleware<T> {
    tracer: T,
}

impl<T: RequestTracer> TracingMiddleware<T> {
    /// Create a new tracing middleware instance with the given tracer.
    pub fn new(tracer: T) -> Self {
        Self { tracer }
    }
}

#[async_trait]
impl<T: RequestTracer + Send + Sync + 'static> Middleware for TracingMiddleware<T> {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let start = Instant::now();
        let method = req.method().clone();
        let url = req.url().clone();

        let result = next.run(req, extensions).await;

        let duration = Instant::now() - start;
        self.tracer.observe_request(url, method, &result, duration);
        result
    }
}
