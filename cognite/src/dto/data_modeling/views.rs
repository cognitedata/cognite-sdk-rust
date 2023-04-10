use std::collections::HashMap;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::CreateViewPropertyOrConnectionDefinition::{Connection, Property};
use crate::models::{
    AsReference, ContainerReference, CorePropertyType, DefaultValue, DirectRelationReference,
};
use crate::{models::SourceReference, to_query, AsParams};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", rename = "view")]
pub struct ViewReference {
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
    pub properties: Option<HashMap<String, CreateViewPropertyOrConnectionDefinition>>,
}

impl AsReference for ViewCreateDefinition {
    fn to_reference(&self) -> SourceReference {
        let view_ref = ViewReference {
            space: self.space.to_owned(),
            external_id: self.external_id.to_owned(),
            version: self.version.to_owned(),
        };
        SourceReference::View(view_ref)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum CreateViewPropertyOrConnectionDefinition {
    Property(CreateViewProperty),
    Connection(ConnectionDefinition),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateViewProperty {
    pub name: Option<String>,
    pub description: Option<String>,
    pub container: ContainerReference,
    pub container_property_identifier: String,
    pub source: Option<ViewReference>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDefinition {
    pub name: Option<String>,
    pub description: Option<String>,
    pub r#type: DirectRelationReference,
    pub direction: Option<ConnectionDirection>,
    pub source: ViewReference,
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

impl AsReference for ViewDefinition {
    fn to_reference(&self) -> SourceReference {
        let view_ref = ViewReference {
            space: self.space.to_owned(),
            external_id: self.external_id.to_owned(),
            version: self.version.to_owned(),
        };
        SourceReference::View(view_ref)
    }
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
    pub r#type: CorePropertyType,
    pub container: SourceReference,
    pub container_property_identifier: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectNodeRelation {
    pub container: Option<SourceReference>,
    pub source: Option<SourceReference>,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionDirection {
    Outwards,
    Inwards,
}
