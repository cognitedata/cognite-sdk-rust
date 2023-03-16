use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use futures::future::try_join_all;
use serde::{de::DeserializeOwned, Serialize};

use crate::dto::items::*;
use crate::{
    ApiClient, AsParams, EqIdentity, Filter, Identity, Partition, Patch, Result, Search, SetCursor,
    WithPartition,
};

use super::utils::{get_duplicates_from_result, get_missing_from_result};

pub struct Resource<T> {
    pub api_client: Arc<ApiClient>,
    marker: PhantomData<T>,
}

impl<T> Resource<T> {
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Resource {
            api_client,
            marker: PhantomData,
        }
    }
}

impl<T> WithApiClient for Resource<T> {
    fn get_client(&self) -> &ApiClient {
        &self.api_client
    }
}

pub trait WithApiClient {
    fn get_client(&self) -> &ApiClient;
}

pub trait WithBasePath {
    const BASE_PATH: &'static str;
}

#[async_trait]
pub trait List<TParams, TResponse>
where
    TParams: AsParams + Send + Sync + 'static,
    TResponse: Serialize + DeserializeOwned + Send + Sync,
    Self: WithApiClient + WithBasePath,
{
    async fn list(&self, params: Option<TParams>) -> Result<ItemsWithCursor<TResponse>> {
        Ok(self
            .get_client()
            .get_with_params(Self::BASE_PATH, params)
            .await?)
    }
}

#[async_trait]
pub trait Create<TCreate, TResponse>
where
    TCreate: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned + Send,
    Self: WithApiClient + WithBasePath,
{
    async fn create(&self, creates: &[TCreate]) -> Result<Vec<TResponse>> {
        let items = Items::from(creates);
        let response: ItemsWithoutCursor<TResponse> =
            self.get_client().post(Self::BASE_PATH, &items).await?;
        Ok(response.items)
    }

    async fn create_from<T: 'a, 'a>(&self, creates: &'a [T]) -> Result<Vec<TResponse>>
    where
        T: std::marker::Sync,
        TCreate: From<&'a T>,
    {
        let to_add: Vec<TCreate> = creates.iter().map(TCreate::from).collect();
        self.create(&to_add).await
    }

    async fn create_ignore_duplicates(&self, creates: &[TCreate]) -> Result<Vec<TResponse>>
    where
        TCreate: EqIdentity,
    {
        let resp = self.create(creates).await;

        let duplicates: Option<Vec<Identity>> = get_duplicates_from_result(&resp);

        if let Some(duplicates) = duplicates {
            let next: Vec<&TCreate> = creates
                .iter()
                .filter(|c| !duplicates.iter().any(|i| c.eq(i)))
                .collect();

            if next.is_empty() {
                if duplicates.len() == creates.len() {
                    return Ok(vec![]);
                }
                return resp;
            }

            let items = Items::from(next);
            let response: ItemsWithoutCursor<TResponse> =
                self.get_client().post(Self::BASE_PATH, &items).await?;
            Ok(response.items)
        } else {
            resp
        }
    }

    async fn create_from_ignore_duplicates<T: 'a, 'a>(
        &self,
        creates: &'a [T],
    ) -> Result<Vec<TResponse>>
    where
        T: std::marker::Sync,
        TCreate: From<&'a T> + EqIdentity,
    {
        let to_add: Vec<TCreate> = creates.iter().map(TCreate::from).collect();
        self.create_ignore_duplicates(&to_add).await
    }
}

pub trait FromDuplicateCreate<'a, TCreate> {
    fn from(create: &'a TCreate, id: Identity) -> Self;
}

#[async_trait]
pub trait Upsert<TCreate, TUpdate, TResponse, 'a>
where
    TCreate: Serialize + Sync + Send + EqIdentity + 'a,
    TUpdate: Serialize + Sync + Send + From<&'a TCreate> + Default,
    TResponse: Serialize + DeserializeOwned + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    async fn upsert(&'a self, upserts: &'a [TCreate]) -> Result<Vec<TResponse>> {
        let items = Items::from(upserts);
        let resp: Result<ItemsWithoutCursor<TResponse>> =
            self.get_client().post(Self::BASE_PATH, &items).await;

        let duplicates: Option<Vec<Identity>> = get_duplicates_from_result(&resp);

        if let Some(duplicates) = duplicates {
            let mut to_create = Vec::with_capacity(upserts.len() - duplicates.len());
            let mut to_update = Vec::with_capacity(duplicates.len());
            for it in upserts {
                let idt = duplicates.iter().find(|i| it.eq(i));
                if let Some(idt) = idt {
                    to_update.push(Patch::<TUpdate> {
                        id: idt.clone(),
                        update: TUpdate::from(it),
                    });
                } else {
                    to_create.push(it);
                }
            }

            let create_response: ItemsWithoutCursor<TResponse> = self
                .get_client()
                .post(Self::BASE_PATH, &Items::from(to_create))
                .await?;
            let update_response: ItemsWithoutCursor<TResponse> = self
                .get_client()
                .post(
                    &format!("{}/update", Self::BASE_PATH),
                    &Items::from(&to_update),
                )
                .await?;
            Ok(create_response
                .items
                .into_iter()
                .chain(update_response.items.into_iter())
                .collect())
        } else {
            resp.map(|i| i.items)
        }
    }
}

impl<'a, T, TCreate, TUpdate, TResponse> Upsert<'a, TCreate, TUpdate, TResponse> for T
where
    T: Create<TCreate, TResponse> + Update<Patch<TUpdate>, TResponse>,
    TCreate: Serialize + Sync + Send + EqIdentity + 'a,
    TUpdate: Serialize + Sync + Send + From<&'a TCreate> + Default,
    TResponse: Serialize + DeserializeOwned + Sync + Send,
{
}

#[async_trait]
pub trait UpsertCollection<TUpsert, TResponse> {
    async fn upsert(&self, collection: &TUpsert) -> Result<Vec<TResponse>>
    where
        TUpsert: Serialize + Sync + Send,
        TResponse: Serialize + DeserializeOwned + Sync + Send,
        Self: WithApiClient + WithBasePath,
    {
        let response: ItemsWithoutCursor<TResponse> =
            self.get_client().post(Self::BASE_PATH, &collection).await?;
        Ok(response.items)
    }
}

#[async_trait]
pub trait Delete<TIdt>
where
    TIdt: Serialize + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    async fn delete(&self, deletes: &[TIdt]) -> Result<()> {
        let items = Items::from(deletes);
        self.get_client()
            .post::<::serde_json::Value, Items>(&format!("{}/delete", Self::BASE_PATH), &items)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait DeleteWithRequest<TReq>
where
    TReq: Serialize + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    async fn delete(&self, req: &TReq) -> Result<()> {
        self.get_client()
            .post::<::serde_json::Value, TReq>(&format!("{}/delete", Self::BASE_PATH), req)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait DeleteWithIgnoreUnknownIds<TIdt>
where
    TIdt: Serialize + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    async fn delete(&self, deletes: &[TIdt], ignore_unknown_ids: bool) -> Result<()> {
        let mut req = ItemsWithIgnoreUnknownIds::from(deletes);
        req.ignore_unknown_ids = ignore_unknown_ids;
        self.get_client()
            .post::<::serde_json::Value, ItemsWithIgnoreUnknownIds>(
                &format!("{}/delete", Self::BASE_PATH),
                &req,
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait DeleteWithResponse<TIdt, TResponse>
where
    TIdt: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    async fn delete(&self, deletes: &[TIdt]) -> Result<ItemsWithoutCursor<TResponse>> {
        let items = Items::from(deletes);
        let response: ItemsWithoutCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/delete", Self::BASE_PATH), &items)
            .await?;
        Ok(response)
    }
}

#[async_trait]
pub trait Update<TUpdate, TResponse>
where
    TUpdate: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    async fn update(&self, updates: &[TUpdate]) -> Result<Vec<TResponse>> {
        let items = Items::from(updates);
        let response: ItemsWithoutCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/update", Self::BASE_PATH), &items)
            .await?;
        Ok(response.items)
    }

    async fn update_from<T: 'a, 'a>(&self, updates: &'a [T]) -> Result<Vec<TResponse>>
    where
        T: std::marker::Sync,
        TUpdate: From<&'a T>,
    {
        let to_update: Vec<TUpdate> = updates.iter().map(TUpdate::from).collect();
        self.update(&to_update).await
    }

    async fn update_ignore_unknown_ids(&self, updates: &[TUpdate]) -> Result<Vec<TResponse>>
    where
        TUpdate: EqIdentity,
        TResponse: Send,
    {
        let response = self.update(updates).await;
        let missing: Option<Vec<Identity>> = get_missing_from_result(&response);

        if let Some(missing) = missing {
            let next: Vec<&TUpdate> = updates
                .iter()
                .filter(|c| !missing.iter().any(|i| c.eq(i)))
                .collect();

            if next.is_empty() {
                if missing.len() == updates.len() {
                    return Ok(vec![]);
                }
                return response;
            }

            let items = Items::from(next);
            let response: ItemsWithoutCursor<TResponse> = self
                .get_client()
                .post(&format!("{}/update", Self::BASE_PATH), &items)
                .await?;
            Ok(response.items)
        } else {
            response
        }
    }

    async fn update_from_ignore_unknown_ids<T: 'a, 'a>(
        &self,
        updates: &'a [T],
    ) -> Result<Vec<TResponse>>
    where
        T: std::marker::Sync,
        TUpdate: From<&'a T> + EqIdentity,
        TResponse: Send,
    {
        let to_update: Vec<TUpdate> = updates.iter().map(TUpdate::from).collect();
        self.update_ignore_unknown_ids(&to_update).await
    }
}

#[async_trait]
pub trait Retrieve<TIdt, TResponse>
where
    TIdt: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    async fn retrieve(&self, ids: &[TIdt]) -> Result<Vec<TResponse>> {
        let items = Items::from(ids);
        let response: ItemsWithoutCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/byids", Self::BASE_PATH), &items)
            .await?;
        Ok(response.items)
    }
}

#[async_trait]
pub trait RetrieveWithIgnoreUnknownIds<TIdt, TResponse>
where
    TIdt: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    async fn retrieve(&self, ids: &[TIdt], ignore_unknown_ids: bool) -> Result<Vec<TResponse>> {
        let mut items = ItemsWithIgnoreUnknownIds::from(ids);
        items.ignore_unknown_ids = ignore_unknown_ids;
        let response: ItemsWithoutCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/byids", Self::BASE_PATH), &items)
            .await?;
        Ok(response.items)
    }
}

#[async_trait]
pub trait FilterItems<TFilter, TResponse>
where
    TFilter: Serialize + Sync + Send + 'static,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    async fn filter_items(
        &self,
        filter: TFilter,
        cursor: Option<String>,
        limit: Option<u32>,
    ) -> Result<ItemsWithCursor<TResponse>> {
        let filter = Filter::<TFilter>::new(filter, cursor, limit);
        let response: ItemsWithCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/list", Self::BASE_PATH), &filter)
            .await?;
        Ok(response)
    }
}

impl<TFilter, TResponse, T> FilterWithRequest<Filter<TFilter>, TResponse> for T
where
    TFilter: Serialize + Sync + Send + 'static,
    TResponse: Serialize + DeserializeOwned,
    T: FilterItems<TFilter, TResponse>,
    Self: WithApiClient + WithBasePath,
{
}

#[async_trait]
pub trait FilterWithRequest<TFilter, TResponse>
where
    TFilter: Serialize + Sync + Send + 'static,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    async fn filter(&self, filter: TFilter) -> Result<ItemsWithCursor<TResponse>> {
        let response: ItemsWithCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/list", Self::BASE_PATH), &filter)
            .await?;
        Ok(response)
    }

    async fn filter_all(&self, mut filter: TFilter) -> Result<Vec<TResponse>>
    where
        TFilter: SetCursor,
        TResponse: Send,
    {
        let mut result = vec![];
        loop {
            let response: ItemsWithCursor<TResponse> = self
                .get_client()
                .post(&format!("{}/list", Self::BASE_PATH), &filter)
                .await?;
            for it in response.items {
                result.push(it);
            }
            match response.next_cursor {
                Some(cursor) => filter.set_cursor(Some(cursor)),
                None => return Ok(result),
            }
        }
    }

    async fn filter_all_partitioned(
        &self,
        filter: TFilter,
        num_partitions: u32,
    ) -> Result<Vec<TResponse>>
    where
        TFilter: SetCursor + WithPartition,
        TResponse: Send,
    {
        let mut futures = Vec::with_capacity(num_partitions as usize);
        for partition in 0..num_partitions {
            let part_filter = filter.with_partition(Partition::new(partition + 1, num_partitions));
            futures.push(self.filter_all(part_filter));
        }
        let results = try_join_all(futures).await?;
        let mut response_items = Vec::with_capacity(results.iter().map(|i| i.len()).sum());
        for chunk in results.into_iter() {
            response_items.extend(chunk);
        }
        Ok(response_items)
    }
}

#[async_trait]
pub trait SearchItems<TFilter, TSearch, TResponse, 'a>
where
    TFilter: Serialize + Sync + Send + 'a,
    TSearch: Serialize + Sync + Send + 'a,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    async fn search(
        &'a self,
        filter: TFilter,
        search: TSearch,
        limit: Option<u32>,
    ) -> Result<Vec<TResponse>> {
        let req = Search::<TFilter, TSearch>::new(filter, search, limit);
        let response: ItemsWithoutCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/search", Self::BASE_PATH), &req)
            .await?;
        Ok(response.items)
    }
}
