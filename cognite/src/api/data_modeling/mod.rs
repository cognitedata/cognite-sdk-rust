pub mod instances;
pub mod views;

use std::sync::Arc;

use crate::api::data_modeling::{instances::Instances, views::Views};
use crate::ApiClient;

pub struct Models {
    pub instances: Instances,
    pub views: Views,
}

impl Models {
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Models {
            instances: Instances::new(api_client.clone()),
            views: Views::new(api_client),
        }
    }
}
