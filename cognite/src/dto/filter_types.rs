use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::CogniteExternalId;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Range<T> {
    pub max: Option<T>,
    pub min: Option<T>,
}

impl<T> Range<T> {
    pub fn new(min: Option<T>, max: Option<T>) -> Range<T> {
        Range::<T> { min, max }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Filter<T> {
    pub filter: T,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

impl<T> Filter<T> {
    pub fn new(filter: T, cursor: Option<String>, limit: Option<u32>) -> Filter<T> {
        Filter {
            filter,
            cursor,
            limit,
        }
    }
}

impl<T> SetCursor for Filter<T> {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Search<TFilter, TSearch> {
    pub filter: TFilter,
    pub search: TSearch,
    pub limit: Option<u32>,
}

impl<TFilter, TSearch> Search<TFilter, TSearch> {
    pub fn new(filter: TFilter, search: TSearch, limit: Option<u32>) -> Search<TFilter, TSearch> {
        Search {
            filter,
            search,
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LabelsFilter {
    ContainsAny(Vec<CogniteExternalId>),
    ContainsAll(Vec<CogniteExternalId>),
}

#[derive(Debug, Clone)]
pub struct Partition {
    pub num_partitions: u32,
    pub partition_number: u32,
}

impl Partition {
    pub fn new(partition_number: u32, num_partitions: u32) -> Self {
        Self {
            partition_number,
            num_partitions,
        }
    }
}

impl Display for Partition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.partition_number, self.num_partitions)
    }
}

impl Serialize for Partition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartitionedFilter<T> {
    pub filter: T,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub partition: Option<Partition>,
}

impl<T> PartitionedFilter<T> {
    pub fn new(
        filter: T,
        cursor: Option<String>,
        limit: Option<u32>,
        partition: Option<Partition>,
    ) -> PartitionedFilter<T> {
        PartitionedFilter {
            filter,
            cursor,
            limit,
            partition,
        }
    }
}

pub trait SetCursor {
    fn set_cursor(&mut self, cursor: Option<String>);
}

impl<T> SetCursor for PartitionedFilter<T> {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

pub trait WithPartition {
    fn with_partition(&self, partition: Partition) -> Self;
}

impl<T> WithPartition for PartitionedFilter<T>
where
    T: Clone,
{
    fn with_partition(&self, partition: Partition) -> Self {
        Self {
            filter: self.filter.clone(),
            cursor: None,
            limit: self.limit,
            partition: Some(partition),
        }
    }
}
