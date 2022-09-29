use crate::api::resource::*;
use crate::dto::iam::group::*;
use crate::dto::iam::service_account::*;
use crate::dto::items::Items;
use crate::error::Result;
use crate::Create;
use crate::WithBasePath;

pub type Groups = Resource<Group>;

impl WithBasePath for Groups {
    const BASE_PATH: &'static str = "groups";
}

impl Create<Group, Group> for Groups {}
impl List<GroupQuery, Group> for Groups {}
impl Delete<u64> for Groups {}

impl Groups {
    pub async fn list_service_accounts(&self, group_id: u64) -> Result<Vec<ServiceAccount>> {
        let service_accounts_response: ServiceAccountListResponse = self
            .api_client
            .get(&format!("groups/{}/serviceaccounts", group_id))
            .await?;
        Ok(service_accounts_response.items)
    }

    pub async fn add_service_accounts(
        &self,
        group_id: u64,
        service_account_ids: &[u64],
    ) -> Result<()> {
        let id_items = Items::from(service_account_ids);
        self.api_client
            .post::<::serde_json::Value, Items>(
                &format!("groups/{}/serviceaccounts", group_id),
                &id_items,
            )
            .await?;
        Ok(())
    }

    pub async fn remove_service_accounts(
        &self,
        group_id: u64,
        service_account_ids: &[u64],
    ) -> Result<()> {
        let id_items = Items::from(service_account_ids);
        self.api_client
            .post::<::serde_json::Value, Items>(
                &format!("groups/{}/serviceaccounts/remove", group_id),
                &id_items,
            )
            .await?;
        Ok(())
    }
}
