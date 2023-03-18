use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeAndEdgeCreateCollection<TProperties> {
    pub items: Vec<NodeOrEdgeCreate<TProperties>>,
    pub auto_create_start_nodes: Option<bool>,
    pub auto_create_end_nodes: Option<bool>,
    pub skip_on_version_conflict: Option<bool>,
    pub replace: Option<bool>,
}

impl<TProperties> Default for NodeAndEdgeCreateCollection<TProperties> {
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
#[serde(rename_all = "camelCase")]
pub struct SourceReferenceId {
    pub space: String,
    pub external_id: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SourceReference {
    View(SourceReferenceId),
    Container(InstanceId),
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
pub enum NodeOrEdge {
    Node(NodeDefinition),
    Edge(EdgeDefinition),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeDefinition {
    pub space: String,
    pub version: i32,
    pub external_id: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub deleted_time: Option<i64>,
    pub properties: Option<PropertiesObject>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EdgeDefinition {
    pub space: String,
    pub r#type: DirectRelationReference,
    pub version: String,
    pub external_id: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub start_node: DirectRelationReference,
    pub end_node: DirectRelationReference,
    pub deleted_time: Option<i64>,
    pub properties: Option<PropertiesObject>,
}

type PropertiesObject = serde_json::Map<String, serde_json::Value>;

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
pub struct NodeAndEdgeRetrieveResponse {
    pub items: Vec<NodeOrEdge>,
    pub typing: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "instanceType")]
pub enum NodeOrEdgeSpecification {
    Node(InstanceId),
    Edge(InstanceId),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstanceId {
    pub space: String,
    pub external_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstancesFilter {
    // todo
}
