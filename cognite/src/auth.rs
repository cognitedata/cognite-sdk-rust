use async_trait::async_trait;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{ClientWithMiddleware, Middleware, Next, Result};

use crate::AuthHeaderManager;

/// Middleware for token authentication.
///
/// Note that in order to use this, you need to add `ClientWithMiddleware` as an extension
/// to your requests.
pub struct AuthenticatorMiddleware {
    authenticator: AuthHeaderManager,
}

#[derive(Clone)]
struct AuthenticatorFlag;

impl AuthenticatorMiddleware {
    /// Create a new authenticator middleware from an authenticator.
    ///
    /// # Arguments
    ///
    /// * `authenticator` - Header manager.
    pub fn new(authenticator: AuthHeaderManager) -> crate::Result<Self> {
        Ok(Self { authenticator })
    }
}

#[async_trait]
impl Middleware for AuthenticatorMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        // Since we are reusing the client, we want to avoid infinitely calling the authenticator recursively,
        // so we add a flag indicating that we have already called the authenticator in this chain.
        if extensions.get::<AuthenticatorFlag>().is_none() {
            // Add the flag before we call the authenticator, this prevents the authenticator from
            // attempting to add headers to its own request, which would deadlock.
            extensions.insert(AuthenticatorFlag);
            // This is all a little hacky, we add the client itself as an extension to the request
            // so that we can use it from in here. The deadlocky-ness of this is exactly why it isn't
            // possible by default.
            // If it isn't in there we assume that it isn't supposed to be there, and skip the whole layer.
            if let Some(client) = extensions.get::<ClientWithMiddleware>() {
                self.authenticator
                    .set_headers(req.headers_mut(), client)
                    .await
                    .map_err(|e| reqwest_middleware::Error::Middleware(e.into()))?;
            }
            // Once we're done, remove the flag
            extensions.remove::<AuthenticatorFlag>();
        }
        next.run(req, extensions).await
    }
}
