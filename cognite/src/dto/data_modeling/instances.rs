use std::collections::HashMap;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    FdmFilter, ItemId, RawValue, SourceReference, TaggedViewReference, ViewCorePropertyType,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeAndEdgeCreateCollection<TProperties> {
    pub items: Vec<NodeOrEdgeCreate<TProperties>>,
    pub auto_create_direct_relations: Option<bool>,
    pub auto_create_start_nodes: Option<bool>,
    pub auto_create_end_nodes: Option<bool>,
    pub skip_on_version_conflict: Option<bool>,
    pub replace: Option<bool>,
}

impl<TProperties> Default for NodeAndEdgeCreateCollection<TProperties> {
    fn default() -> Self {
        Self {
            items: vec![],
            auto_create_direct_relations: None,
            auto_create_start_nodes: None,
            auto_create_end_nodes: None,
            skip_on_version_conflict: None,
            replace: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "instanceType")]
pub enum NodeOrEdgeCreate<TProperties> {
    Node(NodeWrite<TProperties>),
    Edge(EdgeWrite<TProperties>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeWrite<TProperties> {
    pub space: String,
    pub external_id: String,
    pub sources: Option<Vec<EdgeOrNodeData<TProperties>>>,
    pub existing_version: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EdgeWrite<TProperties> {
    pub space: String,
    pub r#type: DirectRelationReference,
    pub external_id: String,
    pub start_node: DirectRelationReference,
    pub end_node: DirectRelationReference,
    pub sources: Option<Vec<EdgeOrNodeData<TProperties>>>,
    pub existing_version: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EdgeOrNodeData<TProperties> {
    pub source: SourceReference,
    pub properties: TProperties,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "instanceType")]
pub enum SlimNodeOrEdge {
    Node(SlimNodeDefinition),
    Edge(SlimEdgeDefinition),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimNodeDefinition {
    pub space: String,
    pub version: i32,
    pub was_modified: bool,
    pub external_id: String,
    pub created_time: Option<i64>,
    pub last_updated_time: Option<i64>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimEdgeDefinition {
    pub space: String,
    pub version: i32,
    pub was_modified: bool,
    pub external_id: String,
    pub created_time: Option<i64>,
    pub last_updated_time: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "instanceType")]
pub enum NodeOrEdge<TProperties> {
    Node(NodeDefinition<TProperties>),
    Edge(EdgeDefinition<TProperties>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeDefinition<TProperties> {
    pub space: String,
    pub version: i32,
    pub external_id: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub deleted_time: Option<i64>,
    pub properties: Option<PropertiesObject<TProperties>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EdgeDefinition<TProperties> {
    pub space: String,
    pub r#type: DirectRelationReference,
    pub version: String,
    pub external_id: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub start_node: DirectRelationReference,
    pub end_node: DirectRelationReference,
    pub deleted_time: Option<i64>,
    pub properties: Option<PropertiesObject<TProperties>>,
}

type PropertiesObject<TProperties> = HashMap<String, HashMap<String, TProperties>>;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectRelationReference {
    pub space: String,
    pub external_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeAndEdgeRetrieveRequest {
    pub sources: Option<Vec<SourceReferenceInternal>>,
    pub items: Vec<NodeOrEdgeSpecification>,
    #[derivative(Default(value = "false"))]
    pub include_typing: Option<bool>,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceReferenceInternal {
    pub source: SourceReference,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeAndEdgeRetrieveResponse<TProperties> {
    pub items: Vec<NodeOrEdge<TProperties>>,
    pub typing: Option<TypeInformation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "instanceType")]
pub enum NodeOrEdgeSpecification {
    Node(ItemId),
    Edge(ItemId),
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum InstanceType {
    #[default]
    Node,
    Edge,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PropertySort {
    pub property: Vec<String>,
    pub direction: Option<SortDirection>,
    pub nulls_first: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilterInstancesRequest {
    pub include_typing: Option<bool>,
    pub sources: Option<Vec<TaggedViewReference>>,
    pub instance_type: InstanceType,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub sort: Option<Vec<PropertySort>>,
    pub filter: Option<FdmFilter>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypePropertyDefinition {
    pub nullable: Option<bool>,
    pub auto_increment: Option<bool>,
    pub default_value: Option<RawValue>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub r#type: ViewCorePropertyType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeInformation(
    HashMap<String, HashMap<String, HashMap<String, TypePropertyDefinition>>>,
);

impl TypeInformation {
    pub fn get_property_info(
        &self,
        space: &str,
        view_or_container: &str,
        property: &str,
    ) -> Option<&TypePropertyDefinition> {
        self.0.get(space)?.get(view_or_container)?.get(property)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstancesFilterResponse<TProperties> {
    pub items: Vec<NodeOrEdge<TProperties>>,
    pub typing: Option<TypeInformation>,
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum InstancesAggregate {
    Avg { property: String },
    Count { property: String },
    Min { property: String },
    Max { property: String },
    Sum { property: String },
    Histogram { property: String, interval: f64 },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregateInstancesRequest {
    pub query: Option<String>,
    pub properties: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub aggregates: Option<Vec<InstancesAggregate>>,
    pub group_by: Option<Vec<String>>,
    pub filter: Option<FdmFilter>,
    pub instance_type: InstanceType,
    pub view: TaggedViewReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AggregateGroupValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NumericAggregateResult {
    pub property: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistogramBucket {
    pub start: f64,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AggregateResult {
    Avg(NumericAggregateResult),
    Min(NumericAggregateResult),
    Max(NumericAggregateResult),
    Count(NumericAggregateResult),
    Sum(NumericAggregateResult),
    Histogram {
        interval: f64,
        property: String,
        buckets: Vec<HistogramBucket>,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregateResultItem {
    pub instance_type: InstanceType,
    pub group: Option<HashMap<String, AggregateGroupValue>>,
    pub aggregates: Vec<AggregateResult>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregateInstancesResponse {
    pub items: Vec<AggregateResultItem>,
    pub typing: Option<TypeInformation>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchInstancesRequest {
    pub view: TaggedViewReference,
    pub query: Option<String>,
    pub instance_type: Option<InstanceType>,
    pub properties: Option<Vec<String>>,
    pub filter: Option<FdmFilter>,
    pub limit: Option<i32>,
}
