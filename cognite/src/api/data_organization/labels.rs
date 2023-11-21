use crate::api::resource::*;
use crate::dto::data_organization::labels::*;
use crate::CogniteExternalId;

/// Labels let you annotate resources such as assets and time series.
pub type LabelsResource = Resource<Label>;

impl WithBasePath for LabelsResource {
    const BASE_PATH: &'static str = "labels";
}

impl Create<AddLabel, Label> for LabelsResource {}
impl FilterItems<LabelFilter, Label> for LabelsResource {}
impl Delete<CogniteExternalId> for LabelsResource {}
