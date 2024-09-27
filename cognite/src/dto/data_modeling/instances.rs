use std::collections::HashMap;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{ItemId, PropertySort, SourceReference, TaggedViewReference},
    AdvancedFilter, RawValue, SetCursor,
};

use super::views::ViewCorePropertyType;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// A list of nodes and edges to create. Parametrized by the type used for
/// instance properties.
pub struct NodeAndEdgeCreateCollection<TProperties> {
    /// Nodes and edges to create.
    pub items: Vec<NodeOrEdgeCreate<TProperties>>,
    /// Whether to auto create direct relations that do not exist.
    pub auto_create_direct_relations: Option<bool>,
    /// Whether to auto create start nodes that do not exist.
    pub auto_create_start_nodes: Option<bool>,
    /// Whether to auto create end nodes that do not exist.
    pub auto_create_end_nodes: Option<bool>,
    /// If `existing_version` is specified on any of the nodes/edges in the input,
    /// the default behaviour is that the entire ingestion will fail when
    /// version conflicts occur. If `skip_on_version_conflict` is set to true,
    /// items with version conflicts will be skipped instead. If no version is
    /// specified for nodes/edges, it will do the write directly.
    pub skip_on_version_conflict: Option<bool>,
    /// How do we behave when a property value exists?
    /// Do we replace all matching and existing values with the supplied values (true)?
    /// Or should we merge in new values for properties together with the existing values (false)?
    /// Note: This setting applies for all nodes or edges specified in the ingestion call.
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
/// Enum of instance types being created.
pub enum NodeOrEdgeCreate<TProperties> {
    /// Create a node.
    Node(NodeWrite<TProperties>),
    /// Create an edge.
    Edge(EdgeWrite<TProperties>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Node to create.
pub struct NodeWrite<TProperties> {
    /// Node space.
    pub space: String,
    /// Node external ID.
    pub external_id: String,
    /// List of properties in various containers or views.
    pub sources: Option<Vec<EdgeOrNodeData<TProperties>>>,
    /// Fail the ingestion request if the node's version is greater than or equal to this value.
    /// If no `existing_version` is specified, the ingestion will always overwrite any existing
    /// data for the node (for the specified container or view). If `existing_version` is
    /// set to 0, the upsert will behave as an insert, so it will fail the bulk if the
    /// item already exists. If skipOnVersionConflict is set on the ingestion request,
    /// then the item will be skipped instead of failing the ingestion request.
    pub existing_version: Option<i32>,
    /// Node type (direct relation).
    pub r#type: Option<InstanceId>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Edge to create.
pub struct EdgeWrite<TProperties> {
    /// Edge space.
    pub space: String,
    /// Edge type (direct relation).
    pub r#type: InstanceId,
    /// Edge external ID.
    pub external_id: String,
    /// Edge start node.
    pub start_node: InstanceId,
    /// Edge end node.
    pub end_node: InstanceId,
    /// List of properties in various containers or views.
    pub sources: Option<Vec<EdgeOrNodeData<TProperties>>>,
    /// Fail the ingestion request if the edge's version is greater than or equal to this value.
    /// If no `existing_version` is specified, the ingestion will always overwrite any existing
    /// data for the edge (for the specified container or view). If `existing_version` is
    /// set to 0, the upsert will behave as an insert, so it will fail the bulk if the
    /// item already exists. If skipOnVersionConflict is set on the ingestion request,
    /// then the item will be skipped instead of failing the ingestion request.
    pub existing_version: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Data in a single container or view for an edge or a node.
pub struct EdgeOrNodeData<TProperties> {
    /// Reference to the container or view this data belongs to.
    pub source: SourceReference,
    /// Property values on the form `{ property_name: value, ... }`
    pub properties: TProperties,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "instanceType")]
/// Minimal representation of a node or edge.
pub enum SlimNodeOrEdge {
    /// Slim node
    Node(SlimNodeDefinition),
    /// Slim edge
    Edge(SlimEdgeDefinition),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Minimal representation of a node.
pub struct SlimNodeDefinition {
    /// Node space.
    pub space: String,
    /// Node version.
    pub version: i32,
    /// Whether or not the node was midified by this ingestion. We only
    /// update nodes if the input differs from the existing state.
    pub was_modified: bool,
    /// Node external ID.
    pub external_id: String,
    /// Time this node was created, in milliseconds since epoch.
    pub created_time: Option<i64>,
    /// Time this node was last modified, in milliseconds since epoch.
    pub last_updated_time: Option<i64>,
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Minimal representation of an edge.
pub struct SlimEdgeDefinition {
    /// Edge space.
    pub space: String,
    /// Edge version.
    pub version: i32,
    /// Whether or not the edge was midified by this ingestion. We only
    /// update nodes if the input differs from the existing state.
    pub was_modified: bool,
    /// Edge external ID.
    pub external_id: String,
    /// Time this edge was created, in milliseconds since epoch.
    pub created_time: Option<i64>,
    /// Time this edge was last modified, in milliseconds since epoch.
    pub last_updated_time: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "instanceType")]
/// Node or edge with properties.
pub enum NodeOrEdge<TProperties> {
    /// Full node.
    Node(NodeDefinition<TProperties>),
    /// Full edge.
    Edge(EdgeDefinition<TProperties>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Representation of a node with properties.
pub struct NodeDefinition<TProperties> {
    /// Node space.
    pub space: String,
    /// Node version.
    pub version: i32,
    /// Node external ID.
    pub external_id: String,
    /// Node type.
    pub r#type: Option<InstanceId>,
    /// Time this node was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this node was last modified, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// Timestamp when the node was soft deleted. Note that deleted nodes are
    /// filtered out of query results, but present in sync results.
    /// This means that this value will only be present in sync results.
    pub deleted_time: Option<i64>,
    /// Node properties.
    pub properties: Option<PropertiesObject<TProperties>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Representation of an edge with properties.
pub struct EdgeDefinition<TProperties> {
    /// Edge space.
    pub space: String,
    /// Edge type.
    pub r#type: InstanceId,
    /// Edge version.
    pub version: String,
    /// Edge external ID.
    pub external_id: String,
    /// Time this edge was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this edge was last modified, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// Edge start node.
    pub start_node: InstanceId,
    /// Edge end node.
    pub end_node: InstanceId,
    /// Timestamp when the edge was soft deleted. Note that deleted nodes are
    /// filtered out of query results, but present in sync results.
    /// This means that this value will only be present in sync results.
    pub deleted_time: Option<i64>,
    /// Edge properties.
    pub properties: Option<PropertiesObject<TProperties>>,
}

/// Shorthand for map from space and view/container external ID to properties object.
pub(crate) type PropertiesObject<TProperties> = HashMap<String, HashMap<String, TProperties>>;

#[derive(Serialize, Deserialize, Default, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Direct reference to a node.
pub struct InstanceId {
    /// Node space.
    pub space: String,
    /// Node external ID.
    pub external_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Request for retrieving nodes or edges by ID.
pub struct NodeAndEdgeRetrieveRequest {
    /// List of sources to include properties from.
    pub sources: Option<Vec<SourceReferenceInternal>>,
    /// List of node or edge IDs to retrieve.
    pub items: Vec<NodeOrEdgeSpecification>,
    #[derivative(Default(value = "false"))]
    /// Whether to include type information in the response.
    pub include_typing: Option<bool>,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Wrapped reference to a view.
pub struct SourceReferenceInternal {
    /// View ID.
    pub source: TaggedViewReference,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Response for retrieving nodes and edges.
pub struct NodeAndEdgeRetrieveResponse<TProperties> {
    /// Retrieved nodes and edges.
    pub items: Vec<NodeOrEdge<TProperties>>,
    /// Type information if requested.
    pub typing: Option<TypeInformation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "instanceType")]
/// ID of a node or an edge.
pub enum NodeOrEdgeSpecification {
    /// Node ID.
    Node(ItemId),
    /// Edge ID.
    Edge(ItemId),
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Enum over instance types.
pub enum InstanceType {
    /// Node instances.
    #[default]
    Node,
    /// Edge instances.
    Edge,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Request for filtering instances.
pub struct FilterInstancesRequest {
    /// Whether to include type information in the response.
    pub include_typing: Option<bool>,
    /// List of sources to retrieve data from.
    pub sources: Option<Vec<SourceReferenceInternal>>,
    /// Type of instances to filter.
    pub instance_type: InstanceType,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Maximum number of instances to retrieve. Default 1000, maximum 1000.
    pub limit: Option<i32>,
    /// Optional list of properties to sort the result by.
    pub sort: Option<Vec<PropertySort>>,
    /// Optional filter.
    pub filter: Option<AdvancedFilter>,
}

impl SetCursor for FilterInstancesRequest {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Defintion of a property in type information.
pub struct TypePropertyDefinition {
    /// Whether the property is nullable.
    pub nullable: Option<bool>,
    /// Whether the property auto increments.
    pub auto_increment: Option<bool>,
    /// Property default value.
    pub default_value: Option<RawValue>,
    /// Description of the content and suggested use for this property.
    pub description: Option<String>,
    /// Readable property name.
    pub name: Option<String>,
    /// The data-type to use when storing the property.
    pub r#type: ViewCorePropertyType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Type information retrieved when doing filter or retrieve requests and setting
/// `include_typing` to `true`.
///
/// Map from space to view/container to property.
pub struct TypeInformation(
    HashMap<String, HashMap<String, HashMap<String, TypePropertyDefinition>>>,
);

impl TypeInformation {
    /// Get information about a property by space, view/container external ID,
    /// and property name, if it is present in the type information.
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
/// Response when filtering instances.
pub struct InstancesFilterResponse<TProperties> {
    /// List of retrieved nodes and edges.
    pub items: Vec<NodeOrEdge<TProperties>>,
    /// Type information if requested.
    pub typing: Option<TypeInformation>,
    /// Cursor for pagination.
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Requested aggregate on instances.
pub enum InstancesAggregate {
    /// Average of values of specified property.
    Avg {
        /// Numerical property to compute average of.
        property: String,
    },
    /// Counts the number of items. When you specify a property,
    /// it returns the number of non-null values for that property.
    Count {
        /// The property to count. If specified, counts non-null values.
        property: Option<String>,
    },
    /// Calculate the lowest value for a numerical property.
    Min {
        /// Numerical property to compute minimum value for.
        property: String,
    },
    /// Calculate the highest value for a numerical property.
    Max {
        /// Numerical proeprty to compute maximum value for.
        property: String,
    },
    /// Calculate the sum of values for a numerical property.
    Sum {
        /// Numerical property to compute sum for.
        property: String,
    },
    /// A histogram aggregator function. This function will generate a histogram from the values of
    /// the specified property. It uses the specified interval as defined in your interval argument.
    Histogram {
        /// Numerical property to compute histogram for.
        property: String,
        /// Interval between each bucket.
        interval: f64,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Request for aggregating accross instances.
pub struct AggregateInstancesRequest {
    /// Optional query string. The API will parse the query string,
    /// and use it to match the text properties on elements to use for the aggregate(s).
    pub query: Option<String>,
    /// Optional list of properties to apply the `query` to. If no properties are listed,
    /// text fields are searched by default.
    pub properties: Option<Vec<String>>,
    /// Maximum number of results to return. The default is 100, maximum is 1000.
    pub limit: Option<i32>,
    /// List of aggregates to calculate.
    pub aggregates: Option<Vec<InstancesAggregate>>,
    /// The selection of fields to group the results by when doing aggregations.
    ///
    /// When you do not specify any aggregates. The fields in the `groupBy` clause will
    /// return the unique values stored for each field.
    pub group_by: Option<Vec<String>>,
    /// Optional filter on nodes or edges.
    pub filter: Option<AdvancedFilter>,
    /// Instance type to aggregate accross.
    pub instance_type: InstanceType,
    /// Reference to a view to query.
    pub view: TaggedViewReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// Value of an aggregate group.
pub enum AggregateGroupValue {
    /// String value.
    String(String),
    /// Numerical value.
    Number(f64),
    /// Boolean value.
    Boolean(bool),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Result of a numerical aggregate.
pub struct NumericAggregateResult {
    /// Aggregated property.
    pub property: String,
    /// Aggregate value.
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Result of a count aggregate.
pub struct CountAggregateResult {
    /// Aggregated property.
    pub property: Option<String>,
    /// Aggregate value.
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// A single bucket in a histogram aggregate.
pub struct HistogramBucket {
    /// Start value of bucket.
    pub start: f64,
    /// Number of values in bucket.
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Result item in instances aggregate response.
pub enum AggregateResult {
    /// Result of average aggregate.
    Avg(NumericAggregateResult),
    /// Result of min aggregate.
    Min(NumericAggregateResult),
    /// Result of max aggregate.
    Max(NumericAggregateResult),
    /// Result of count aggregate.
    Count(CountAggregateResult),
    /// Result of sum aggregate.
    Sum(NumericAggregateResult),
    /// Result of histogram aggregate.
    Histogram {
        /// Histogram interval.
        interval: f64,
        /// Aggregate property.
        property: String,
        /// List of buckets with start and count.
        buckets: Vec<HistogramBucket>,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// A single item in the result of an instances aggregate request.
pub struct AggregateResultItem {
    /// Type of instance aggregated.
    pub instance_type: InstanceType,
    /// Value of group in aggregate.
    pub group: Option<HashMap<String, AggregateGroupValue>>,
    /// List of computed aggregates for this group.
    pub aggregates: Vec<AggregateResult>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Result when aggregating instances.
pub struct AggregateInstancesResponse {
    /// Computed aggregates.
    pub items: Vec<AggregateResultItem>,
    /// Type information if requested.
    pub typing: Option<TypeInformation>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Query for searching text fields for nodes or edges.
pub struct SearchInstancesRequest {
    /// View to search in.
    pub view: TaggedViewReference,
    /// Query string that will be parsed for search.
    pub query: Option<String>,
    /// Instance type to search.
    pub instance_type: InstanceType,
    /// List of properties to search through. If you do not specify one
    /// or more properties, the service will search all text fields
    /// in the view.
    pub properties: Option<Vec<String>>,
    /// Optional advanced filter.
    pub filter: Option<AdvancedFilter>,
    /// Maximum number of results to return. Default 1000, maximum 1000.
    pub limit: Option<i32>,
}
