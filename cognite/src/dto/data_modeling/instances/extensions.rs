use serde::Serialize;

use crate::models::{instances::NodeOrEdge, views::ViewReference};

use super::NodeOrEdgeCreate;

pub mod common;
pub mod files;
pub mod timeseries;
pub mod units;

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

pub trait WithInstance<TProperties>
where
    TProperties: Serialize + Send,
{
    fn instance(self) -> NodeOrEdgeCreate<TProperties>;
}

impl<TEntity, TProperties> From<TEntity> for NodeOrEdgeCreate<TProperties>
where
    TEntity: WithView + WithInstance<TProperties>,
    TProperties: Serialize + Send,
{
    fn from(value: TEntity) -> NodeOrEdgeCreate<TProperties> {
        value.instance()
    }
}
