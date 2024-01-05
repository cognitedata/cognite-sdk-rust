use crate::api::resource::*;
use crate::dto::{data_organization::relationships::*, items::ItemsWithCursor};
use crate::error::Result;
use crate::{CogniteExternalId, Patch};

/// API resource for relationships.
pub type RelationshipsResource = Resource<Relationship>;

impl WithBasePath for RelationshipsResource {
    const BASE_PATH: &'static str = "relationships";
}

impl Create<AddRelationship, Relationship> for RelationshipsResource {}
impl Update<Patch<PatchRelationship>, Relationship> for RelationshipsResource {}
impl DeleteWithIgnoreUnknownIds<CogniteExternalId> for RelationshipsResource {}
impl FilterWithRequest<FilterRelationshipsQuery, Relationship> for RelationshipsResource {}
impl RetrieveWithRequest<RetrieveRelationshipsRequest, Relationship> for RelationshipsResource {}

impl RelationshipsResource {
    /// Retrieve a list of relationships by their ID.
    ///
    /// # Arguments
    ///
    /// * `relationship_ids` - IDs of relationships to retrieve.
    /// * `ignore_unknown_ids` - Set this to `true` to ignore any IDs not found in CDF.
    /// If this is `false`, any missing IDs will cause the request to fail.
    /// * `fetch_resources` - Whether to fetch the associated resources along with the relationship itself.
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
