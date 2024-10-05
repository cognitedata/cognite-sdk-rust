use serde::Serialize;

use crate::models::{instances::NodeOrEdge, views::ViewReference};

use super::NodeOrEdgeCreate;

pub mod common;
pub mod files;
pub mod timeseries;
pub mod units;

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
}

pub trait WithInstance<TProperties>
where
    TProperties: Serialize + Send,
{
    fn instance(self) -> NodeOrEdgeCreate<TProperties>;
}

impl<T, TProperties> From<T> for NodeOrEdgeCreate<TProperties>
where
    T: WithView + WithInstance<TProperties>,
    TProperties: Serialize + Send,
{
    fn from(value: T) -> NodeOrEdgeCreate<TProperties> {
        value.instance()
    }
}

// pub enum InstanceType {
//     NODE,
//     EDGE,
// }
// pub trait WithType {
//     const TYPE: InstanceType;
// }

// impl<'a, V, T> IntoWritable<V> for T
// where
//     // Self: WithType + WithView<'a>,
//     T: WithType + WithView<'a>, // + FromReadable<dyn Serialize>,
//     V: Serialize + Send,
// {
//     fn try_into_writable(self) -> crate::Result<V> {
//         todo!()
//         // Ok(NodeOrEdgeCreate::Node(NodeWrite {
//         //     space: self.id.space.to_owned(),
//         //     external_id: self.id.external_id.to_owned(),
//         //     existing_version: None,
//         //     r#type: None,
//         //     sources: Some(vec![EdgeOrNodeData {
//         //         source: SourceReference::View(
//         //             self.view()
//         //                 .unwrap_or(ViewReference {
//         //                     space: T::SPACE.to_string(),
//         //                     external_id: T::EXTERNAL_ID.to_string(),
//         //                     version: T::VERSION.to_string(),
//         //                 })
//         //                 .to_owned(),
//         //         ),
//         //         properties: self.properties(),
//         //     }]),
//         // }))
//     }
// }
