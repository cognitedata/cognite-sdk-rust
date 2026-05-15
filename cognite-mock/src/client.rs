use std::sync::atomic::{AtomicI64, Ordering};

use tokio::sync::RwLock;

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
