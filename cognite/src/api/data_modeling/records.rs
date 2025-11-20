use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    models::{
        records::{
            CursorAndHasNext, Record, RecordWrite, RecordsRetrieveRequest, RecordsSyncRequest,
        },
        ItemId,
    },
    Items, ItemsVec, RawValue, Resource, Result,
};

pub type RecordsResource = Resource<Record<HashMap<String, RawValue>>>;

impl RecordsResource {
    /// Ingest records into a stream.
    ///
    /// Note: The maximum total request size is 10 MB.
    ///
    /// # Arguments
    ///
    /// * `stream_id` - ID of the stream to ingest records into.
    /// * `records` - Records to ingest.
    pub async fn ingest<T: Serialize>(
        &self,
        stream_id: &str,
        records: &[RecordWrite<T>],
    ) -> Result<()> {
        self.api_client
            .post::<serde_json::Value, _>(
                &format!("streams/{stream_id}/records"),
                &Items::new(records),
            )
            .await?;
        Ok(())
    }

    /// Upsert records into a stream.
    ///
    /// Note: The maximum total request size is 10 MB.
    ///
    /// # Arguments
    ///
    /// * `stream_id` - ID of the stream to ingest records into.
    /// * `records` - Records to ingest.
    pub async fn upsert<T: Serialize>(
        &self,
        stream_id: &str,
        records: &[RecordWrite<T>],
    ) -> Result<()> {
        self.api_client
            .post::<serde_json::Value, _>(
                &format!("streams/{stream_id}/records/upsert"),
                &Items::new(records),
            )
            .await?;
        Ok(())
    }

    /// Retrieve records from a stream.
    ///
    /// # Arguments
    ///
    /// * `stream_id` - ID of the stream to retrieve records from.
    /// * `request` - Request with optional filter and sort.
    pub async fn retrieve<T: DeserializeOwned>(
        &self,
        stream_id: &str,
        request: &RecordsRetrieveRequest,
    ) -> Result<ItemsVec<Record<T>>> {
        self.api_client
            .post(&format!("streams/{stream_id}/records/filter"), request)
            .await
    }

    /// Subscribe to changes for records from the stream, matching a supplied filter.
    ///
    /// # Arguments
    ///
    /// * `stream_id` - ID of the stream to subscribe to.
    /// * `request` - Request with optional filter.
    pub async fn sync<T: DeserializeOwned>(
        &self,
        stream_id: &str,
        request: &RecordsSyncRequest,
    ) -> Result<ItemsVec<Record<T>, CursorAndHasNext>> {
        self.api_client
            .post(&format!("streams/{stream_id}/records/sync"), request)
            .await
    }

    /// Delete records from a stream.
    ///
    /// # Arguments
    ///
    /// * `stream_id` - ID of the stream to delete records from.
    /// * `items` - IDs of the records to delete.
    pub async fn delete(&self, stream_id: &str, items: &[ItemId]) -> Result<()> {
        self.api_client
            .post_request(&format!("streams/{stream_id}/records/delete"))
            .json(&Items::new(items))?
            .accept_nothing()
            .send()
            .await?;
        Ok(())
    }
}
