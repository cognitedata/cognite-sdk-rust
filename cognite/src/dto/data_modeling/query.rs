use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{instances::NodeOrEdge, PropertySort, TaggedViewReference},
    AdvancedFilter, RawValue,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewPropertyReference {
    pub view: TaggedViewReference,
    pub identifier: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodesQuery {
    pub from: Option<String>,
    pub through: Option<ViewPropertyReference>,
    pub filter: Option<AdvancedFilter>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryNodeTableExpression {
    pub sort: Option<Vec<PropertySort>>,
    pub limit: Option<i32>,
    pub nodes: NodesQuery,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum EdgeDirection {
    #[default]
    Outwards,
    Inwards,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EdgesQuery {
    pub from: Option<String>,
    pub max_distance: Option<i32>,
    pub direction: Option<EdgeDirection>,
    pub filter: Option<AdvancedFilter>,
    pub node_filter: Option<AdvancedFilter>,
    pub termination_filter: Option<AdvancedFilter>,
    pub limit_each: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryEdgeTableExpression {
    pub sort: Option<Vec<PropertySort>>,
    pub post_sort: Option<Vec<PropertySort>>,
    pub limit: Option<i32>,
    pub edges: EdgesQuery,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum QuerySetOperationTableExpression {
    UnionAll {
        union_all: Vec<QuerySetOrString>,
        except: Option<Vec<String>>,
        limit: Option<i32>,
    },
    Union {
        union: Vec<QuerySetOrString>,
        except: Option<Vec<String>>,
        limit: Option<i32>,
    },
    Intersection {
        intersection: Vec<QuerySetOrString>,
        except: Option<Vec<String>>,
        limit: Option<i32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum QuerySetOrString {
    QuerySet(Box<QuerySetOperationTableExpression>),
    String(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum QueryTableExpression {
    Node(QueryNodeTableExpression),
    Edge(QueryEdgeTableExpression),
    SetOperation(QuerySetOperationTableExpression),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceSelector {
    pub source: TaggedViewReference,
    pub properties: Vec<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelectExpression {
    pub sources: Vec<SourceSelector>,
    pub sort: Option<Vec<PropertySort>>,
    pub limit: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryInstancesRequest {
    pub with: HashMap<String, QueryTableExpression>,
    pub cursors: Option<HashMap<String, String>>,
    pub select: HashMap<String, SelectExpression>,
    pub parameters: Option<HashMap<String, RawValue>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryInstancesResponse<TProperties> {
    pub items: HashMap<String, Vec<NodeOrEdge<TProperties>>>,
    pub next_cursor: Option<HashMap<String, String>>,
}
