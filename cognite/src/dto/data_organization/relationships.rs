use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    CogniteExternalId, Identity, LabelsFilter, Partition, Patch, Range, SetCursor, UpdateList,
    UpdateSet, UpdateSetNull, WithPartition,
};

use crate::{assets::Asset, events::Event, files::FileMetadata, time_series::TimeSerie};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum RelationshipVertexType {
    #[default]
    Asset,
    TimeSeries,
    File,
    Event,
    Sequence,
}

// Want a default impl for AddRelationship, so we need a default value here
// not ideal, really...

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum RelationshipVertex {
    Asset(Asset),
    TimeSeries(TimeSerie),
    File(FileMetadata),
    Event(Event),
    Sequence(()),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub external_id: String,
    pub source_external_id: String,
    pub source_type: RelationshipVertexType,
    pub target_external_id: String,
    pub target_type: RelationshipVertexType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<CogniteExternalId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RelationshipVertex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<RelationshipVertex>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddRelationship {
    pub external_id: String,
    pub source_external_id: String,
    pub source_type: RelationshipVertexType,
    pub target_external_id: String,
    pub target_type: RelationshipVertexType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<CogniteExternalId>>,
}

impl From<&Relationship> for AddRelationship {
    fn from(rel: &Relationship) -> Self {
        AddRelationship {
            external_id: rel.external_id.clone(),
            source_external_id: rel.source_external_id.clone(),
            source_type: rel.source_type,
            target_external_id: rel.target_external_id.clone(),
            target_type: rel.target_type,
            start_time: rel.start_time,
            end_time: rel.end_time,
            confidence: rel.confidence,
            data_set_id: rel.data_set_id,
            labels: rel.labels.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchRelationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<UpdateSet<RelationshipVertexType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_external_id: Option<UpdateSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<UpdateSet<RelationshipVertexType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_external_id: Option<UpdateSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<UpdateSetNull<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<UpdateSetNull<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<UpdateList<CogniteExternalId, CogniteExternalId>>,
}

impl From<&Relationship> for Patch<PatchRelationship> {
    fn from(rel: &Relationship) -> Self {
        Patch::<PatchRelationship> {
            id: Identity::ExternalId {
                external_id: rel.external_id.clone(),
            },
            update: PatchRelationship {
                source_type: Some(rel.source_type.into()),
                source_external_id: Some(rel.source_external_id.clone().into()),
                target_type: Some(rel.target_type.into()),
                target_external_id: Some(rel.target_external_id.clone().into()),
                confidence: Some(rel.confidence.into()),
                start_time: Some(rel.start_time.into()),
                end_time: Some(rel.end_time.into()),
                data_set_id: Some(rel.data_set_id.into()),
                labels: Some(rel.labels.clone().into()),
            },
        }
    }
}

impl From<&AddRelationship> for PatchRelationship {
    fn from(rel: &AddRelationship) -> Self {
        PatchRelationship {
            source_type: Some(rel.source_type.into()),
            source_external_id: Some(rel.source_external_id.clone().into()),
            target_type: Some(rel.target_type.into()),
            target_external_id: Some(rel.target_external_id.clone().into()),
            confidence: Some(rel.confidence.into()),
            start_time: Some(rel.start_time.into()),
            end_time: Some(rel.end_time.into()),
            data_set_id: Some(rel.data_set_id.into()),
            labels: Some(rel.labels.clone().into()),
        }
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

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipsFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_external_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_types: Option<Vec<RelationshipVertexType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_external_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_types: Option<Vec<RelationshipVertexType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_ids: Option<Vec<Identity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Range<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_at_time: Option<Range<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_or_targets: Option<Vec<SourceOrTargetFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LabelsFilter>,
}

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilterRelationshipsQuery {
    pub filter: RelationshipsFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_resources: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
