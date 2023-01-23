use crate::api::resource::*;
use crate::dto::data_organization::labels::*;
use crate::CogniteExternalId;

pub type Labels = Resource<Label>;

impl WithBasePath for Labels {
    const BASE_PATH: &'static str = "labels";
}

impl Create<AddLabel, Label> for Labels {}
impl FilterItems<LabelFilter, Label> for Labels {}
impl Delete<CogniteExternalId> for Labels {}
