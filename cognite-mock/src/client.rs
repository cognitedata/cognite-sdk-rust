use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::resource::{
    MockAssetsResource, MockDataSetsResource, MockEventsResource, MockFilesResource,
    MockLabelsResource, MockRawResource, MockRelationshipsResource, MockTimeSeriesResource,
};
use crate::store::{
    AssetStore, DataSetStore, DatapointStore, EventStore, FileMetaStore, LabelStore, RawStore,
    RelationshipStore, TimeSeriesStore,
};

pub struct IdGen(AtomicI64);

impl IdGen {
    fn new() -> Self {
        Self(AtomicI64::new(1))
    }

    pub fn next(&self) -> i64 {
        self.0.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for IdGen {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MockClient {
    pub assets: RwLock<AssetStore>,
    pub events: RwLock<EventStore>,
    pub time_series: RwLock<TimeSeriesStore>,
    pub datapoints: RwLock<DatapointStore>,
    pub files: RwLock<FileMetaStore>,
    pub labels: RwLock<LabelStore>,
    pub raw: RwLock<RawStore>,
    pub relationships: RwLock<RelationshipStore>,
    pub data_sets: RwLock<DataSetStore>,
    pub id_gen: IdGen,
}

impl Default for MockClient {
    fn default() -> Self {
        Self {
            assets: RwLock::new(AssetStore::default()),
            events: RwLock::new(EventStore::default()),
            time_series: RwLock::new(TimeSeriesStore::default()),
            datapoints: RwLock::new(DatapointStore::default()),
            files: RwLock::new(FileMetaStore::default()),
            labels: RwLock::new(LabelStore::default()),
            raw: RwLock::new(RawStore::default()),
            relationships: RwLock::new(RelationshipStore::default()),
            data_sets: RwLock::new(DataSetStore::default()),
            id_gen: IdGen::new(),
        }
    }
}

pub struct MockCogniteClient {
    pub assets: MockAssetsResource,
    pub events: MockEventsResource,
    pub time_series: MockTimeSeriesResource,
    pub files: MockFilesResource,
    pub labels: MockLabelsResource,
    pub raw: MockRawResource,
    pub relationships: MockRelationshipsResource,
    pub data_sets: MockDataSetsResource,
    client: Arc<MockClient>,
}

impl MockCogniteClient {
    pub fn new() -> Self {
        let client = Arc::new(MockClient::default());
        Self::from_client(client)
    }

    fn from_client(client: Arc<MockClient>) -> Self {
        Self {
            assets: MockAssetsResource::new(client.clone()),
            events: MockEventsResource::new(client.clone()),
            time_series: MockTimeSeriesResource::new(client.clone()),
            files: MockFilesResource::new(client.clone()),
            labels: MockLabelsResource::new(client.clone()),
            raw: MockRawResource::new(client.clone()),
            relationships: MockRelationshipsResource::new(client.clone()),
            data_sets: MockDataSetsResource::new(client.clone()),
            client,
        }
    }

    /// Reset all stores, clearing all data.
    pub async fn reset(&self) {
        *self.client.assets.write().await = AssetStore::default();
        *self.client.events.write().await = EventStore::default();
        *self.client.time_series.write().await = TimeSeriesStore::default();
        *self.client.datapoints.write().await = DatapointStore::default();
        *self.client.files.write().await = FileMetaStore::default();
        *self.client.labels.write().await = LabelStore::default();
        *self.client.raw.write().await = RawStore::default();
        *self.client.relationships.write().await = RelationshipStore::default();
        *self.client.data_sets.write().await = DataSetStore::default();
    }
}

impl Default for MockCogniteClient {
    fn default() -> Self {
        Self::new()
    }
}
