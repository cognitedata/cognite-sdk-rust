pub mod instances;
pub mod spaces;
pub mod views;

use std::sync::Arc;

use crate::api::data_modeling::{instances::Instances, views::Views};
use crate::ApiClient;

use self::spaces::Spaces;

pub struct Models {
    pub instances: Instances,
    pub views: Views,
    pub spaces: Spaces,
}

impl Models {
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Models {
            instances: Instances::new(api_client.clone()),
            views: Views::new(api_client.clone()),
            spaces: Spaces::new(api_client),
        }
    }
}
