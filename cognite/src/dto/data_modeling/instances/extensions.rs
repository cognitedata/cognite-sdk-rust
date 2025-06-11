use serde::{de::DeserializeOwned, Serialize};

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
pub trait FromReadable<TProperties>: Sized
where
    TProperties: Serialize + Send,
    Self: WithView,
{
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
pub trait WithInstance<TProperties>
where
    TProperties: Serialize + Send,
{
    /// Generate a new instance of this type.
    fn instance(self) -> NodeOrEdgeCreate<TProperties>;
}

#[derive(Clone, Debug, Default)]
/// Cognite extendable type. This is a data model convenience that allows ease of use of the
/// `cognite_client.models.instances.apply(...)` method.
pub struct CogniteExtendable<TProperties>
where
    TProperties: Serialize + DeserializeOwned + Send + Sync,
{
    /// The where the instance belong. This can be none if the default view is preferred.
    pub view: Option<ViewReference>,
    /// Id of the instance.
    pub id: InstanceId,
    /// An audit of the lifecycle of the instance
    pub audit: CogniteAuditable,
    /// Properties of this type.
    pub properties: TProperties,
}

impl<TProperties> CogniteExtendable<TProperties>
where
    TProperties: Serialize + DeserializeOwned + Send + Sync + Default,
{
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

impl<TProperties> WithInstance<TProperties> for CogniteExtendable<TProperties>
where
    TProperties: Serialize + DeserializeOwned + Send + Sync,
    Self: WithView,
{
    fn instance(self) -> NodeOrEdgeCreate<TProperties> {
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

impl<TProperties> FromReadable<TProperties> for CogniteExtendable<TProperties>
where
    TProperties: Serialize + DeserializeOwned + Send + Sync + Default + Clone,
    Self: WithView,
{
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
                        space: Self::SPACE.to_string(),
                        external_id: Self::EXTERNAL_ID.to_string(),
                        version: Self::VERSION.to_string(),
                    }),
                    &mut properties,
                )
                .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteExtendable {
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
                    properties: extracted_properties.to_owned(),
                })
            }
            _ => Err(Error::Other("Invalid type".to_string())),
        }
    }
}
