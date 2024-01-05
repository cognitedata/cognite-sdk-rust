use std::collections::VecDeque;
use std::pin::Pin;
use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use futures::future::try_join_all;
use futures::stream::try_unfold;
use futures::TryStream;
use serde::{de::DeserializeOwned, Serialize};

use crate::dto::items::*;
use crate::{
    ApiClient, AsParams, EqIdentity, Filter, Identity, IntoPatch, Partition, Patch, Result, Search,
    SetCursor, WithPartition,
};

use super::utils::{get_duplicates_from_result, get_missing_from_result};

/// A resource instance contains methods for accessing a single
/// CDF resource type.
pub struct Resource<T> {
    /// A reference to the shared API Client.
    pub api_client: Arc<ApiClient>,
    marker: PhantomData<T>,
}

impl<T> Resource<T> {
    /// Create a new resource with given API client.
    ///
    /// # Arguments
    ///
    /// * `api_client` - API client reference.
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

/// Trait for a type that contains an API client.
pub trait WithApiClient {
    /// Get the API client for this type.
    fn get_client(&self) -> &ApiClient;
}

/// Trait for a type with a base path.
pub trait WithBasePath {
    /// Base path for this resource type.
    const BASE_PATH: &'static str;
}

#[async_trait]
/// Trait for simple GET / endpoints.
pub trait List<TParams, TResponse>
where
    TParams: AsParams + Send + Sync + 'static,
    TResponse: Serialize + DeserializeOwned + Send + Sync,
    Self: WithApiClient + WithBasePath,
{
    /// Query a resource with optional query parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters.
    async fn list(&self, params: Option<TParams>) -> Result<ItemsWithCursor<TResponse>> {
        Ok(self
            .get_client()
            .get_with_params(Self::BASE_PATH, params)
            .await?)
    }

    /// Query a resource with query parameters, continuing until the cursor is exhausted.
    ///
    /// # Arguments
    ///
    /// * `params` - Initial query parameters. This can contain a cursor which is the starting point.
    async fn list_all(&self, mut params: TParams) -> Result<Vec<TResponse>>
    where
        TParams: SetCursor + Clone,
        TResponse: Send,
    {
        let mut result = vec![];
        loop {
            let lparams = params.clone();
            let response: ItemsWithCursor<TResponse> = self
                .get_client()
                .get_with_params(Self::BASE_PATH, Some(lparams))
                .await?;
            for it in response.items {
                result.push(it);
            }
            match response.next_cursor {
                Some(cursor) => params.set_cursor(Some(cursor)),
                None => return Ok(result),
            }
        }
    }

    /// List resources, following cursors. This returns a stream, you can abort the stream whenever you
    /// want and only resources retrieved up to that point will be returned.
    ///
    /// Each item in the stream will be a result, after the first error is returned the
    /// stream will end.
    ///
    /// # Arguments
    ///
    /// * `params` - Initial query parameters. This can contain a cursors used as starting point.
    fn list_all_stream<'a>(
        &'a self,
        params: TParams,
    ) -> Pin<Box<dyn TryStream<Ok = TResponse, Error = crate::Error, Item = Result<TResponse>> + 'a>>
    where
        TParams: SetCursor + Clone,
        TResponse: Send + 'static,
    {
        let state = CursorStreamState {
            req: params,
            responses: VecDeque::new(),
            next_cursor: CursorState::Initial,
        };

        Box::pin(try_unfold(state, move |mut state| async move {
            if let Some(next) = state.responses.pop_front() {
                Ok(Some((next, state)))
            } else {
                let cursor = match std::mem::take(&mut state.next_cursor) {
                    CursorState::Initial => None,
                    CursorState::Some(x) => Some(x),
                    CursorState::End => {
                        return Ok(None);
                    }
                };
                state.req.set_cursor(cursor);
                let response: ItemsWithCursor<TResponse> = self
                    .get_client()
                    .get_with_params(Self::BASE_PATH, Some(state.req.clone()))
                    .await?;

                state.responses.extend(response.items);
                state.next_cursor = match response.next_cursor {
                    Some(x) => CursorState::Some(x),
                    None => CursorState::End,
                };
                if let Some(next) = state.responses.pop_front() {
                    Ok(Some((next, state)))
                } else {
                    Ok(None)
                }
            }
        }))
    }
}

#[async_trait]
/// Trait for creating resources with POST / requests.
pub trait Create<TCreate, TResponse>
where
    TCreate: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned + Send,
    Self: WithApiClient + WithBasePath,
{
    /// Create a list of resources.
    ///
    /// # Arguments
    ///
    /// `creates` - List of resources to create.
    async fn create(&self, creates: &[TCreate]) -> Result<Vec<TResponse>> {
        let items = Items::from(creates);
        let response: ItemsWithoutCursor<TResponse> =
            self.get_client().post(Self::BASE_PATH, &items).await?;
        Ok(response.items)
    }

    /// Create a list of resources, converting from a different type.
    ///
    /// # Arguments
    ///
    /// * `creates` - List of resources to create.
    async fn create_from<'a>(
        &self,
        creates: &'a [impl Into<TCreate> + Sync + Clone],
    ) -> Result<Vec<TResponse>> {
        let to_add: Vec<TCreate> = creates.iter().map(|i| i.clone().into()).collect();
        self.create(&to_add).await
    }

    /// Create a list of resources, ignoring any that fail with general "conflict" errors.
    ///
    /// # Arguments
    ///
    /// * `creates` - List of resources to create.
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

    /// Create a list of resources, converting from a different type, and ignoring any that fail
    /// with general "conflict" errors.
    ///
    /// # Arguments
    ///
    /// * `creates` - List of resources to create.
    async fn create_from_ignore_duplicates<T: 'a, 'a>(
        &self,
        creates: &'a [impl Into<TCreate> + Sync + Clone],
    ) -> Result<Vec<TResponse>>
    where
        TCreate: EqIdentity,
    {
        let to_add: Vec<TCreate> = creates.iter().map(|i| i.clone().into()).collect();
        self.create_ignore_duplicates(&to_add).await
    }
}

#[async_trait]
/// Trait for upserts of resources that support both Create and Update.
pub trait Upsert<TCreate, TUpdate, TResponse, 'a>
where
    TCreate: Serialize + Sync + Send + EqIdentity + 'a + Clone + IntoPatch<TUpdate>,
    TUpdate: Serialize + Sync + Send + Default,
    TResponse: Serialize + DeserializeOwned + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    /// Upsert a list resources, meaning that they will first be attempted created,
    /// and if that fails with a conflict, update any that already existed, and create
    /// the remainder.
    ///
    /// # Arguments
    ///
    /// * `upserts` - Resources to insert or update.
    async fn upsert(
        &'a self,
        upserts: &'a [TCreate],
        ignore_nulls: bool,
    ) -> Result<Vec<TResponse>> {
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
                        update: it.clone().patch(ignore_nulls),
                    });
                } else {
                    to_create.push(it);
                }
            }

            let mut result = Vec::with_capacity(to_create.len() + to_update.len());
            if !to_create.is_empty() {
                let mut create_response: ItemsWithoutCursor<TResponse> = self
                    .get_client()
                    .post(Self::BASE_PATH, &Items::from(to_create))
                    .await?;
                result.append(&mut create_response.items);
            }
            if !to_update.is_empty() {
                let mut update_response: ItemsWithoutCursor<TResponse> = self
                    .get_client()
                    .post(
                        &format!("{}/update", Self::BASE_PATH),
                        &Items::from(&to_update),
                    )
                    .await?;
                result.append(&mut update_response.items);
            }

            Ok(result)
        } else {
            resp.map(|i| i.items)
        }
    }
}

impl<'a, T, TCreate, TUpdate, TResponse> Upsert<'a, TCreate, TUpdate, TResponse> for T
where
    T: Create<TCreate, TResponse> + Update<Patch<TUpdate>, TResponse>,
    TCreate: Serialize + Sync + Send + EqIdentity + 'a + Clone + IntoPatch<TUpdate>,
    TUpdate: Serialize + Sync + Send + Default,
    TResponse: Serialize + DeserializeOwned + Sync + Send,
{
}

#[async_trait]
/// Trait for resource types that support upserts directly.
pub trait UpsertCollection<TUpsert, TResponse> {
    /// Upsert a list of resources.
    ///
    /// # Arguments
    ///
    /// * `collection` - Items to insert or update.
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
/// Trait for resource types that can be deleted with a list of `TIdt`.
pub trait Delete<TIdt>
where
    TIdt: Serialize + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    /// Delete a list of resources by ID.
    ///
    /// # Arguments
    ///
    /// * `deletes` - IDs of items to delete.
    async fn delete(&self, deletes: &[TIdt]) -> Result<()> {
        let items = Items::from(deletes);
        self.get_client()
            .post::<::serde_json::Value, Items<&[TIdt]>>(
                &format!("{}/delete", Self::BASE_PATH),
                &items,
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
/// Trait for resource types that can be deleted with a more complex request.
pub trait DeleteWithRequest<TReq>
where
    TReq: Serialize + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    /// Delete resources using `req`.
    ///
    /// # Arguments
    ///
    /// * `req` - Request describing items to delete.
    async fn delete(&self, req: &TReq) -> Result<()> {
        self.get_client()
            .post::<::serde_json::Value, TReq>(&format!("{}/delete", Self::BASE_PATH), req)
            .await?;
        Ok(())
    }
}

#[async_trait]
/// Trait for resource types that can be deleted with a list of identities and
/// a boolean option to ignore unknown ids.
pub trait DeleteWithIgnoreUnknownIds<TIdt>
where
    TIdt: Serialize + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    /// Delete a list of resources, optionally ignore unknown ids.
    ///
    /// # Arguments
    ///
    /// * `deletes` - IDs of items to delete.
    /// * `ignore_unknown_ids` - If `true`, missing IDs will be ignored, and not
    /// cause the request to fail.
    async fn delete(&self, deletes: &[TIdt], ignore_unknown_ids: bool) -> Result<()> {
        let mut req = ItemsWithIgnoreUnknownIds::from(deletes);
        req.ignore_unknown_ids = ignore_unknown_ids;
        self.get_client()
            .post::<::serde_json::Value, _>(&format!("{}/delete", Self::BASE_PATH), &req)
            .await?;
        Ok(())
    }
}

#[async_trait]
/// Trait for resource types that can be deleted, and where the delete request
/// has a non-empty response.
pub trait DeleteWithResponse<TIdt, TResponse>
where
    TIdt: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned + Sync + Send,
    Self: WithApiClient + WithBasePath,
{
    /// Delete a list of resources.
    ///
    /// # Arguments
    ///
    /// * `deletes` - IDs of items to delete.
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
/// Trait for resource types that can be patch updated.
pub trait Update<TUpdate, TResponse>
where
    TUpdate: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    /// Update a list of resources.
    ///
    /// # Arguments
    ///
    /// * `updates` - Items to update.
    async fn update(&self, updates: &[TUpdate]) -> Result<Vec<TResponse>> {
        let items = Items::from(updates);
        let response: ItemsWithoutCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/update", Self::BASE_PATH), &items)
            .await?;
        Ok(response.items)
    }

    /// Update a list of resources by converting to the update from a different type.
    ///
    /// # Arguments
    ///
    /// * `updates` - Items to update.
    async fn update_from<T: 'a, 'a>(&self, updates: &'a [T]) -> Result<Vec<TResponse>>
    where
        T: std::marker::Sync + Clone,
        TUpdate: From<T>,
    {
        let to_update: Vec<TUpdate> = updates.iter().map(|i| TUpdate::from(i.clone())).collect();
        self.update(&to_update).await
    }

    /// Update a list of resources, ignoring any that fail due to items missing in CDF.
    ///
    /// # Arguments
    ///
    /// * `updates` - Items to update.
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

    /// Update a list of resources by converting from a different type, ignoring any that fail
    /// due items missing in CDF.
    ///
    /// # Arguments
    ///
    /// * `updates` - Items to update.
    async fn update_from_ignore_unknown_ids<T: 'a, 'a>(
        &self,
        updates: &'a [T],
    ) -> Result<Vec<TResponse>>
    where
        T: std::marker::Sync + Clone,
        TUpdate: From<T> + EqIdentity,
        TResponse: Send,
    {
        let to_update: Vec<TUpdate> = updates.iter().map(|i| TUpdate::from(i.clone())).collect();
        self.update_ignore_unknown_ids(&to_update).await
    }
}

#[async_trait]
/// Trait for retrieving items from CDF by id.
pub trait Retrieve<TIdt, TResponse>
where
    TIdt: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    /// Retrieve a list of items from CDF by id.
    ///
    /// # Arguments
    ///
    /// * `ids` - IDs of items to retrieve.
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
/// Trait for retrieving items from CDF with a more complex request type.
pub trait RetrieveWithRequest<TRequest, TResponse>
where
    TRequest: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    /// Retrieve items from CDF with a more complex request.
    ///
    /// # Arguments
    ///
    /// * `req` - Request describing items to retrieve.
    async fn retrieve(&self, req: &TRequest) -> Result<TResponse> {
        let response: TResponse = self
            .get_client()
            .post(&format!("{}/byids", Self::BASE_PATH), req)
            .await?;
        Ok(response)
    }
}

#[async_trait]
/// Trait for retrieving items from CDF with an option to ignore unknown IDs.
pub trait RetrieveWithIgnoreUnknownIds<TIdt, TResponse>
where
    TIdt: Serialize + Sync + Send,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    /// Retrieve a list of items from CDF. If ignore_unknown_ids is false,
    /// this will fail if any items are missing from CDF.
    ///
    /// # Arguments
    ///
    /// * `ids` - IDs of items to retrieve.
    /// * `ignore_unknown_ids` - If `true`, items missing from CDF will be ignored, and not
    /// cause the request to fail.
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
/// Trait for resource types that allow filtering with a simple filter.
pub trait FilterItems<TFilter, TResponse>
where
    TFilter: Serialize + Sync + Send + 'static,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    /// Filter resources using a simple filter.
    /// The response may contain a cursor that can be used to paginate results.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter which items to retrieve.
    /// * `cursor` - Optional cursor for pagination.
    /// * `limit` - Maximum number of result items.
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

#[derive(Debug, Default)]
enum CursorState {
    Initial,
    Some(String),
    #[default]
    End,
}

struct CursorStreamState<TFilter, TResponse> {
    req: TFilter,
    responses: VecDeque<TResponse>,
    next_cursor: CursorState,
}

#[async_trait]
/// Trait for resource types that allow filtering with a more complex request.
pub trait FilterWithRequest<TFilter, TResponse>
where
    TFilter: Serialize + Sync + Send + 'static,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    /// Filter resources.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter which items to retrieve.
    async fn filter(&self, filter: TFilter) -> Result<ItemsWithCursor<TResponse>> {
        let response: ItemsWithCursor<TResponse> = self
            .get_client()
            .post(&format!("{}/list", Self::BASE_PATH), &filter)
            .await?;
        Ok(response)
    }

    /// Filter resources, following cursors until they are exhausted.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter which items to retrieve.
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

    /// Filter resources, following cursors. This returns a stream, you can abort the stream whenever you
    /// want and only resources retrieved up to that point will be returned.
    ///
    /// Each item in the stream will be a result, after the first error is returned the
    /// stream will end.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter which items to retrieve.
    fn filter_all_stream<'a>(
        &'a self,
        filter: TFilter,
    ) -> Pin<Box<dyn TryStream<Ok = TResponse, Error = crate::Error, Item = Result<TResponse>> + 'a>>
    where
        TFilter: SetCursor,
        TResponse: Send + 'static,
    {
        let state = CursorStreamState {
            req: filter,
            responses: VecDeque::new(),
            next_cursor: CursorState::Initial,
        };

        Box::pin(try_unfold(state, move |mut state| async move {
            if let Some(next) = state.responses.pop_front() {
                Ok(Some((next, state)))
            } else {
                let cursor = match std::mem::take(&mut state.next_cursor) {
                    CursorState::Initial => None,
                    CursorState::Some(x) => Some(x),
                    CursorState::End => {
                        return Ok(None);
                    }
                };
                state.req.set_cursor(cursor);
                let response: ItemsWithCursor<TResponse> = self
                    .get_client()
                    .post(&format!("{}/list", Self::BASE_PATH), &state.req)
                    .await?;

                state.responses.extend(response.items);
                state.next_cursor = match response.next_cursor {
                    Some(x) => CursorState::Some(x),
                    None => CursorState::End,
                };
                if let Some(next) = state.responses.pop_front() {
                    Ok(Some((next, state)))
                } else {
                    Ok(None)
                }
            }
        }))
    }

    /// Filter resources using partitioned reads, following cursors until all partitions are
    /// exhausted.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter which items to retrieve.
    /// * `num_partitions` - Number of partitions.
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
/// Trait for resource types that allow filtering with fuzzy search.
pub trait SearchItems<TFilter, TSearch, TResponse, 'a>
where
    TFilter: Serialize + Sync + Send + 'a,
    TSearch: Serialize + Sync + Send + 'a,
    TResponse: Serialize + DeserializeOwned,
    Self: WithApiClient + WithBasePath,
{
    /// Fuzzy search resources.
    ///
    /// # Arguments
    ///
    /// * `filter` - Simple filter applied to items.
    /// * `search` - Fuzzy search.
    /// * `limit` - Maximum number of items to retrieve.
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
