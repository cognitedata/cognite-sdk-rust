use serde::Serialize;

use crate::error::Result;
use crate::{
    to_query, Create, Cursor, DeleteWithIgnoreUnknownIds, FilterItems, IdentityList, IntoParams,
    ItemsVec, Patch, Resource, RetrieveWithIgnoreUnknownIds, Update, WithBasePath,
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

struct ConfigQuery {
    external_id: String,
    revision: Option<i32>,
    active_at_time: Option<i64>,
}

impl IntoParams for ConfigQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = vec![("externalId".to_string(), self.external_id)];
        to_query("revision", &self.revision, &mut params);
        to_query("activeAtTime", &self.active_at_time, &mut params);
        params
    }
}

struct RevisionsQuery {
    external_id: String,
    limit: Option<i32>,
    cursor: Option<String>,
}

impl IntoParams for RevisionsQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = vec![("externalId".to_string(), self.external_id)];
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        params
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RevertRequest<'a> {
    external_id: &'a str,
    revision: i32,
}

impl ExtPipeConfigsResource {
    /// Retrieve the latest revision, a specific revision, or the revision active at a time.
    pub async fn retrieve(
        &self,
        external_id: &str,
        revision: Option<i32>,
        active_at_time: Option<i64>,
    ) -> Result<ExtPipeConfig> {
        let query = ConfigQuery {
            external_id: external_id.to_owned(),
            revision,
            active_at_time,
        };
        self.api_client
            .get_with_params(Self::BASE_PATH, Some(query))
            .await
    }

    /// List revisions, oldest first, without their content.
    pub async fn list_revisions(
        &self,
        external_id: &str,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<ItemsVec<ExtPipeConfigRevision, Cursor>> {
        let query = RevisionsQuery {
            external_id: external_id.to_owned(),
            limit,
            cursor,
        };
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
