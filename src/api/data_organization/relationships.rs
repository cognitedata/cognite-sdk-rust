use crate::api::resource::*;
use crate::dto::{data_organization::relationships::*, items::ItemsWithCursor};
use crate::error::Result;
use crate::{CogniteExternalId, Patch};

pub type Relationships = Resource<Relationship>;

impl WithBasePath for Relationships {
    const BASE_PATH: &'static str = "relationships";
}

impl Create<AddRelationship, Relationship> for Relationships {}
impl Update<Patch<PatchRelationship>, Relationship> for Relationships {}
impl DeleteWithIgnoreUnknownIds<CogniteExternalId> for Relationships {}
impl FilterWithRequest<FilterRelationshipsQuery, Relationship> for Relationships {}

impl Relationships {
    pub async fn retrieve(
        &self,
        relationship_ids: &[CogniteExternalId],
        ignore_unknown_ids: bool,
        fetch_resources: bool,
    ) -> Result<Vec<Relationship>> {
        let mut id_items = RetrieveRelationshipsRequest::from(relationship_ids);
        id_items.fetch_resources = fetch_resources;
        id_items.ignore_unknown_ids = ignore_unknown_ids;
        let rel_response: ItemsWithCursor<Relationship> = self
            .api_client
            .post("relationships/byids", &id_items)
            .await?;
        Ok(rel_response.items)
    }
}
