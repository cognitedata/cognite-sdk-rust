use crate::api::resource::Resource;
use crate::dto::iam::service_account::*;
use crate::dto::items::Items;
use crate::error::Result;

pub type ServiceAccounts = Resource<ServiceAccount>;

impl ServiceAccounts {
    pub async fn list_all(&self) -> Result<Vec<ServiceAccount>> {
        let service_accounts_response: ServiceAccountListResponse =
            self.api_client.get("serviceaccounts").await?;
        Ok(service_accounts_response.items)
    }

    pub async fn create(&self, service_accounts: &[ServiceAccount]) -> Result<Vec<ServiceAccount>> {
        let service_accounts_items = Items::from(service_accounts);
        let service_accounts_response: ServiceAccountListResponse = self
            .api_client
            .post("serviceaccounts", &service_accounts_items)
            .await?;
        Ok(service_accounts_response.items)
    }

    pub async fn delete(&self, service_account_ids: &[u64]) -> Result<()> {
        let id_items = Items::from(service_account_ids);
        self.api_client
            .post::<::serde_json::Value, Items>("serviceaccounts/delete", &id_items)
            .await?;
        Ok(())
    }
}
