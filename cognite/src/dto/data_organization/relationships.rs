use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::time_series::TimeSeries;
use crate::{
    CogniteExternalId, Identity, IntoPatch, IntoPatchItem, LabelsFilter, Partition, Patch, Range,
    SetCursor, UpdateList, UpdateSet, UpdateSetNull, WithPartition,
};

use crate::{assets::Asset, events::Event, files::FileMetadata};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipVertexType {
    #[default]
    Asset,
    TimeSeries,
    File,
    Event,
    Sequence,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum RelationshipVertex {
    Asset(Asset),
    TimeSeries(TimeSeries),
    File(FileMetadata),
    Event(Event),
    Sequence(()),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub external_id: String,
    pub source_external_id: String,
    pub source_type: RelationshipVertexType,
    pub target_external_id: String,
    pub target_type: RelationshipVertexType,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub confidence: Option<f32>,
    pub data_set_id: Option<i64>,
    pub labels: Option<Vec<CogniteExternalId>>,
    pub created_time: Option<i64>,
    pub last_updated_time: Option<i64>,
    pub source: Option<RelationshipVertex>,
    pub target: Option<RelationshipVertex>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddRelationship {
    pub external_id: String,
    pub source_external_id: String,
    pub source_type: RelationshipVertexType,
    pub target_external_id: String,
    pub target_type: RelationshipVertexType,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub confidence: Option<f32>,
    pub data_set_id: Option<i64>,
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
pub struct PatchRelationship {
    pub source_type: Option<UpdateSet<RelationshipVertexType>>,
    pub source_external_id: Option<UpdateSet<String>>,
    pub target_type: Option<UpdateSet<RelationshipVertexType>>,
    pub target_external_id: Option<UpdateSet<String>>,
    pub confidence: Option<UpdateSetNull<f32>>,
    pub start_time: Option<UpdateSetNull<i64>>,
    pub end_time: Option<UpdateSetNull<i64>>,
    pub data_set_id: Option<UpdateSetNull<i64>>,
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
}

impl IntoPatch<Patch<PatchRelationship>> for Relationship {
    fn patch(self, ignore_nulls: bool) -> Patch<PatchRelationship> {
        Patch::<PatchRelationship> {
            id: Identity::ExternalId {
                external_id: self.external_id,
            },
            update: PatchRelationship {
                source_type: self.source_type.patch(ignore_nulls),
                source_external_id: self.source_external_id.patch(ignore_nulls),
                target_type: self.target_type.patch(ignore_nulls),
                target_external_id: self.target_external_id.patch(ignore_nulls),
                confidence: self.confidence.patch(ignore_nulls),
                start_time: self.start_time.patch(ignore_nulls),
                end_time: self.end_time.patch(ignore_nulls),
                data_set_id: self.data_set_id.patch(ignore_nulls),
                labels: self.labels.patch(ignore_nulls),
            },
        }
    }
}

impl IntoPatch<PatchRelationship> for AddRelationship {
    fn patch(self, ignore_nulls: bool) -> PatchRelationship {
        PatchRelationship {
            source_type: self.source_type.patch(ignore_nulls),
            source_external_id: self.source_external_id.patch(ignore_nulls),
            target_type: self.target_type.patch(ignore_nulls),
            target_external_id: self.target_external_id.patch(ignore_nulls),
            confidence: self.confidence.patch(ignore_nulls),
            start_time: self.start_time.patch(ignore_nulls),
            end_time: self.end_time.patch(ignore_nulls),
            data_set_id: self.data_set_id.patch(ignore_nulls),
            labels: self.labels.patch(ignore_nulls),
        }
    }
}

impl From<Relationship> for Patch<PatchRelationship> {
    fn from(rel: Relationship) -> Self {
        IntoPatch::<Patch<PatchRelationship>>::patch(rel, false)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveRelationshipsRequest {
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
pub struct SourceOrTargetFilter {
    pub r#type: RelationshipVertexType,
    pub external_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipsFilter {
    pub source_external_ids: Option<Vec<String>>,
    pub source_types: Option<Vec<RelationshipVertexType>>,
    pub target_external_ids: Option<Vec<String>>,
    pub target_types: Option<Vec<RelationshipVertexType>>,
    pub data_set_ids: Option<Vec<Identity>>,
    pub start_time: Option<Range<i64>>,
    pub end_time: Option<Range<i64>>,
    pub confidence: Option<Range<f64>>,
    pub last_updated_time: Option<Range<i64>>,
    pub created_time: Option<Range<i64>>,
    pub active_at_time: Option<Range<i64>>,
    pub sources_or_targets: Option<Vec<SourceOrTargetFilter>>,
    pub labels: Option<LabelsFilter>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilterRelationshipsQuery {
    pub filter: RelationshipsFilter,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
    pub fetch_resources: Option<bool>,
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
