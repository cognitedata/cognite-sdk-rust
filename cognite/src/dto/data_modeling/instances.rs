use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Derivative)]
pub struct NodeAndEdgeCreateCollection<TProperties> {
    pub items: Vec<NodeOrEdgeCreate<TProperties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "Some(false)"))]
    pub auto_create_start_nodes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "Some(false)"))]
    pub auto_create_end_nodes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "Some(false)"))]
    pub skip_on_version_conflict: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Default(value = "Some(false)"))]
    pub replace: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum NodeOrEdgeCreate<TProperties> {
    Node(NodeWrite<TProperties>),
    Edge(EdgeWrite<TProperties>),
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum InstanceType {
    #[default]
    Node,
    Edge,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRequest {}

#[derive(Serialize, Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct NodeWrite<TProperties> {
    #[derivative(Default(value = "InstanceType::Node"))]
    pub instance_type: InstanceType,
    pub space: String,
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<EdgeOrNodeData<TProperties>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    existing_version: Option<i64>,
}

#[derive(Serialize, Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct EdgeWrite<TProperties> {
    #[derivative(Default(value = "InstanceType::Edge"))]
    pub instance_type: InstanceType,
    pub space: String,
    pub r#type: EdgeType,
    pub external_id: String,
    pub start_node: DirectRelationReference,
    pub end_node: DirectRelationReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<EdgeOrNodeData<TProperties>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    existing_version: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct EdgeOrNodeData<TProperties> {
    pub source: SourceReference,
    pub properties: TProperties,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceReference {
    pub r#type: String,
    pub space: String,
    pub external_id: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlimNodeOrEdge {
    pub instance_type: InstanceType,
    pub space: String,
    pub version: String,
    pub was_modified: bool,
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub r#type: EdgeType,
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

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EdgeType {
    pub space: String,
    pub external_id: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectRelationReference {
    pub space: String,
    pub external_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
    pub instance_type: String,
    pub external_id: String,
    pub space: String,
}

impl From<(String, String, String)> for InstanceInfo {
    fn from((instance_type, external_id, space): (String, String, String)) -> Self {
        InstanceInfo {
            instance_type,
            external_id,
            space,
        }
    }
}
