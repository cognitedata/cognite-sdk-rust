use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::dto::data_modeling::containers::ContainerReference;
use crate::models::FilterValueDefinition::{RawPropertyValue, ReferencedPropertyValue};
use crate::models::ViewReference;
use crate::sequences::SequenceFilter;

pub trait AsReference {
    fn to_reference(&self) -> SourceReference;
    fn to_instance_source(&self) -> InstanceSource {
        InstanceSource {
            source: self.to_reference(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemIdOptionalVersion {
    pub space: String,
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SourceReference {
    View(ViewReference),
    Container(ContainerReference),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct InstanceSource {
    pub source: SourceReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UsedFor {
    Node,
    Edge,
    All,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum DefaultValue {
    String(String),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Boolean(bool),
    Object(serde_json::Value),
}

#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum CorePropertyType {
    Text(TextProperty),
    Boolean(PrimitiveProperty),
    Float32(PrimitiveProperty),
    Float64(PrimitiveProperty),
    Int32(PrimitiveProperty),
    Int64(PrimitiveProperty),
    Timestamp(PrimitiveProperty),
    Date(PrimitiveProperty),
    JSON(PrimitiveProperty),
    Direct(DirectNodeRelation),
    TimeSeries,
    File,
    Sequence,
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
pub struct DirectNodeRelation {
    pub container: Option<SourceReference>,
}
