use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    get_instance_properties,
    models::{
        instances::{EdgeOrNodeData, InstanceId, NodeOrEdge, NodeOrEdgeCreate, NodeWrite},
        views::ViewReference,
        SourceReference,
    },
    Error,
};

use super::{
    common::{CogniteAuditable, CogniteDescribable},
    FromReadable, WithInstance, WithView,
};

#[derive(Clone, Debug, Default)]
/// Represents a single unit of measurement.
pub struct CogniteUnit {
    /// The where the instance belong. This can be none if the default view is preferred.
    pub view: Option<ViewReference>,
    /// Id of the instance.
    pub id: InstanceId,
    /// An audit of the lifecycle of the instance
    pub audit: CogniteAuditable,
    /// Unit instance.
    pub properties: Unit,
}

impl CogniteUnit {
    /// Create a new instance of cognite timeseries.
    ///
    /// # Arguments
    ///
    /// * `space` - The space where this entity will be saved.
    /// * `external_id` - A unique external id for this entity.
    /// * `name` - A name for the entity.
    pub fn new(space: String, external_id: String, name: String) -> Self {
        CogniteUnit {
            id: InstanceId { space, external_id },
            view: None,
            properties: Unit::new(name),
            ..Default::default()
        }
    }
}

impl WithView for CogniteUnit {
    const SPACE: &'static str = "cdf_cdm";
    const EXTERNAL_ID: &'static str = "CogniteUnit";
    const VERSION: &'static str = "v1";
}

impl WithInstance<Unit> for CogniteUnit {
    fn instance(self) -> NodeOrEdgeCreate<Unit> {
        NodeOrEdgeCreate::Node(NodeWrite {
            space: self.id.space.to_owned(),
            external_id: self.id.external_id.to_owned(),
            existing_version: None,
            r#type: None,
            sources: Some(vec![EdgeOrNodeData {
                source: SourceReference::View(
                    self.view
                        .unwrap_or(ViewReference {
                            space: Self::SPACE.to_string(),
                            external_id: Self::EXTERNAL_ID.to_string(),
                            version: Self::VERSION.to_string(),
                        })
                        .to_owned(),
                ),
                properties: self.properties,
            }]),
        })
    }
}

impl FromReadable<Unit> for CogniteUnit {
    fn try_from(
        value: NodeOrEdge<Unit>,
        view: Option<&ViewReference>,
    ) -> crate::Result<CogniteUnit> {
        match value {
            NodeOrEdge::Node(node_definition) => {
                let mut properties = node_definition
                    .properties
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let unit: &Unit = get_instance_properties(
                    view.unwrap_or(&ViewReference {
                        space: Self::SPACE.to_string(),
                        external_id: Self::EXTERNAL_ID.to_string(),
                        version: Self::VERSION.to_string(),
                    }),
                    &mut properties,
                )
                .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteUnit {
                    view: view.map(|v| v.to_owned()),
                    id: InstanceId {
                        external_id: node_definition.external_id,
                        space: node_definition.space,
                    },
                    audit: CogniteAuditable {
                        created_time: node_definition.created_time,
                        last_updated_time: node_definition.last_updated_time,
                        deleted_time: node_definition.deleted_time,
                    },
                    properties: unit.to_owned(),
                })
            }
            _ => Err(Error::Other("Invalid type".to_string())),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// The properties of the file object.
pub struct Unit {
    #[serde(flatten)]
    /// Cognite describable.
    pub description: CogniteDescribable,
    /// The symbol for the unit of measurement.
    pub symbol: Option<String>,
    /// Specifies the physical quantity the unit measures.
    pub quantity: Option<String>,
    /// Source of the unit definition
    pub source: Option<String>,
    /// Reference to the source of the unit definition.
    pub source_reference: Option<String>,
}

impl Unit {
    /// Create a new timeseries instance.
    pub fn new(name: String) -> Self {
        Self {
            description: CogniteDescribable::new(name),
            ..Default::default()
        }
    }
}
