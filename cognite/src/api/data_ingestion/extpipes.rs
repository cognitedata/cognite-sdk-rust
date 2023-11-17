use crate::{
    Create, DeleteWithIgnoreUnknownIds, FilterItems, Identity, Patch, Resource,
    RetrieveWithIgnoreUnknownIds, Update, WithBasePath,
};

use crate::extpipes::*;

/// Extraction pipelines represent applications and software running outside CDF.
pub type ExtPipesResource = Resource<ExtPipe>;

impl WithBasePath for ExtPipesResource {
    const BASE_PATH: &'static str = "extpipes";
}

impl Create<AddExtPipe, ExtPipe> for ExtPipesResource {}
impl DeleteWithIgnoreUnknownIds<Identity> for ExtPipesResource {}
impl Update<Patch<PatchExtPipe>, ExtPipe> for ExtPipesResource {}
impl RetrieveWithIgnoreUnknownIds<Identity, ExtPipe> for ExtPipesResource {}
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
