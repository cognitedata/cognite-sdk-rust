use serde::Serialize;

use crate::models::{
    instances::{NodeOrEdge, NodeOrEdgeCreate},
    views::ViewReference
};

pub mod files;

/// Trait to convert this type into a writeable node
pub trait IntoWritable<TProperties> {
    /// Try converting to writeable node/edge.
    ///
    /// # Arguments
    ///
    /// `view` - A view reference representing the source of this type.
    fn try_into_writable(self, view: ViewReference)
        -> crate::Result<NodeOrEdgeCreate<TProperties>>;
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
        view: ViewReference,
    ) -> crate::Result<Self>;
}
