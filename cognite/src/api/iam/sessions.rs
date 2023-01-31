use crate::api::resource::Resource;
use crate::dto::iam::session::*;
use crate::{CogniteId, Create, Items, List, Retrieve, WithBasePath};
use crate::{ItemsWithoutCursor, Result};

pub type Sessions = Resource<Session>;

impl WithBasePath for Sessions {
    const BASE_PATH: &'static str = "sessions";
}

impl List<SessionQuery, Session> for Sessions {}
impl Create<AddSession, Session> for Sessions {}
impl Retrieve<CogniteId, Session> for Sessions {}

impl Sessions {
    pub async fn revoke(&self, session_ids: &[CogniteId]) -> Result<Vec<Session>> {
        let items = Items::from(session_ids);
        let response: ItemsWithoutCursor<Session> =
            self.api_client.post("sessions/revoke", &items).await?;
        Ok(response.items)
    }
}
