use async_trait::async_trait;
use http::HeaderMap;
use reqwest_middleware::Middleware;

/// Middleware for adding custom headers to requests.
/// Can also be used to clear existing headers say for downloading files.
pub struct CustomHeadersMiddleware;

#[async_trait]
impl Middleware for CustomHeadersMiddleware {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let custom_headers = extensions.get::<HeaderMap>();

        if let Some(headers) = custom_headers {
            let req_headers = req.headers_mut();
            req_headers.clear();
            for (key, value) in headers.iter() {
                req_headers.insert(key.clone(), value.clone());
            }
        };

        next.run(req, extensions).await
    }
}
