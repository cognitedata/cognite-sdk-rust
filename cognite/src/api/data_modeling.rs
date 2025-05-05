pub(crate) mod containers;
pub(crate) mod data_models;
pub(crate) mod instances;
// pub(crate) mod resource;
pub(crate) mod spaces;
pub(crate) mod streams;
pub(crate) mod views;

use std::sync::Arc;

use streams::StreamsResource;

use crate::api::data_modeling::{instances::Instances, views::ViewsResource};
use crate::ApiClient;

use self::containers::ContainersResource;
use self::data_models::DataModelsResource;
use self::spaces::SpacesResource;

/// API resource for data modeling.
pub struct Models {
    /// Data model instances (nodes and edges)
    pub instances: Instances,
    /// Data modeling views.
    pub views: ViewsResource,
    /// Data modeling spaces.
    pub spaces: SpacesResource,
    /// Data models.
    pub data_models: DataModelsResource,
    /// Data modeling containers.
    pub containers: ContainersResource,
    /// Data modeling streams.
    pub streams: StreamsResource,
}

impl Models {
    pub(crate) fn new(api_client: Arc<ApiClient>) -> Self {
        Models {
            instances: Instances::new(api_client.clone()),
            views: ViewsResource::new(api_client.clone()),
            spaces: SpacesResource::new(api_client.clone()),
            data_models: DataModelsResource::new(api_client.clone()),
            containers: ContainersResource::new(api_client.clone()),
            streams: StreamsResource::new(Arc::new(api_client.clone_with_api_version("beta"))),
        }
    }
}
