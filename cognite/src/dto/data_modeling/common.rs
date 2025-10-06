use std::collections::HashMap;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{models::views::ViewReference, PropertyIdentifier};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// ID of a non-versioned resource in the data modeling API.
pub struct ItemId {
    /// Resource space.
    pub space: String,
    /// Resource external ID.
    pub external_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// ID of an optionally versioned resource in the data modelling API.
pub struct ItemIdOptionalVersion {
    /// Resource space.
    pub space: String,
    /// Resource external ID.
    pub external_id: String,
    /// Resurce version.
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
/// A reference to the source of a property.
pub enum SourceReference {
    /// A view with a specific version.
    View(ViewReference),
    /// A container.
    Container(ItemId),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
/// A reference to a property in a source view or container.
pub struct SourcePropertyReference {
    /// ID of the view or container.
    pub source: SourceReference,
    /// Identifier of the property in the source.
    pub identifier: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// ID of a space.
pub struct SpaceId {
    /// Space value
    pub space: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
/// A reference to a view, but tagged with `type: view`
pub enum TaggedViewReference {
    /// Tagged view variant
    View(ViewReference),
}

impl From<ViewReference> for TaggedViewReference {
    fn from(value: ViewReference) -> Self {
        Self::View(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
/// A reference to a container, but tagged with `type: container`
pub enum TaggedContainerReference {
    /// Tagged container reference.
    Container(ItemId),
}

impl TaggedContainerReference {
    /// Create a new tagged container reference.
    pub fn new(space: impl Into<String>, external_id: impl Into<String>) -> Self {
        Self::Container(ItemId {
            space: space.into(),
            external_id: external_id.into(),
        })
    }
}

impl From<ItemId> for TaggedContainerReference {
    fn from(value: ItemId) -> Self {
        Self::Container(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Enum for whether a resource is used for nodes, edges, or both.
pub enum UsedFor {
    #[default]
    /// Used for nodes.
    Node,
    /// Used for edges.
    Edge,
    /// Used for both nodes and edges.
    All,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Description of a text property.
pub struct TextProperty {
    #[derivative(Default(value = "false"))]
    /// Whether this is a list or not.
    pub list: Option<bool>,
    /// Optional text collation.
    pub collation: Option<String>,
    /// Maximum allowed length of the list.
    pub max_list_size: Option<i32>,
    /// Maximum allowed size of each text entry, as utf-8 bytes.
    pub max_text_size: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Reference to a unit in the Cognite unit catalog.
pub struct UnitReference {
    /// The external ID of the unit in the Cognite unit catalog.
    pub external_id: String,
    /// The value of the unit in the source.
    pub source_unit: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Description of a primitive property
pub struct PrimitiveProperty {
    #[derivative(Default(value = "false"))]
    /// Whether this is a list or not.
    pub list: Option<bool>,
    /// The unit of the data stored in this property.
    /// Can only be assigned to types float32 or float64,
    /// external ID needs to match with a unit in the Cognite unit catalog.
    pub unit: Option<UnitReference>,
    /// Maximum allowed length of the list.
    pub max_list_size: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Description of an enum value in an enum property.
pub struct EnumValueDescription {
    /// Name of the enum value.
    pub name: Option<String>,
    /// Enum value description.
    pub description: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Description of an enum property.
pub struct EnumProperty {
    /// The value to use when the enum value is unknown.
    pub unknown_value: Option<String>,
    /// Map from enum value identifier to description.
    pub values: HashMap<String, EnumValueDescription>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Derivative, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Description of a property referencing a CDF resource.
pub struct CDFExternalIdReference {
    #[derivative(Default(value = "false"))]
    /// Whether this is a list or not.
    pub list: Option<bool>,
    /// Maximum allowed length of the list.
    pub max_list_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// Direction to sort data modeling query results in.
pub enum SortDirection {
    #[default]
    /// Sort ascending.
    Ascending,
    /// Sort descending.
    Descending,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Sort on a dynamic property
pub struct PropertySort {
    /// List of strings representing the property
    pub property: Vec<String>,
    /// Direction to sort.
    pub direction: Option<SortDirection>,
    /// Whether nulls are first or last.
    pub nulls_first: Option<bool>,
}

impl PropertySort {
    /// Create a new property sort object.
    ///
    /// # Arguments
    ///
    /// * `property` - Property to sort by
    /// * `direction` - Direction to sort in.
    /// * `nulls_first` - Whether to put nulls first or last.
    pub fn new(
        property: impl PropertyIdentifier,
        direction: SortDirection,
        nulls_first: bool,
    ) -> Self {
        Self {
            property: property.into_identifier(),
            direction: Some(direction),
            nulls_first: Some(nulls_first),
        }
    }
}
