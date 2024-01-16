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
/// Reference to a property on a view.
pub struct ViewPropertyReference {
    /// View reference
    pub view: TaggedViewReference,
    /// Property identifier.
    pub identifier: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Query for nodes.
pub struct NodesQuery {
    /// Chain result starting from this query.
    pub from: Option<String>,
    /// Chain result through this direct relation property in the
    /// query referenced by `from`.
    pub through: Option<ViewPropertyReference>,
    /// Filter on nodes to retrieve.
    pub filter: Option<AdvancedFilter>,
    /// The direction to use when traversing direct relations. Only
    /// applicable when `through` is specified.
    pub direction: Option<QueryDirection>,
    /// Control which side of the edge to chain from. This option is only applicable if
    /// the view referenced in the `from` field consists of edges.
    pub chain_to: Option<QueryChainSide>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Expression for querying nodes.
pub struct QueryNodeTableExpression {
    /// Sort result set by a list of properties. The order is significant.
    pub sort: Option<Vec<PropertySort>>,
    /// Maximum number of nodes to retrieve.
    pub limit: Option<i32>,
    /// Node query.
    pub nodes: NodesQuery,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Direction to query edges or direct relations.
pub enum QueryDirection {
    #[default]
    /// Query edges or direct relations outwards.
    Outwards,
    /// Query edges or direct relations inwards.
    Inwards,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Which side of edge to chain from.
pub enum QueryChainSide {
    /// Chain to `start` if `direction = outwards`, or
    /// `end` if `direction = inwards`
    Source,
    #[default]
    /// Chain to `end` if `direction = outwards`, or
    /// `start` if direction `inwards`.
    Destination,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Query for edges
pub struct EdgesQuery {
    /// Chain result starting from this query.
    pub from: Option<String>,
    /// Maximum number of levels to traverse.
    pub max_distance: Option<i32>,
    /// The direction to use when traversing. Defaults to `outwards`.
    pub direction: Option<QueryDirection>,
    /// Filter on edges.
    pub filter: Option<AdvancedFilter>,
    /// Filter on nodes along the path.
    pub node_filter: Option<AdvancedFilter>,
    /// Termination filter, if this matches, the query won't go any deeper, but the
    /// edge will be included.
    pub termination_filter: Option<AdvancedFilter>,
    /// Limit the number of returned edges for each of the source nodes in the result set.
    /// The indicated limit applies to the result set from the referenced `from`.
    /// `limit_each` only has meaning when you also specify `max_distance = 1` and `from`.
    pub limit_each: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Expression for querying edges.
pub struct QueryEdgeTableExpression {
    /// Sort result set by a list of properties. The order is significant.
    pub sort: Option<Vec<PropertySort>>,
    /// Sort result set when using recursive graph traversal. The order is significant.
    pub post_sort: Option<Vec<PropertySort>>,
    /// Maximum number of edges to retrieve.
    pub limit: Option<i32>,
    /// Query for edges.
    pub edges: EdgesQuery,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// Composite query performing operations on other result sets.
pub enum QuerySetOperationTableExpression {
    /// Return the union of the specified result sets. May return duplicate results.
    UnionAll {
        /// List of query sets or references to other queries.
        union_all: Vec<QuerySetOrString>,
        /// Exclude matching results.
        except: Option<Vec<String>>,
        /// Maximum number of results in result set.
        limit: Option<i32>,
    },
    /// Return the union of the specified result sets, will not return duplicate results.
    ///
    /// Note: Using `UnionAll` is more efficient in general.
    Union {
        /// List of query sets or references to other queries.
        union: Vec<QuerySetOrString>,
        /// Exclude matching results.
        except: Option<Vec<String>>,
        /// Maximum number of results in result set.
        limit: Option<i32>,
    },
    /// Find the common elements in the returned result set.
    Intersection {
        /// List of query sets or references to other queries.
        intersection: Vec<QuerySetOrString>,
        /// Exclude matching results.
        except: Option<Vec<String>>,
        /// Maximum number of results in result set.
        limit: Option<i32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// Element of a query set operation. Either another query set, or a reference
/// to a different query.
pub enum QuerySetOrString {
    /// Nested query set.
    QuerySet(Box<QuerySetOperationTableExpression>),
    /// Reference to a query.
    String(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// Expression for querying instances.
pub enum QueryTableExpression {
    /// Query nodes.
    Node(QueryNodeTableExpression),
    /// Query edges.
    Edge(QueryEdgeTableExpression),
    /// Perform complex operations on other query results.
    SetOperation(QuerySetOperationTableExpression),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Select a set of properties from a view.
pub struct SourceSelector {
    /// View identifier.
    pub source: TaggedViewReference,
    /// List of property identifiers.
    pub properties: Vec<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Expression for selecting properties from a list of views.
pub struct SelectExpression {
    /// List of views to retrieve from.
    pub sources: Vec<SourceSelector>,
    /// Optional sort on returned values.
    pub sort: Option<Vec<PropertySort>>,
    /// Maximum number of values to return.
    pub limit: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Request for querying instances.
pub struct QueryInstancesRequest {
    /// Collection of queries, indexed by their ID.
    pub with: HashMap<String, QueryTableExpression>,
    /// Map of cursors. The keys here should match the expressions in
    /// the `with` clause.
    pub cursors: Option<HashMap<String, String>>,
    /// Define which properties to return for each query.
    pub select: HashMap<String, SelectExpression>,
    /// Values in filters can be parameterised. Parameters are provided as part of the query object,
    /// and referenced in the filter itself.
    pub parameters: Option<HashMap<String, RawValue>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Response for querying instances.
pub struct QueryInstancesResponse<TProperties> {
    /// Retrieved instances, grouped by query.
    pub items: HashMap<String, Vec<NodeOrEdge<TProperties>>>,
    /// Set of cursors for pagination.
    pub next_cursor: Option<HashMap<String, String>>,
}
