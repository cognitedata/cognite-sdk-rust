use std::collections::HashMap;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{
        CDFExternalIdReference, PrimitiveProperty, SourceReference, TaggedContainerReference,
        TaggedViewReference, TextProperty, UsedFor,
    },
    to_query, AdvancedFilter, IntoParams, RawValue, SetCursor,
};

use super::{
    instances::InstanceId,
    query::{QueryDirection, ViewPropertyReference},
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Reference to a view.
pub struct ViewReference {
    /// Space ID.
    pub space: String,
    /// View external ID.
    pub external_id: String,
    /// View version.
    pub version: String,
}

#[derive(Default, Clone, Debug)]
/// Query for listing views.
pub struct ViewQuery {
    /// Maximum number of views to return. Default 10, maximum 1000.
    pub limit: Option<i32>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Filter on view space.
    pub space: Option<String>,
    /// Include properties inherited from views each view implements.
    pub include_inherited_properties: Option<bool>,
    /// Whether to include all versions of the view, or just the latest.
    pub all_versions: Option<bool>,
    /// Whether to include global views.
    pub include_global: Option<bool>,
}

impl IntoParams for ViewQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("space", &self.space, &mut params);
        to_query(
            "includeInheritedProperties",
            &self.include_inherited_properties,
            &mut params,
        );
        to_query("allVersions", &self.all_versions, &mut params);
        params
    }
}

impl SetCursor for ViewQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// Create a view or reference an existing view.
pub enum ViewCreateOrReference {
    /// Create a new view.
    Create(ViewCreateDefinition),
    /// Reference an existing view.
    Reference(ViewReference),
}

impl From<ViewDefinitionOrReference> for ViewCreateOrReference {
    fn from(value: ViewDefinitionOrReference) -> Self {
        match value {
            ViewDefinitionOrReference::Definition(x) => Self::Create(x.into()),
            ViewDefinitionOrReference::Reference(x) => Self::Reference(x),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// A reference to an existing view, or a definition for a newly created view.
pub enum ViewDefinitionOrReference {
    /// Definition for a newly created view.
    Definition(ViewDefinition),
    /// Refernece to an existing view.
    Reference(ViewReference),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Create a new view.
pub struct ViewCreateDefinition {
    /// External ID identifying this view.
    ///
    /// The values `Query`, `Mutation`, `Subscription`, `String`, `Int32`, `Int64`, `Int`,
    /// `Float32`, `Float64`, `Float`, `Timestamp`, `JSONObject`, `Date`, `Numeric`,
    /// `Boolean`, `PageInfo`, `File`, `Sequence` and `TimeSeries` are reserved.
    pub external_id: String,
    /// Space this view belongs to.
    pub space: String,
    /// Human readsable name for the view.
    pub name: Option<String>,
    /// Description of the content and use of this view.
    pub description: Option<String>,
    /// Filter for instances included in this view.
    pub filter: Option<AdvancedFilter>,
    /// List of views this view implements.
    pub implements: Option<Vec<ViewReference>>,
    /// Whether this view should be used for nodes, edges, or both.
    pub used_for: Option<UsedFor>,
    /// View version.
    pub version: String,
    /// Collection of properties and connections for this view.
    pub properties: HashMap<String, CreateViewPropertyOrConnectionDefinition>,
}

impl From<ViewDefinition> for ViewCreateDefinition {
    fn from(value: ViewDefinition) -> Self {
        Self {
            external_id: value.external_id,
            space: value.space,
            name: value.name,
            description: value.description,
            implements: value.implements,
            version: value.version,
            used_for: Some(value.used_for),
            filter: value.filter,
            properties: value
                .properties
                .into_iter()
                .map(|(key, val)| (key, val.into()))
                .collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// New view property or definition of a connection.
pub enum CreateViewPropertyOrConnectionDefinition {
    /// New view property referencing a container.
    CreateViewProperty(CreateViewProperty),
    /// Connection referencing a view.
    ConnectionDefinition(ConnectionDefinition),
}

impl From<ViewDefinitionProperties> for CreateViewPropertyOrConnectionDefinition {
    fn from(value: ViewDefinitionProperties) -> Self {
        match value {
            ViewDefinitionProperties::ViewCorePropertyDefinition(p) => {
                Self::CreateViewProperty(p.into())
            }
            ViewDefinitionProperties::ConnectionDefinition(d) => Self::ConnectionDefinition(d),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Create a new view property.
pub struct CreateViewProperty {
    /// Human readable property name.
    pub name: Option<String>,
    /// Description of content and suggested use for this property.
    pub description: Option<String>,
    /// Reference to an existing container.
    pub container: TaggedContainerReference,
    /// The unique identifier for the property (Unique within the referenced container).
    pub container_property_identifier: String,
    /// Indicates what type a referenced direct relation is expected to be.
    pub source: Option<SourceReference>,
}

impl From<ViewCorePropertyDefinition> for CreateViewProperty {
    fn from(value: ViewCorePropertyDefinition) -> Self {
        Self {
            name: value.name,
            description: value.description,
            container: value.container.clone(),
            container_property_identifier: value.container_property_identifier,
            source: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Definition of a connection going through an edge.
pub struct EdgeConnection {
    /// Readable connection name.
    pub name: Option<String>,
    /// Description of the connection.
    pub description: Option<String>,
    /// Reference to the node pointed to by the edge type. Consists of a
    /// space and an external ID.
    pub r#type: InstanceId,
    /// Direction of the connection. Defaults to `outwards`.
    pub direction: Option<QueryDirection>,
    /// Which view this connection references.
    pub source: TaggedViewReference,
    /// Which view the edges of this connection belong to.
    pub edge_source: Option<TaggedViewReference>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Connection to a view through a reverse direct relation.
pub struct ReverseDirectRelationConnection {
    /// Readable connection name.
    pub name: Option<String>,
    /// Description of the connection.
    pub description: Option<String>,
    /// Which view this connection references.
    pub source: TaggedViewReference,
    /// Which property this connection uses.
    pub through: ViewPropertyReference,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "connectionType")]
/// Definition of a connection. Describes edges or reverse direct relations
/// that are expected to exist.
pub enum ConnectionDefinition {
    /// A single edge is expected to exist.
    SingleEdgeConnection(EdgeConnection),
    /// Multiple edges are expected to exist.
    MultiEdgeConnection(EdgeConnection),
    /// A single reverse direct relation is expected to exist.
    SingleReverseDirectRelation(ReverseDirectRelationConnection),
    /// Multiple reverse direct relations are expected to exist.
    MultiReverseDirectRelation(ReverseDirectRelationConnection),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Definition of a view.
pub struct ViewDefinition {
    /// View external ID.
    pub external_id: String,
    /// View space.
    pub space: String,
    /// Human readable name.
    pub name: Option<String>,
    /// Description for contents and intended use of view.
    pub description: Option<String>,
    /// Filter for instances in view.
    pub filter: Option<AdvancedFilter>,
    /// List of views this view implements.
    pub implements: Option<Vec<ViewReference>>,
    /// Version of view.
    pub version: String,
    /// Time this view was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this view was last modified, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// Whether this view can be written to, i.e. it maps all non-nullable properties.
    pub writable: bool,
    /// Whether this view can be used for nodes, edges, or both.
    pub used_for: UsedFor,
    /// List of properties and connections in this view.
    pub properties: HashMap<String, ViewDefinitionProperties>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
#[allow(clippy::large_enum_variant)]
/// Properties in a view definition.
pub enum ViewDefinitionProperties {
    /// A view property referencing a property in a container.
    ViewCorePropertyDefinition(ViewCorePropertyDefinition),
    /// A connection to a view.
    ConnectionDefinition(ConnectionDefinition),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Definition of a view property.
pub struct ViewCorePropertyDefinition {
    #[derivative(Default(value = "true"))]
    /// Whether the property value is optional.
    pub nullable: Option<bool>,
    /// Whether the property value auto-increments.
    pub auto_increment: Option<bool>,
    /// Default value of the property.
    pub default_value: Option<RawValue>,
    /// Description of the content and suggested use for this property.
    pub description: Option<String>,
    /// Human readable property name.
    pub name: Option<String>,
    /// Property type.
    pub r#type: ViewCorePropertyType,
    /// Container reference.
    pub container: TaggedContainerReference,
    /// Unique identifier within the referenced container.
    pub container_property_identifier: String,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "type")]
/// View property type.
pub enum ViewCorePropertyType {
    /// Text property
    Text(TextProperty),
    /// Boolean property.
    Boolean(PrimitiveProperty),
    /// 32 bit floating point property.
    Float32(PrimitiveProperty),
    /// 64 bit floating point property.
    Float64(PrimitiveProperty),
    /// 32 bit integer property.
    Int32(PrimitiveProperty),
    /// 64 bit integer property.
    Int64(PrimitiveProperty),
    /// Timestamp property.
    Timestamp(PrimitiveProperty),
    /// Date property.
    Date(PrimitiveProperty),
    /// JSON object property.
    Json(PrimitiveProperty),
    /// Reference to a CDF timeseries.
    Timeseries(CDFExternalIdReference),
    /// Reference to a CDF file.
    File(CDFExternalIdReference),
    /// Reference to a CDF sequence.
    Sequence(CDFExternalIdReference),
    /// Direct relation to a node.
    Direct(ViewDirectNodeRelation),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Direct node relation type, can include a hint to specify the view this direct
/// relation points to.
pub struct ViewDirectNodeRelation {
    /// The required type for the node the direct relation points to.
    pub container: Option<TaggedContainerReference>,
    /// Hint showing the view that the direct relation points to.
    pub source: Option<TaggedViewReference>,
}
