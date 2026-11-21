use serde::Serialize;

use crate::error::Result;
use crate::{
    Create, Cursor, DeleteWithIgnoreUnknownIds, FilterItems, IdentityList, ItemsVec, Patch,
    Resource, RetrieveWithIgnoreUnknownIds, Update, WithBasePath,
};

use crate::extpipes::*;

/// Extraction pipelines represent applications and software running outside CDF.
pub type ExtPipesResource = Resource<ExtPipe>;

impl WithBasePath for ExtPipesResource {
    const BASE_PATH: &'static str = "extpipes";
}

impl Create<AddExtPipe, ExtPipe> for ExtPipesResource {}
impl<R> DeleteWithIgnoreUnknownIds<IdentityList<R>> for ExtPipesResource
where
    IdentityList<R>: Serialize,
    R: Send + Sync,
{
}
impl Update<Patch<PatchExtPipe>, ExtPipe> for ExtPipesResource {}
impl<R> RetrieveWithIgnoreUnknownIds<IdentityList<R>, ExtPipe> for ExtPipesResource
where
    IdentityList<R>: Serialize,
    R: Send + Sync,
{
}
impl FilterItems<ExtPipeFilter, ExtPipe> for ExtPipesResource {}

/// Extraction pipeline runs represent statuses related to an extraction pipeline.
/// The supported statuses are: `success`, `failure`, and `seen`.
///
/// An extraction pipeline can be configured to create notifications when
/// the state of the extraction pipeline changes.
pub type ExtPipeRunsResource = Resource<ExtPipeRun>;

impl WithBasePath for ExtPipeRunsResource {
    const BASE_PATH: &'static str = "extpipes/runs";
}

impl Create<AddExtPipeRun, ExtPipeRun> for ExtPipeRunsResource {}
impl FilterItems<ExtPipeRunFilter, ExtPipeRun> for ExtPipeRunsResource {}

/// Versioned extractor configurations stored in CDF.
pub type ExtPipeConfigsResource = Resource<ExtPipeConfig>;

impl WithBasePath for ExtPipeConfigsResource {
    const BASE_PATH: &'static str = "extpipes/config";
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RevertRequest<'a> {
    external_id: &'a str,
    revision: i32,
}

impl ExtPipeConfigsResource {
    /// Retrieve the latest revision, a specific revision, or the revision active at a time.
    pub async fn retrieve(&self, query: ExtPipeConfigQuery) -> Result<ExtPipeConfig> {
        self.api_client
            .get_with_params(Self::BASE_PATH, Some(query))
            .await
    }

    /// List revisions, oldest first, without their content.
    pub async fn list_revisions(
        &self,
        query: ExtPipeConfigRevisionsQuery,
    ) -> Result<ItemsVec<ExtPipeConfigRevision, Cursor>> {
        self.api_client
            .get_with_params(&format!("{}/revisions", Self::BASE_PATH), Some(query))
            .await
    }

    /// Create a new revision.
    pub async fn create(&self, config: &AddExtPipeConfig) -> Result<ExtPipeConfig> {
        self.api_client.post(Self::BASE_PATH, config).await
    }

    /// Create a new revision with the content of an older one.
    pub async fn revert(&self, external_id: &str, revision: i32) -> Result<ExtPipeConfig> {
        let body = RevertRequest {
            external_id,
            revision,
        };
        self.api_client
            .post(&format!("{}/revert", Self::BASE_PATH), &body)
            .await
    }
}
