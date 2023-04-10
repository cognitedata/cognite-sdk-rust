use std::collections::HashMap;

use crate::{to_query, AsParams};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{CorePropertyType, DefaultValue, UsedFor};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename = "container")]
pub struct ContainerReference {
    pub space: String,
    pub external_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerListQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub space: Option<String>,
}

impl AsParams for ContainerListQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query("limit", &self.limit, &mut params);
        to_query("cursor", &self.cursor, &mut params);
        to_query("space", &self.space, &mut params);
        params
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerRetrieveQuery {
    pub include_inherited_properties: Option<bool>,
}

impl AsParams for ContainerRetrieveQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
        let mut params = Vec::<(String, String)>::new();
        to_query(
            "includeInheritedProperties",
            &self.include_inherited_properties,
            &mut params,
        );
        params
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerCreateDefinition {
    pub external_id: String,
    pub space: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub used_for: Option<UsedFor>,
    pub properties: HashMap<String, ContainerPropertyDefinition>,
    pub constraints: Option<HashMap<String, ContainerConstraintDefinition>>,
    pub indexes: Option<HashMap<String, ContainerIndexDefinition>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerDefinition {
    pub external_id: String,
    pub space: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub used_for: Option<UsedFor>,
    pub properties: HashMap<String, ContainerPropertyDefinition>,
    pub constraints: Option<HashMap<String, ContainerConstraintDefinition>>,
    pub indexes: Option<HashMap<String, ContainerIndexDefinition>>,
    pub created_time: i64,
    pub last_updated_time: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPropertyDefinition {
    pub nullable: Option<bool>,
    pub auto_increment: Option<bool>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub default_value: Option<DefaultValue>,
    pub r#type: CorePropertyType,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextProperty {
    #[derivative(Default(value = "false"))]
    pub list: Option<bool>,
    pub collation: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrimitiveProperty {
    #[derivative(Default(value = "false"))]
    pub list: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "constraintType")]
pub enum ContainerConstraintDefinition {
    Requires(RequiresConstraintDefinition),
    Uniqueness(UniquenessConstraintDefinition),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequiresConstraintDefinition {
    pub require: ContainerReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UniquenessConstraintDefinition {
    pub properties: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "indexType")]
pub enum ContainerIndexDefinition {
    BTree(BTreeIndexDefinition),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BTreeIndexDefinition {
    pub properties: Vec<String>,
}
