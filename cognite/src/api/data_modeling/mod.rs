pub mod containers;
pub mod data_models;
pub mod instances;
pub mod spaces;
pub mod views;

use std::sync::Arc;

use crate::api::data_modeling::{instances::Instances, views::Views};
use crate::ApiClient;

use self::containers::Containers;
use self::data_models::DataModels;
use self::spaces::Spaces;

pub struct Models {
    pub instances: Instances,
    pub views: Views,
    pub spaces: Spaces,
    pub data_models: DataModels,
    pub containers: Containers,
}

impl Models {
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Models {
            instances: Instances::new(api_client.clone()),
            views: Views::new(api_client.clone()),
            spaces: Spaces::new(api_client.clone()),
            data_models: DataModels::new(api_client.clone()),
            containers: Containers::new(api_client),
        }
    }
}
