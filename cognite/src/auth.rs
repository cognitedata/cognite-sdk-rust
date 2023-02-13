use async_trait::async_trait;
use reqwest::{Request, Response};
use reqwest_middleware::{ClientWithMiddleware, Middleware, Next, Result};
use task_local_extensions::Extensions;

use crate::AuthHeaderManager;

pub struct AuthenticatorMiddleware {
    authenticator: AuthHeaderManager,
}

struct AuthenticatorFlag;

impl AuthenticatorMiddleware {
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
        if !extensions.contains::<AuthenticatorFlag>() {
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
