use crate::{
    dto::data_modeling::datamodels::{DataModel, DataModelCreate},
    models::{ItemIdOptionalVersion, ItemIdWithVersion},
    Create, DeleteWithResponse, LimitCursorQuery, List, Resource, Retrieve, WithBasePath,
};

pub struct DataModelResource {}
pub type DataModels = Resource<DataModelResource>;

impl WithBasePath for DataModels {
    const BASE_PATH: &'static str = "models/datamodels";
}

impl Create<DataModelCreate, DataModel> for DataModels {}
impl List<LimitCursorQuery, DataModel> for DataModels {}
impl Retrieve<ItemIdOptionalVersion, DataModel> for DataModels {}
impl DeleteWithResponse<ItemIdWithVersion, ItemIdWithVersion> for DataModels {}
