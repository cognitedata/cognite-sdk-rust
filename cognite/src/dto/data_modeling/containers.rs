use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{
        CDFExternalIdReference, PrimitiveProperty, RawValue, TaggedContainerReference,
        TextProperty, UsedFor,
    },
    to_query, AsParams, SetCursor,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ContainerPropertyType {
    Text(TextProperty),
    Boolean(PrimitiveProperty),
    Float32(PrimitiveProperty),
    Float64(PrimitiveProperty),
    Int32(PrimitiveProperty),
    Int64(PrimitiveProperty),
    Timestamp(PrimitiveProperty),
    Date(PrimitiveProperty),
    Json(PrimitiveProperty),
    Timeseries(CDFExternalIdReference),
    File(CDFExternalIdReference),
    Sequence(CDFExternalIdReference),
    Direct(DirectNodeRelationType),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectNodeRelationType {
    pub container: Option<TaggedContainerReference>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "constraintType")]
pub enum ContainerConstraint {
    Requires { require: TaggedContainerReference },
    Uniqueness { properties: Vec<String> },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "indexType")]
pub enum ContainerIndex {
    Btree {
        properties: Vec<String>,
        cursorable: Option<bool>,
    },
    Inverted {
        properties: Vec<String>,
    },
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPropertyDefinition {
    pub nullable: Option<bool>,
    pub auto_increment: Option<bool>,
    pub default_value: Option<RawValue>,
    pub description: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: ContainerPropertyType,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerCreate {
    pub space: String,
    pub external_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub used_for: Option<UsedFor>,
    pub properties: HashMap<String, ContainerPropertyDefinition>,
    pub constraints: HashMap<String, ContainerConstraint>,
    pub indexes: HashMap<String, ContainerIndex>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerDefinition {
    pub space: String,
    pub external_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub used_for: Option<UsedFor>,
    pub properties: HashMap<String, ContainerPropertyDefinition>,
    pub constraints: HashMap<String, ContainerConstraint>,
    pub indexes: HashMap<String, ContainerIndex>,
    pub created_time: i64,
    pub last_updated_time: i64,
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
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerComponentId {
    pub space: String,
    pub container_external_id: String,
    pub identifier: String,
}

#[derive(Default, Clone, Debug)]
pub struct ContainerQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub space: Option<String>,
    pub include_global: Option<bool>,
}

impl AsParams for ContainerQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("space", &self.space, &mut params);
        to_query("include_global", &self.include_global, &mut params);
        params
    }
}

impl SetCursor for ContainerQuery {
    fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }
}
