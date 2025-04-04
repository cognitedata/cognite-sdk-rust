use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{TaggedContainerReference, UsedFor},
    to_query, IntoParams, RawValue, SetCursor,
};

use super::common::{CDFExternalIdReference, EnumProperty, PrimitiveProperty, TextProperty};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
/// Property variants in containers.
pub enum ContainerPropertyType {
    /// Text property
    Text(TextProperty),
    /// Boolean property
    Boolean(PrimitiveProperty),
    /// 32-bit floating point property
    Float32(PrimitiveProperty),
    /// 64-bit floating point property
    Float64(PrimitiveProperty),
    /// 32-bit integer property.
    Int32(PrimitiveProperty),
    /// 64-bit integer property.
    Int64(PrimitiveProperty),
    /// Timestamp property.
    Timestamp(PrimitiveProperty),
    /// Date property.
    Date(PrimitiveProperty),
    /// JSON property.
    Json(PrimitiveProperty),
    /// Time series reference property.
    Timeseries(CDFExternalIdReference),
    /// File reference property.
    File(CDFExternalIdReference),
    /// Sequence reference property.
    Sequence(CDFExternalIdReference),
    /// Node reference property.
    Direct(DirectNodeRelationType),
    /// Enum property.
    Enum(EnumProperty),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Constraints for a property that is a reference to a node.
pub struct DirectNodeRelationType {
    /// Container the referenced node must be in.
    pub container: Option<TaggedContainerReference>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "constraintType")]
/// Constraint on a container.
pub enum ContainerConstraint {
    /// In order to have values in this container,
    /// a node or edge must also have values in a different
    /// container.
    Requires {
        /// Required container
        require: TaggedContainerReference,
    },
    /// The given properties must contain only unique sets of values.
    Uniqueness {
        /// Properties in constraint
        properties: Vec<String>,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "indexType")]
/// Index on a container.
pub enum ContainerIndex {
    /// BTree index on a set of properties
    Btree {
        /// List of properties in index.
        properties: Vec<String>,
        /// Whether it should be possible to paginate on this cursor.
        cursorable: Option<bool>,
    },
    /// Inverted index on a set of properties.
    Inverted {
        /// Properties in index.
        properties: Vec<String>,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Definition for a container property
pub struct ContainerPropertyDefinition {
    /// Whether this property is nullable, default `true`
    pub nullable: Option<bool>,
    /// Whether this property auto-increments.
    pub auto_increment: Option<bool>,
    /// Default value of this property.
    pub default_value: Option<RawValue>,
    /// Property description.
    pub description: Option<String>,
    /// Property name.
    pub name: Option<String>,
    #[serde(rename = "type")]
    /// Property type.
    pub r#type: ContainerPropertyType,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Create a container.
pub struct ContainerCreate {
    /// Container space.
    pub space: String,
    /// Container external ID.
    pub external_id: String,
    /// Container name.
    pub name: Option<String>,
    /// Container description.
    pub description: Option<String>,
    /// Whether this container can be used for nodes, edges, or both.
    pub used_for: Option<UsedFor>,
    /// Properties in this container.
    pub properties: HashMap<String, ContainerPropertyDefinition>,
    /// Container constraints.
    pub constraints: HashMap<String, ContainerConstraint>,
    /// Container indexes.
    pub indexes: HashMap<String, ContainerIndex>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Data modeling container.
pub struct ContainerDefinition {
    /// Container space.
    pub space: String,
    /// Container external ID.
    pub external_id: String,
    /// Container name.
    pub name: Option<String>,
    /// Container description.
    pub description: Option<String>,
    /// Whether this container can be used for nodes, edges, or both.
    pub used_for: Option<UsedFor>,
    /// Properties in this container.
    pub properties: HashMap<String, ContainerPropertyDefinition>,
    /// Container constraints.
    pub constraints: HashMap<String, ContainerConstraint>,
    /// Container indexes.
    pub indexes: HashMap<String, ContainerIndex>,
    /// Time this file was created, in milliseconds since epoch.
    pub created_time: i64,
    /// Time this file was last updated, in milliseconds since epoch.
    pub last_updated_time: i64,
    /// Whether this is a global container
    pub is_global: bool,
}

impl From<ContainerDefinition> for ContainerCreate {
    fn from(value: ContainerDefinition) -> Self {
        Self {
            space: value.space,
            external_id: value.external_id,
            name: value.name,
            description: value.description,
            used_for: value.used_for,
            properties: value.properties,
            constraints: value.constraints,
            indexes: value.indexes,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// ID of a container index or constraint.
pub struct ContainerComponentId {
    /// Container space
    pub space: String,
    /// Container external ID.
    pub container_external_id: String,
    /// Index or constraint identifier.
    pub identifier: String,
}

#[derive(Default, Clone, Debug)]
/// Query for listing containers.
pub struct ContainerQuery {
    /// Maximum number of containers in the result.
    pub limit: Option<i32>,
    /// Optional cursor for pagination.
    pub cursor: Option<String>,
    /// Filter on container space.
    pub space: Option<String>,
    /// Include global containers.
    pub include_global: Option<bool>,
}

impl IntoParams for ContainerQuery {
    fn into_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("space", &self.space, &mut params);
        to_query("includeGlobal", &self.include_global, &mut params);
        params
    }
}

impl SetCursor for ContainerQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}
