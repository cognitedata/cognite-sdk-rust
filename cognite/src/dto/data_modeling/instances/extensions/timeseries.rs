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
    common::{CogniteAuditable, CogniteDescribable, CogniteSourceable},
    FromReadable, IntoWritable, WithView,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "lowercase")]
/// Specifies the data type of the data points.
pub enum TimeSeriesType {
    /// Indicates that timeseries type is a string.
    String,
    #[default]
    /// Indicates that timeseries type is a number.
    Numeric,
}

#[derive(Clone, Debug, Default)]
/// Represents a series of data points in time order..
pub struct CogniteTimeseries {
    /// The where the instance belong. This can be none if the default view is preferred.
    pub view: Option<ViewReference>,
    /// Id of the instance.
    pub id: InstanceId,
    /// An audit of the lifecycle of the instance
    pub audit: CogniteAuditable,
    /// Timeseries data.
    pub timeseries: Timeseries,
}

impl CogniteTimeseries {
    /// Create a new instance of cognite timeseries.
    ///
    /// # Arguments
    ///
    /// * `space` - The space where this entity will be saved.
    /// * `external_id` - A unique external id for this entity.
    /// * `name` - A name for the entity.
    /// # `is_step` - Specifies whether the time series is a step time series or not.
    pub fn new(space: String, external_id: String, name: String, is_step: Option<bool>) -> Self {
        CogniteTimeseries {
            id: InstanceId { space, external_id },
            view: None,
            timeseries: Timeseries::new(name, is_step),
            ..Default::default()
        }
    }
}

impl WithView for CogniteTimeseries {
    const SPACE: &'static str = "cdf_cdm";
    const EXTERNAL_ID: &'static str = "CogniteTimeSeries";
    const VERSION: &'static str = "v1";
}

impl IntoWritable<Timeseries> for CogniteTimeseries
where
    Self: WithView,
{
    fn try_into_writable(self) -> crate::Result<NodeOrEdgeCreate<Timeseries>> {
        Ok(NodeOrEdgeCreate::Node(NodeWrite {
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
                properties: self.timeseries,
            }]),
        }))
    }
}

impl FromReadable<Timeseries> for CogniteTimeseries {
    fn try_from_readable(
        value: NodeOrEdge<Timeseries>,
        view: Option<&ViewReference>,
    ) -> crate::Result<CogniteTimeseries> {
        match value {
            NodeOrEdge::Node(node_definition) => {
                let mut properties = node_definition
                    .properties
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let timeseries: &Timeseries = get_instance_properties(
                    view.unwrap_or(&ViewReference {
                        space: Self::SPACE.to_string(),
                        external_id: Self::EXTERNAL_ID.to_string(),
                        version: Self::VERSION.to_string(),
                    }),
                    &mut properties,
                )
                .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteTimeseries {
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
                    timeseries: timeseries.to_owned(),
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
pub struct Timeseries {
    #[serde(flatten)]
    /// Descriptions of the instance.
    pub description: CogniteDescribable,
    #[serde(flatten)]
    /// Source system.
    pub source: CogniteSourceable,
    /// Defines whether the time series is a step series or not.
    pub is_step: bool,
    /// Unit as specified in the source system.
    pub source_unit: String,
    /// Direct relation to the unit of the time series.
    pub unit: InstanceId,
    /// List of assets to which this file relates.
    pub assets: Option<Vec<InstanceId>>,
    /// List of activities associated with this time series.
    pub activities: Option<Vec<InstanceId>>,
    /// Type of datapoints the time series contains.
    pub r#type: TimeSeriesType,
}

impl Timeseries {
    /// Create a new timeseries instance.
    pub fn new(name: String, is_step: Option<bool>) -> Self {
        Self {
            description: CogniteDescribable::new(name),
            is_step: is_step.unwrap_or_default(),
            ..Default::default()
        }
    }
}
