use crate::api::resource::Resource;
use crate::dto::iam::api_key::*;
use crate::dto::items::Items;
use crate::error::Result;
use crate::{List, WithBasePath};

pub type ApiKeys = Resource<ApiKey>;

impl WithBasePath for ApiKeys {
    const BASE_PATH: &'static str = "apikeys";
}

impl List<ApiKeyQuery, ApiKey> for ApiKeys {}

impl ApiKeys {
    pub async fn create(&self, service_account_ids: &[u64]) -> Result<Vec<ApiKey>> {
        let service_account_id_items = Items::from(service_account_ids);
        let api_keys_response: ApiKeyListResponse = self
            .api_client
            .post("apikeys", &service_account_id_items)
            .await?;
        Ok(api_keys_response.items)
    }

    pub async fn delete(&self, service_account_ids: &[u64]) -> Result<()> {
        let service_account_id_items = Items::from(service_account_ids);
        self.api_client
            .post::<::serde_json::Value, Items>("apikeys/delete", &service_account_id_items)
            .await?;
        Ok(())
    }
}
