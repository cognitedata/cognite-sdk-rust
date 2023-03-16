use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Derivative, Clone)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct NodeAndEdgeCreateCollection<TProperties> {
    pub items: Vec<NodeOrEdgeCreate<TProperties>>,
    pub auto_create_start_nodes: Option<bool>,
    pub auto_create_end_nodes: Option<bool>,
    pub skip_on_version_conflict: Option<bool>,
    pub replace: Option<bool>,
}

impl Default for NodeAndEdgeCreateCollection<TProperties> {
    fn default() -> Self {
        Self {
            items: vec![],
            auto_create_start_nodes: Some(false),
            auto_create_end_nodes: Some(false),
            skip_on_version_conflict: Some(false),
            replace: Some(false),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum NodeOrEdgeCreate<TProperties> {
    Node(NodeWrite<TProperties>),
    Edge(EdgeWrite<TProperties>),
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InstanceType {
    #[default]
    Node,
    Edge,
}

#[derive(Serialize, Deserialize, Default, Derivative, Clone)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct NodeWrite<TProperties> {
    #[derivative(Default(value = "InstanceType::Node"))]
    pub instance_type: InstanceType,
    pub space: String,
    pub external_id: String,
    pub sources: Option<Vec<EdgeOrNodeData<TProperties>>>,
    pub existing_version: Option<i64>,
}

#[derive(Serialize, Deserialize, Default, Derivative, Clone)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct EdgeWrite<TProperties> {
    #[derivative(Default(value = "InstanceType::Edge"))]
    pub instance_type: InstanceType,
    pub space: String,
    pub r#type: DirectRelationReference,
    pub external_id: String,
    pub start_node: DirectRelationReference,
    pub end_node: DirectRelationReference,
    pub sources: Option<Vec<EdgeOrNodeData<TProperties>>>,
    pub existing_version: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EdgeOrNodeData<TProperties> {
    pub source: SourceReference,
    pub properties: TProperties,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceReference {
    pub r#type: SourceReferenceType,
    pub space: String,
    pub external_id: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SourceReferenceType {
    View,
    Container,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct SlimNodeOrEdge {
    pub instance_type: InstanceType,
    pub space: String,
    pub version: i64,
    pub was_modified: bool,
    pub external_id: String,
    pub created_time: Option<i64>,
    pub last_updated_time: Option<i64>,
}

#[derive(Serialize, Deserialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct NodeDefinition<TProperties> {
    #[derivative(Default(value = "InstanceType::Node"))]
    pub instance_type: InstanceType,
    pub space: String,
    pub version: String,
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<TProperties>>,
    pub created_time: i64,
    pub last_updated_time: i64,
}

#[derive(Serialize, Deserialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct EdgeDefinition<TProperties> {
    #[derivative(Default(value = "InstanceType::Edge"))]
    pub instance_type: InstanceType,
    pub space: String,
    pub r#type: DirectRelationReference,
    pub version: String,
    pub external_id: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub start_node: DirectRelationReference,
    pub end_node: DirectRelationReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<TProperties>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum NodeOrEdge<TProperties> {
    Node(NodeDefinition<TProperties>),
    Edge(EdgeDefinition<TProperties>),
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DirectRelationReference {
    pub space: String,
    pub external_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
    pub instance_type: InstanceType,
    pub external_id: String,
    pub space: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstanceId {
    pub external_id: String,
    pub space: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct InstancesFilter {
    // todo
}
