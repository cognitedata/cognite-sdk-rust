use serde::Serialize;

use crate::models::{
    instances::{NodeOrEdge, NodeOrEdgeCreate},
    views::ViewReference,
};

pub mod common;
pub mod files;
pub mod timeseries;
pub mod units;

/// Trait to convert this type into a writeable node
pub trait IntoWritable<TProperties> {
    /// Try converting to writeable node/edge.
    fn try_into_writable(self) -> crate::Result<NodeOrEdgeCreate<TProperties>>;
}

/// Trait to convert from Node/Edge into this type.
pub trait FromReadable<TProperties>
where
    TProperties: Serialize + Send,
    Self: Sized,
{
    /// Try converting from node/edge into this type.
    ///
    /// # Arguments
    ///
    /// * `value` - Node/edge to retrieved.
    /// * `view` - A view reference representing the source of this type.
    fn try_from_readable(
        value: NodeOrEdge<TProperties>,
        view: Option<&ViewReference>,
    ) -> crate::Result<Self>;
}

/// Trait for data models special instance
pub trait WithView {
    /// Default space
    const SPACE: &'static str;
    /// Default external ID
    const EXTERNAL_ID: &'static str;
    /// Default version
    const VERSION: &'static str;
    // / Get view for instance
    // fn view(&self) -> ViewReference;
    // fn with_view(&mut self, view: ViewReference);
}
