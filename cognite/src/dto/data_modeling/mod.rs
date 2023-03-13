use std::sync::Arc;

use crate::{api::data_modeling::instances::Instances, ApiClient};

pub mod instances;

pub struct Models {
    pub instances: Instances,
}

impl Models {
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Models {
            instances: Instances::new(api_client),
        }
    }
}
