use crate::{
    dto::data_modeling::data_models::{
        DataModel, DataModelCreate, DataModelFilter, DataModelId, DataModelQuery,
    },
    Create, DeleteWithResponse, FilterWithRequest, List, Resource, Retrieve, WithBasePath,
};

/// A data model is a collection of views. Use the data model to group and structure views into a
/// recognizable and understood model. The model represents a reusable collection of data.
pub type DataModelsResource = Resource<DataModel>;

impl WithBasePath for DataModelsResource {
    const BASE_PATH: &'static str = "models/datamodels";
}

impl Create<DataModelCreate, DataModel> for DataModelsResource {}
impl List<DataModelQuery, DataModel> for DataModelsResource {}
impl FilterWithRequest<DataModelFilter, DataModel> for DataModelsResource {}
impl DeleteWithResponse<DataModelId, DataModelId> for DataModelsResource {}
impl Retrieve<DataModelId, DataModel> for DataModelsResource {}
