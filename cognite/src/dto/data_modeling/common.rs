use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::dto::data_modeling::containers::ContainerReference;
use crate::models::FilterValueDefinition::{RawPropertyValue, ReferencedPropertyValue};
use crate::models::ViewReference;

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

// filter values
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum FilterValueDefinition {
    ReferencedPropertyValue { property: Vec<String> },
    RawPropertyValue(RawPropertyFilterValue),
}

impl FilterValueDefinition {
    pub fn referenced_prop(property: Vec<String>) -> FilterValueDefinition {
        ReferencedPropertyValue { property }
    }
    pub fn str(s: String) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::Comparable(
            ComparableFilterValue::String(s),
        ))
    }
    pub fn int(i: i64) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::Comparable(
            ComparableFilterValue::Int(i),
        ))
    }
    pub fn double(d: f64) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::Comparable(
            ComparableFilterValue::Double(d),
        ))
    }
    pub fn bool(b: bool) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::Logical(LogicalFilterValue::Bool(b)))
    }
    pub fn obj(o: serde_json::Value) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::Object(o))
    }

    pub fn str_list(l: Vec<String>) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::List(SeqFilterValue::StringList(l)))
    }
    pub fn int_list(l: Vec<i64>) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::List(SeqFilterValue::IntList(l)))
    }
    pub fn double_list(l: Vec<f64>) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::List(SeqFilterValue::DoubleList(l)))
    }
    pub fn bool_list(l: Vec<bool>) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::List(SeqFilterValue::BoolList(l)))
    }
    pub fn obj_list(l: Vec<serde_json::Value>) -> FilterValueDefinition {
        RawPropertyValue(RawPropertyFilterValue::List(SeqFilterValue::ObjectList(l)))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum RawPropertyFilterValue {
    Comparable(ComparableFilterValue),
    Logical(LogicalFilterValue),
    List(SeqFilterValue),
    Object(serde_json::Value),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ComparableFilterValue {
    String(String),
    Int(i64),
    Double(f64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum LogicalFilterValue {
    Bool(bool),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SeqFilterValue {
    StringList(Vec<String>),
    IntList(Vec<i64>),
    DoubleList(Vec<f64>),
    ObjectList(Vec<serde_json::Value>),
    BoolList(Vec<bool>),
}

// filter definitions
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum FilterDefinition {
    Logic(Box<LogicalFilter>),
    Leaf(Box<LeafFilter>),
}

impl FilterDefinition {
    // logical filters
    pub fn and(f: Vec<FilterDefinition>) -> FilterDefinition {
        FilterDefinition::Logic(Box::new(LogicalFilter::And(f)))
    }
    pub fn or(f: Vec<FilterDefinition>) -> FilterDefinition {
        FilterDefinition::Logic(Box::new(LogicalFilter::Or(f)))
    }
    pub fn not(f: FilterDefinition) -> FilterDefinition {
        FilterDefinition::Logic(Box::new(LogicalFilter::Not(f)))
    }

    // leaf filters
    pub fn equals(property: Vec<String>, value: FilterValueDefinition) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Equals { property, value }))
    }
    pub fn in_(property: Vec<String>, value: SeqFilterValue) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::In { property, value }))
    }
    pub fn prefix(property: Vec<String>, value: FilterValueDefinition) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Prefix { property, value }))
    }
    pub fn exists(property: Vec<String>) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Exists { property }))
    }
    pub fn contains_any(property: Vec<String>, values: SeqFilterValue) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::ContainsAny { property, values }))
    }
    pub fn contains_all(property: Vec<String>, values: SeqFilterValue) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::ContainsAll { property, values }))
    }
    pub fn nested(scope: Vec<String>, value: FilterValueDefinition) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Nested { scope, value }))
    }
    pub fn overlaps(
        start_property: Vec<String>,
        end_property: Vec<String>,
        gte: Option<ComparableFilterValue>,
        gt: Option<ComparableFilterValue>,
        lte: Option<ComparableFilterValue>,
        lt: Option<ComparableFilterValue>,
    ) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Overlaps {
            start_property,
            end_property,
            gte,
            gt,
            lte,
            lt,
        }))
    }
    pub fn has_data(refs: Vec<SourceReference>) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::HasData { refs }))
    }
    pub fn match_all() -> FilterDefinition {
        let empty_map = serde_json::Map::new();
        FilterDefinition::Leaf(Box::new(LeafFilter::MatchAll {
            value: serde_json::Value::from(empty_map),
        }))
    }
    pub fn range(
        property: Vec<String>,
        gte: Option<ComparableFilterValue>,
        gt: Option<ComparableFilterValue>,
        lte: Option<ComparableFilterValue>,
        lt: Option<ComparableFilterValue>,
    ) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Range {
            property,
            gte,
            gt,
            lte,
            lt,
        }))
    }
    pub fn gte(property: Vec<String>, v: ComparableFilterValue) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Range {
            property,
            gte: Some(v),
            gt: None,
            lte: None,
            lt: None,
        }))
    }
    pub fn gt(property: Vec<String>, v: ComparableFilterValue) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Range {
            property,
            gt: Some(v),
            gte: None,
            lte: None,
            lt: None,
        }))
    }
    pub fn lte(property: Vec<String>, v: ComparableFilterValue) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Range {
            property,
            gte: None,
            gt: None,
            lte: Some(v),
            lt: None,
        }))
    }
    pub fn lt(property: Vec<String>, v: ComparableFilterValue) -> FilterDefinition {
        FilterDefinition::Leaf(Box::new(LeafFilter::Range {
            property,
            gt: None,
            gte: None,
            lte: None,
            lt: Some(v),
        }))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LogicalFilter {
    And(Vec<FilterDefinition>),
    Or(Vec<FilterDefinition>),
    Not(FilterDefinition),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LeafFilter {
    Equals {
        property: Vec<String>,
        value: FilterValueDefinition,
    },
    In {
        property: Vec<String>,
        value: SeqFilterValue,
    },
    Range {
        property: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gte: Option<ComparableFilterValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gt: Option<ComparableFilterValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        lte: Option<ComparableFilterValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        lt: Option<ComparableFilterValue>,
    },
    Prefix {
        property: Vec<String>,
        value: FilterValueDefinition,
    },
    Exists {
        property: Vec<String>,
    },
    ContainsAny {
        property: Vec<String>,
        values: SeqFilterValue,
    },
    ContainsAll {
        property: Vec<String>,
        values: SeqFilterValue,
    },
    Nested {
        scope: Vec<String>,
        value: FilterValueDefinition,
    },
    #[serde(rename_all = "camelCase")]
    Overlaps {
        start_property: Vec<String>,
        end_property: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gte: Option<ComparableFilterValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gt: Option<ComparableFilterValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        lte: Option<ComparableFilterValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        lt: Option<ComparableFilterValue>,
    },
    HasData {
        refs: Vec<SourceReference>,
    },
    MatchAll {
        value: serde_json::Value,
    },
}
