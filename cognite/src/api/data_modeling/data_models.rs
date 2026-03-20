use crate::{
    dto::data_modeling::data_models::{DataModel, DataModelCreate, DataModelId, DataModelQuery},
    Create, DeleteWithResponse, List, Resource, Retrieve, WithBasePath, WithChunkSizes,
};

/// A data model is a collection of views. Use the data model to group and structure views into a
/// recognizable and understood model. The model represents a reusable collection of data.
pub type DataModelsResource = Resource<DataModel>;

impl WithBasePath for DataModelsResource {
    const BASE_PATH: &'static str = "models/datamodels";
}

impl WithChunkSizes for DataModelsResource {
    const REQUEST_CHUNK_SIZE: usize = 100;
    const REQUEST_PARALLELISM: usize = 2;
}

impl Create<DataModelCreate, DataModel> for DataModelsResource {}
impl List<DataModelQuery, DataModel> for DataModelsResource {}
impl DeleteWithResponse<DataModelId, DataModelId> for DataModelsResource {}
impl Retrieve<DataModelId, DataModel> for DataModelsResource {}
