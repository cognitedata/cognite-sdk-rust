use serde::Serialize;

use crate::models::{instances::{NodeOrEdgeCreate, NodeOrEdge}, views::ViewReference};

pub mod files;

/// Node extension
pub trait IntoWritable<TProperties> {
    /// Try converting to node write
    fn try_into_writable(self, view: ViewReference) -> crate::Result<NodeOrEdgeCreate<TProperties>>;
}

pub trait FromReadable<TProperties>
where
    TProperties: Serialize + Send,
    Self: Sized,
{
    fn try_from_readable(value: NodeOrEdge<TProperties>, view: ViewReference) -> crate::Result<Self>;
}
