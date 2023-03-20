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
    pub space: String,
    pub external_id: String,
    pub version: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub properties: CreateViewPropertyOrConnectionDefinition,
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
    pub properties: ViewDefinitionProperties,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ViewDefinitionProperties {
    ViewCorePropertyDefinition(ViewCorePropertyDefinition),
    ConnectionDefinition(ConnectionDefinition),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewCorePropertyDefinition {
    #[derivative(Default(value = "true"))]
    pub nullable: Option<bool>,
    pub auto_increment: Option<bool>,
    pub default_value: Option<DefaultValue>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub r#type: ViewCorePropertyType,
    pub container: SourceReference,
    pub container_property_identifier: String,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ViewCorePropertyType {
    TextProperty(TextProperty),
    PrimitiveProperty(PrimitiveProperty),
    ViewDirectNodeRelation(ViewDirectNodeRelation),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextProperty {
    pub r#type: String,
    pub list: Option<bool>,
    pub collation: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrimitiveProperty {
    pub r#type: PrimitivePropertyType,
    pub list: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewDirectNodeRelation {
    pub r#type: String,
    pub container: Option<SourceReference>,
    pub source: Option<SourceReference>,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PrimitivePropertyType {
    Boolean,
    Float32,
    Float64,
    Int32,
    Int64,
    Timestamp,
    Date,
    JSON,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DefaultValue {
    String,
    Number,
    Boolean,
    Object,
}
