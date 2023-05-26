use std::collections::HashMap;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{ItemId, SourceReference},
    to_query, AsParams,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewReference {
    pub r#type: String,
    pub space: String,
    pub external_id: String,
    pub version: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    pub space: Option<String>,
    pub include_inherited_properties: Option<bool>,
    pub all_versions: Option<bool>,
}

impl AsParams for ViewQuery {
    fn to_tuples(self) -> Vec<(String, String)> {
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewCreateDefinition {
    pub external_id: String,
    pub space: String,
    pub name: Option<String>,
    pub description: Option<String>,
    // pub filter: Option<String>,
    pub implements: Option<Vec<ViewReference>>,
    pub version: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub writable: bool,
    pub used_for: String,
    pub properties: HashMap<String, CreateViewPropertyOrConnectionDefinition>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum CreateViewPropertyOrConnectionDefinition {
    CreateViewProperty(CreateViewProperty),
    ConnectionDefinition(ConnectionDefinition),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateViewProperty {
    pub name: Option<String>,
    pub description: Option<String>,
    pub container: SourceReference,
    pub container_property_identifier: String,
    pub source: Option<SourceReference>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDefinition {
    pub name: Option<String>,
    pub description: Option<String>,
    pub r#type: ItemId,
    pub direction: Option<String>,
    pub source: SourceReference,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewDefinition {
    pub external_id: String,
    pub space: String,
    pub name: Option<String>,
    pub description: Option<String>,
    // pub filter: Option<String>,
    pub implements: Option<Vec<ViewReference>>,
    pub version: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub writable: bool,
    pub used_for: String,
    pub properties: HashMap<String, ViewDefinitionProperties>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ViewDefinitionProperties {
    ConnectionDefinition(ConnectionDefinition),
    ViewCorePropertyDefinition(ViewCorePropertyDefinition),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewCorePropertyDefinition {
    #[derivative(Default(value = "true"))]
    pub nullable: Option<bool>,
    pub auto_increment: Option<bool>,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub r#type: ViewCorePropertyType,
    pub container: SourceReference,
    pub container_property_identifier: String,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ViewCorePropertyType {
    Text(TextProperty),
    Boolean(PrimitiveProperty),
    Float32(PrimitiveProperty),
    Float64(PrimitiveProperty),
    Int32(PrimitiveProperty),
    Int64(PrimitiveProperty),
    Timestamp(PrimitiveProperty),
    Date(PrimitiveProperty),
    JSON(PrimitiveProperty),
    Direct(ViewDirectNodeRelation),
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewDirectNodeRelation {
    pub container: Option<SourceReference>,
    pub source: Option<SourceReference>,
}
