use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Instance<TNodeOrEdgeProperties> {
    Node(NodeWrite<TNodeOrEdgeProperties>),
    Edge(EdgeWrite<TNodeOrEdgeProperties>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeWrite<TNodeOrEdgeProperties> {
    pub instance_type: String,
    pub space: String,
    pub external_id: String,
    pub sources: Vec<EdgeOrNodeData<TNodeOrEdgeProperties>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeWrite<TNodeOrEdgeProperties> {
    pub instance_type: String,
    pub space: String,
    pub r#type: SpaceExternalId,
    pub external_id: String,
    pub start_node: SpaceExternalId,
    pub end_node: SpaceExternalId,
    pub sources: Vec<EdgeOrNodeData<TNodeOrEdgeProperties>>,
}

#[derive(Serialize, Deserialize)]
pub struct EdgeOrNodeData<TNodeOrEdgeProperties> {
    pub source: SourceReference,
    pub properties: TNodeOrEdgeProperties,
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
    pub instance_type: String,
    pub space: String,
    pub version: String,
    pub was_modified: bool,
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDefinition<TNodeOrEdgeProperties> {
    pub instance_type: String,
    pub space: String,
    pub version: String,
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<TNodeOrEdgeProperties>>,
    pub created_time: i64,
    pub last_updated_time: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeDefinition<TNodeOrEdgeProperties> {
    pub instance_type: String,
    pub space: String,
    pub r#type: SpaceExternalId,
    pub version: String,
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<TNodeOrEdgeProperties>>,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub start_node: SpaceExternalId,
    pub end_node: SpaceExternalId,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NodeOrEdge<TProperties> {
    Node(NodeDefinition<TProperties>),
    Edge(EdgeDefinition<TProperties>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceExternalId {
    pub space: String,
    pub external_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveInstancesRequest {
    pub instance_type: String,
    pub external_id: String,
    pub space: String,
}

impl From<(String, String, String)> for RetrieveInstancesRequest {
    fn from((instance_type, external_id, space): (String, String, String)) -> Self {
        RetrieveInstancesRequest {
            instance_type: instance_type.to_string(),
            external_id: external_id.to_string(),
            space: space.to_string(),
        }
    }
}
