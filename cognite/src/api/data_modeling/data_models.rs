use crate::{
    dto::data_modeling::data_models::{
        DataModel, DataModelCreate, DataModelFilter, DataModelId, DataModelQuery,
    },
    Create, DeleteWithResponse, FilterWithRequest, List, Resource, Retrieve, WithBasePath,
};

pub struct _DataModel {}
pub type DataModels = Resource<_DataModel>;

impl WithBasePath for DataModels {
    const BASE_PATH: &'static str = "models/datamodels";
}

impl Create<DataModelCreate, DataModel> for DataModels {}
impl List<DataModelQuery, DataModel> for DataModels {}
impl FilterWithRequest<DataModelFilter, DataModel> for DataModels {}
impl DeleteWithResponse<DataModelId, DataModelId> for DataModels {}
impl Retrieve<DataModelId, DataModel> for DataModels {}
