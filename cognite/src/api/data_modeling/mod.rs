pub mod containers;
pub mod data_models;
pub mod instances;
pub mod spaces;
pub mod views;

use std::sync::Arc;

use crate::api::data_modeling::{instances::Instances, views::ViewsResource};
use crate::ApiClient;

use self::containers::ContainersResource;
use self::data_models::DataModelsResource;
use self::spaces::SpacesResource;

/// API resource for data modeling.
pub struct Models {
    pub instances: Instances,
    pub views: ViewsResource,
    pub spaces: SpacesResource,
    pub data_models: DataModelsResource,
    pub containers: ContainersResource,
}

impl Models {
    pub(crate) fn new(api_client: Arc<ApiClient>) -> Self {
        Models {
            instances: Instances::new(api_client.clone()),
            views: ViewsResource::new(api_client.clone()),
            spaces: SpacesResource::new(api_client.clone()),
            data_models: DataModelsResource::new(api_client.clone()),
            containers: ContainersResource::new(api_client),
        }
    }
}
