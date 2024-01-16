use crate::api::resource::Resource;
use crate::dto::iam::session::*;
use crate::{CogniteId, Create, Items, List, Retrieve, WithBasePath};
use crate::{ItemsWithoutCursor, Result};

/// Sessions are used to maintain access to CDF resources for an extended period of time.
pub type SessionsResource = Resource<Session>;

impl WithBasePath for SessionsResource {
    const BASE_PATH: &'static str = "sessions";
}

impl List<SessionQuery, Session> for SessionsResource {}
impl Create<AddSession, Session> for SessionsResource {}
impl Retrieve<CogniteId, Session> for SessionsResource {}

impl SessionsResource {
    /// Revoke a list of sessions.
    ///
    /// # Arguments
    ///
    /// * `session_ids` - Sessions to revoke.
    pub async fn revoke(&self, session_ids: &[CogniteId]) -> Result<Vec<Session>> {
        let items = Items::from(session_ids);
        let response: ItemsWithoutCursor<Session> =
            self.api_client.post("sessions/revoke", &items).await?;
        Ok(response.items)
    }
}
