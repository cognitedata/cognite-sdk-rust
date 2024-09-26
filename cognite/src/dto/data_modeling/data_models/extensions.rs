use serde::Serialize;

use crate::models::{instances::NodeWrite, views::ViewReference};

pub mod files;

/// Node extension
pub trait IntoWritable<TProperties> {
    /// Try converting to node write
    fn try_into_writable(self, view: ViewReference) -> crate::Result<NodeWrite<TProperties>>;
}

pub trait FromReadable<TReadable>
where
    TReadable: Serialize + Send,
    Self: Sized,
{
    fn try_from_node_definition(value: TReadable, view: ViewReference) -> crate::Result<Self>;
}
