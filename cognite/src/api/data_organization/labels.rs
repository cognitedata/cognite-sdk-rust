use crate::api::resource::*;
use crate::dto::data_organization::labels::*;
use crate::CogniteExternalId;

/// API resource for labels.
pub type LabelsResource = Resource<Label>;

impl WithBasePath for LabelsResource {
    const BASE_PATH: &'static str = "labels";
}

impl WithChunkSizes for LabelsResource {
    const REQUEST_CHUNK_SIZE: usize = 1000;
    const REQUEST_PARALLELISM: usize = 4;
}

impl Create<AddLabel, Label> for LabelsResource {}
impl FilterItems<LabelFilter, Label> for LabelsResource {}
impl Delete<CogniteExternalId> for LabelsResource {}
