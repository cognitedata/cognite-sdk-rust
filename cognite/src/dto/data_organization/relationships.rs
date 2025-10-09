use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::sequences::Sequence;
use crate::time_series::TimeSeries;
use crate::{
    CogniteExternalId, Identity, IntoPatch, IntoPatchItem, LabelsFilter, Partition, Patch, Range,
    SetCursor, UpdateList, UpdateSet, UpdateSetNull, UpsertOptions, WithPartition,
};

use crate::{assets::Asset, events::Event, files::FileMetadata};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Vertex type for a relationship.
pub enum RelationshipVertexType {
    #[default]
    /// Reference an asset
    Asset,
    /// Reference a time series.
    TimeSeries,
    /// Reference a file
    File,
    /// Reference an event.
    Event,
    /// Reference a sequence.
    Sequence,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// The vertex of a relationship, with the referenced data.
pub enum RelationshipVertex {
    /// Asset vertex.
    Asset(Asset),
    /// Time series vertex.
    TimeSeries(TimeSeries),
    /// File vertex.
    File(FileMetadata),
    /// Event vertex.
    Event(Event),
    /// Sequence vertex.
    Sequence(Sequence),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A CDF relationship, defining a custom connection between two resources.
pub struct Relationship {
    /// Relationship external ID, must be unique within the project.
    pub external_id: String,
    /// External ID of the source resource.
    pub source_external_id: String,
    /// Type of the source resource.
    pub source_type: RelationshipVertexType,
    /// External ID of the target resource.
    pub target_external_id: String,
    /// Type of the target resource.
    pub target_type: RelationshipVertexType,
    /// Timestamp in milliseconds since epoch when this relationship becomes valid.
    pub start_time: Option<i64>,
    /// Timestamp in milliseconds since epoch when this relationship stops being valid.
    pub end_time: Option<i64>,
    /// Confidence value of the existence of the relationship, between 0.0 and 1.0.
    /// Generated relationships provide a score of the likelihood of
    /// the relationship existing. Relationships without a confidence value
    /// can be interpreted at the discretion of each project.
    pub confidence: Option<f32>,
    /// Data set this relationship belongs to.
    pub data_set_id: Option<i64>,
    /// List of labels on this relationship.
    pub labels: Option<Vec<CogniteExternalId>>,
    /// Time this relationship was created, in milliseconds since epoch.
    pub created_time: Option<i64>,
    /// Time this relationship was last modified, in milliseconds since epoch.
    pub last_updated_time: Option<i64>,
    /// Source of this relationship, if requested.
    pub source: Option<RelationshipVertex>,
    /// Target of this relationship, if requested.
    pub target: Option<RelationshipVertex>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Create a new relationship.
pub struct AddRelationship {
    /// Relationship external ID, must be unique within the project.
    pub external_id: String,
    /// External ID of the source resource.
    pub source_external_id: String,
    /// Type of the source resource.
    pub source_type: RelationshipVertexType,
    /// External ID of the target resource.
    pub target_external_id: String,
    /// Type of the target resource.
    pub target_type: RelationshipVertexType,
    /// Timestamp in milliseconds since epoch when this relationship becomes valid.
    pub start_time: Option<i64>,
    /// Timestamp in milliseconds since epoch when this relationship stops being valid.
    pub end_time: Option<i64>,
    /// Confidence value of the existence of the relationship, between 0.0 and 1.0.
    /// Generated relationships provide a score of the likelihood of
    /// the relationship existing. Relationships without a confidence value
    /// can be interpreted at the discretion of each project.
    pub confidence: Option<f32>,
    /// Data set this relationship belongs to.
    pub data_set_id: Option<i64>,
    /// List of labels on this relationship.
    pub labels: Option<Vec<CogniteExternalId>>,
}

impl From<Relationship> for AddRelationship {
    fn from(rel: Relationship) -> Self {
        AddRelationship {
            external_id: rel.external_id,
            source_external_id: rel.source_external_id,
            source_type: rel.source_type,
            target_external_id: rel.target_external_id,
            target_type: rel.target_type,
            start_time: rel.start_time,
            end_time: rel.end_time,
            confidence: rel.confidence,
            data_set_id: rel.data_set_id,
            labels: rel.labels,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Update a relationship
pub struct PatchRelationship {
    /// Type of the source resource.
    pub source_type: Option<UpdateSet<RelationshipVertexType>>,
    /// External ID of the source resource.
    pub source_external_id: Option<UpdateSet<String>>,
    /// Type of the target resource.
    pub target_type: Option<UpdateSet<RelationshipVertexType>>,
    /// External ID of the target resource.
    pub target_external_id: Option<UpdateSet<String>>,
    /// Confidence value of the existence of the relationship, between 0.0 and 1.0.
    /// Generated relationships provide a score of the likelihood of
    /// the relationship existing. Relationships without a confidence value
    /// can be interpreted at the discretion of each project.
    pub confidence: Option<UpdateSetNull<f32>>,
    /// Timestamp in milliseconds since epoch when this relationship becomes valid.
    pub start_time: Option<UpdateSetNull<i64>>,
    /// Timestamp in milliseconds since epoch when this relationship stops being valid.
    pub end_time: Option<UpdateSetNull<i64>>,
    /// Data set this relationship belongs to.
    pub data_set_id: Option<UpdateSetNull<i64>>,
    /// List of labels on this relationship.
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
}

impl IntoPatch<Patch<PatchRelationship>> for Relationship {
    fn patch(self, options: &UpsertOptions) -> Patch<PatchRelationship> {
        Patch::<PatchRelationship> {
            id: Identity::ExternalId {
                external_id: self.external_id,
            },
            update: PatchRelationship {
                source_type: self.source_type.patch(options),
                source_external_id: self.source_external_id.patch(options),
                target_type: self.target_type.patch(options),
                target_external_id: self.target_external_id.patch(options),
                confidence: self.confidence.patch(options),
                start_time: self.start_time.patch(options),
                end_time: self.end_time.patch(options),
                data_set_id: self.data_set_id.patch(options),
                labels: self.labels.patch(options),
            },
        }
    }
}

impl IntoPatch<PatchRelationship> for AddRelationship {
    fn patch(self, options: &UpsertOptions) -> PatchRelationship {
        PatchRelationship {
            source_type: self.source_type.patch(options),
            source_external_id: self.source_external_id.patch(options),
            target_type: self.target_type.patch(options),
            target_external_id: self.target_external_id.patch(options),
            confidence: self.confidence.patch(options),
            start_time: self.start_time.patch(options),
            end_time: self.end_time.patch(options),
            data_set_id: self.data_set_id.patch(options),
            labels: self.labels.patch(options),
        }
    }
}

impl From<Relationship> for Patch<PatchRelationship> {
    fn from(rel: Relationship) -> Self {
        IntoPatch::<Patch<PatchRelationship>>::patch(rel, &Default::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetrieveRelationshipsRequest {
    pub items: ::serde_json::Value,
    pub ignore_unknown_ids: bool,
    pub fetch_resources: bool,
}

impl<T: Serialize> From<&Vec<T>> for RetrieveRelationshipsRequest {
    fn from(items: &Vec<T>) -> RetrieveRelationshipsRequest {
        RetrieveRelationshipsRequest {
            items: json!(items),
            ignore_unknown_ids: true,
            fetch_resources: false,
        }
    }
}

impl<T: Serialize> From<&[T]> for RetrieveRelationshipsRequest {
    fn from(items: &[T]) -> RetrieveRelationshipsRequest {
        RetrieveRelationshipsRequest {
            items: json!(items),
            ignore_unknown_ids: true,
            fetch_resources: false,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter on a relationship source or target.
pub struct SourceOrTargetFilter {
    /// Relationship vertex type.
    pub r#type: RelationshipVertexType,
    /// External ID of the resource.
    pub external_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter relationships.
pub struct RelationshipsFilter {
    /// Include relationships that have any of these values as their `source_external_id`.
    pub source_external_ids: Option<Vec<String>>,
    /// Include relationships that has any of these values as their `source_type`.
    pub source_types: Option<Vec<RelationshipVertexType>>,
    /// Include relationships that have any of these values as their `target_external_id`.
    pub target_external_ids: Option<Vec<String>>,
    /// Include relationships that has any of these values as their `target_type`.
    pub target_types: Option<Vec<RelationshipVertexType>>,
    /// Include relationships that belongs to any of these data sets.
    pub data_set_ids: Option<Vec<Identity>>,
    /// Range of timestamps for `start_time`.
    pub start_time: Option<Range<i64>>,
    /// Range of timestamps for `end_time`.
    pub end_time: Option<Range<i64>>,
    /// Range of values for `confidence`.
    pub confidence: Option<Range<f64>>,
    /// Range of timestamps for `last_updated_time`.
    pub last_updated_time: Option<Range<i64>>,
    /// Range of timestamps for `created_time`.
    pub created_time: Option<Range<i64>>,
    /// Limits results to those active within the specified time range, that is,
    /// if there is any overlap in the intervals `[activeAtTime.min, activeAtTime.max]`
    /// and `[startTime, endTime]`, where both intervals are inclusive.
    /// If a relationship does not have a startTime, it is regarded as active
    /// from the beginning of time by this filter. If it does not have an `endTime`
    /// is is regarded as active until the end of time. Similarly,
    /// if a min is not supplied to the filter, the min is implicitly
    /// set to the beginning of time. If a max is not supplied, the max is
    /// implicitly set to the end of time.
    pub active_at_time: Option<Range<i64>>,
    /// Include relationships that match any of the resources in either their source-
    /// or target-related fields.
    pub sources_or_targets: Option<Vec<SourceOrTargetFilter>>,
    /// Include relationships that match these label constraints.
    pub labels: Option<LabelsFilter>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Query for filtering relationships.
pub struct FilterRelationshipsQuery {
    /// Relationship filter.
    pub filter: RelationshipsFilter,
    /// Maximum number of relationships to return, default 100, maximum 1000.
    pub limit: Option<u32>,
    /// Cursor for pagination.
    pub cursor: Option<String>,
    /// Whether to fetch the associated resources.
    pub fetch_resources: Option<bool>,
    /// Split the data set into partitions.
    pub partition: Option<Partition>,
}

impl SetCursor for FilterRelationshipsQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

impl WithPartition for FilterRelationshipsQuery {
    fn with_partition(&self, partition: Partition) -> Self {
        Self {
            filter: self.filter.clone(),
            limit: self.limit,
            cursor: None,
            fetch_resources: self.fetch_resources,
            partition: Some(partition),
        }
    }
}
