use serde::Serialize;

use crate::{
    get_instance_properties,
    models::{instances::NodeOrEdge, views::ViewReference, SourceReference},
    Error,
};

use super::{EdgeOrNodeData, InstanceId, NodeOrEdgeCreate, NodeWrite};

mod common;
pub use common::*;
mod extractors;
pub use extractors::*;
mod timeseries;
pub use timeseries::*;
mod units;
pub use units::*;

/// Trait to convert from Node/Edge into this type.
pub trait FromReadable<TProperties>: Sized {
    /// Try converting from node/edge into this type.
    ///
    /// # Arguments
    ///
    /// * `value` - Node/edge to retrieved.
    /// * `view` - A view reference representing the source of this type.
    fn try_from(
        value: NodeOrEdge<TProperties>,
        view: Option<&ViewReference>,
    ) -> crate::Result<Self>;
}

/// Trait for data models special instance
pub trait WithView {
    /// Default space.
    const SPACE: &'static str;
    /// Default external ID.
    const EXTERNAL_ID: &'static str;
    /// Default version.
    const VERSION: &'static str;
}

/// Trait to generate a `NodeWrite` instance for this type.
pub trait WithInstance {
    /// The properties of new instances of this type.
    type Properties: Serialize;

    /// Generate a new instance of this type.
    fn instance(self) -> NodeOrEdgeCreate<Self::Properties>;
}

#[derive(Clone, Debug, Default)]
/// Cognite extendable type. This is a data model convenience that allows ease of use of the
/// `cognite_client.models.instances.apply(...)` method.
pub struct CogniteExtendable<TProperties> {
    /// The where the instance belong. This can be none if the default view is preferred.
    pub view: Option<ViewReference>,
    /// Id of the instance.
    pub id: InstanceId,
    /// An audit of the lifecycle of the instance
    pub audit: CogniteAuditable,
    /// Properties of this type.
    pub properties: TProperties,
}

impl<TProperties: Default> CogniteExtendable<TProperties> {
    /// Create a new instance of this type.
    pub fn new(space: String, external_id: String, properties: TProperties) -> Self {
        CogniteExtendable {
            id: InstanceId { space, external_id },
            view: None,
            properties,
            ..Default::default()
        }
    }
}

impl<TProperties: WithView> WithView for CogniteExtendable<TProperties> {
    const SPACE: &'static str = TProperties::SPACE;
    const EXTERNAL_ID: &'static str = TProperties::EXTERNAL_ID;
    const VERSION: &'static str = TProperties::VERSION;
}

impl<TProperties: Serialize + WithView> WithInstance for CogniteExtendable<TProperties> {
    type Properties = TProperties;

    fn instance(self) -> NodeOrEdgeCreate<TProperties> {
        NodeOrEdgeCreate::Node(NodeWrite {
            space: self.id.space.clone(),
            external_id: self.id.external_id.clone(),
            existing_version: None,
            r#type: None,
            sources: Some(vec![EdgeOrNodeData {
                source: SourceReference::View(
                    self.view
                        .unwrap_or(ViewReference {
                            space: TProperties::SPACE.to_string(),
                            external_id: TProperties::EXTERNAL_ID.to_string(),
                            version: TProperties::VERSION.to_string(),
                        })
                        .clone(),
                ),
                properties: self.properties,
            }]),
        })
    }
}

impl<TProperties: Clone + WithView> FromReadable<TProperties> for CogniteExtendable<TProperties> {
    fn try_from(
        value: NodeOrEdge<TProperties>,
        view: Option<&ViewReference>,
    ) -> crate::Result<CogniteExtendable<TProperties>> {
        match value {
            NodeOrEdge::Node(node_definition) => {
                let mut properties = node_definition
                    .properties
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let extracted_properties = get_instance_properties(
                    view.unwrap_or(&ViewReference {
                        space: TProperties::SPACE.to_string(),
                        external_id: TProperties::EXTERNAL_ID.to_string(),
                        version: TProperties::VERSION.to_string(),
                    }),
                    &mut properties,
                )
                .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteExtendable {
                    view: view.cloned(),
                    id: InstanceId {
                        external_id: node_definition.external_id,
                        space: node_definition.space,
                    },
                    audit: CogniteAuditable {
                        created_time: node_definition.created_time,
                        last_updated_time: node_definition.last_updated_time,
                        deleted_time: node_definition.deleted_time,
                    },
                    properties: extracted_properties.clone(),
                })
            }
            _ => Err(Error::Other("Invalid type".to_string())),
        }
    }
}
