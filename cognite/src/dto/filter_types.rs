use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::CogniteExternalId;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A range between two points.
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
/// A type wrapping a CDF filter, with cursor and limit.
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
/// A type warpping a CDf filter and search, with limit.
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
/// A wrapper around a partition, with custom serializer and deserializer
/// for converting to the [a]/[b] format used by CDF.
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

struct PartitionVisitor;

impl<'de> Visitor<'de> for PartitionVisitor {
    type Value = Partition;
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let pair = v.split('/').collect::<Vec<_>>();
        if pair.len() != 2 {
            Err(E::custom("Expect a string on the form N/M"))
        } else {
            let lh = pair[0];
            let rh = pair[1];

            let lh_v = lh
                .parse()
                .map_err(|_| E::custom("Expected a string on the form u32/u32"))?;
            let rh_v = rh
                .parse()
                .map_err(|_| E::custom("Expected a string on the form u32/u32"))?;

            Ok(Partition {
                num_partitions: lh_v,
                partition_number: rh_v,
            })
        }
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string on the form N/M")
    }
}

impl<'de> Deserialize<'de> for Partition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PartitionVisitor)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A wrapper around a filter, with cursor, limit, and partition.
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

/// Trait implemented by types with a cursor, to allow automatic pagination.
pub trait SetCursor {
    /// Set cursor to the given value.
    fn set_cursor(&mut self, cursor: Option<String>);
}

impl<T> SetCursor for PartitionedFilter<T> {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

/// Trait implemented by types with a partition, to allow automatic handling of partitions.
pub trait WithPartition {
    /// Create a clone of self with given partition.
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
