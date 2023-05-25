pub mod datamodels;
pub mod instances;
pub mod spaces;
pub mod views;

use std::sync::Arc;

use self::spaces::Spaces;
use crate::api::data_modeling::{datamodels::DataModels, instances::Instances, views::Views};
use crate::ApiClient;

pub struct Models {
    pub instances: Instances,
    pub views: Views,
    pub spaces: Spaces,
    pub datamodels: DataModels,
}

impl Models {
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Models {
            instances: Instances::new(api_client.clone()),
            views: Views::new(api_client.clone()),
            spaces: Spaces::new(api_client.clone()),
            datamodels: DataModels::new(api_client),
        }
    }
}
