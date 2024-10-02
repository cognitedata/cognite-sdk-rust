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
    FromReadable, IntoWritable,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum TimeSeriesType {
    String,
    #[default]
    Number,
}

#[derive(Clone, Debug, Default)]
/// A special data models instance type.
pub struct CogniteTimeseries {
    /// Id of the instance.
    pub id: InstanceId,
    /// Descriptions of the instance.
    pub description: CogniteDescribable,
    /// Source system.
    pub source: CogniteSourceable,
    /// Defines whether the time series is a step series or not.
    pub is_step: Option<bool>,
    /// Unit as specified in the source system.
    pub source_unit: String,
    /// Direct relation to the unit of the time series.
    pub unit: InstanceId,
    /// List of assets to which this file relates.
    pub assets: Option<Vec<InstanceId>>,
    /// List of activities associated with this time series.
    pub activities: Option<Vec<InstanceId>>,
    /// Type of datapoints the time series contains.
    pub r#type: Option<TimeSeriesType>,
    /// An audit of the lifecycle of the instance
    pub audit: CogniteAuditable,
}

impl CogniteTimeseries {
    /// Create a new instance of cognite timeseries.
    ///
    /// # Arguments
    ///
    /// * `space` - The space where this entity will be saved.
    /// * `external_id` - A unique external id for this entity.
    /// * `name` - A name for the entity.
    pub fn new(space: String, external_id: String, name: String) -> Self {
        CogniteTimeseries {
            description: CogniteDescribable {
                name,
                ..Default::default()
            },
            id: InstanceId { space, external_id },
            ..Default::default()
        }
    }
}

impl From<CogniteTimeseries> for Timeseries {
    fn from(value: CogniteTimeseries) -> Self {
        Self {
            description: value.description,
            source: value.source,
            assets: value.assets,
            is_step: value.is_step,
            activities: value.activities,
            unit: value.unit,
            source_unit: value.source_unit,
            r#type: value.r#type,
        }
    }
}

impl IntoWritable<Timeseries> for CogniteTimeseries {
    fn try_into_writable(self, view: ViewReference) -> crate::Result<NodeOrEdgeCreate<Timeseries>> {
        Ok(NodeOrEdgeCreate::Node(NodeWrite {
            space: self.id.space.to_owned(),
            external_id: self.id.external_id.to_owned(),
            existing_version: None,
            r#type: None,
            sources: Some(vec![EdgeOrNodeData {
                source: SourceReference::View(view),
                properties: self.into(),
            }]),
        }))
    }
}

impl FromReadable<Timeseries> for CogniteTimeseries {
    fn try_from_readable(
        value: NodeOrEdge<Timeseries>,
        view: ViewReference,
    ) -> crate::Result<CogniteTimeseries> {
        match value {
            NodeOrEdge::Node(node_definition) => {
                let mut properties = node_definition
                    .properties
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                let timeseries: &Timeseries = get_instance_properties(view, &mut properties)
                    .ok_or(Error::Other("Invalid properties".to_string()))?;
                Ok(CogniteTimeseries {
                    id: InstanceId {
                        external_id: node_definition.external_id,
                        space: node_definition.space,
                    },
                    description: timeseries.description.clone(),
                    source: timeseries.source.clone(),
                    audit: CogniteAuditable {
                        created_time: node_definition.created_time,
                        last_updated_time: node_definition.last_updated_time,
                        deleted_time: node_definition.deleted_time,
                    },
                    assets: timeseries.assets.clone(),
                    is_step: timeseries.is_step,
                    activities: timeseries.activities.clone(),
                    unit: timeseries.unit.clone(),
                    source_unit: timeseries.source_unit.clone(),
                    r#type: timeseries.r#type.clone(),
                })
            }
            _ => Err(Error::Other("Invalid type".to_string())),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub is_step: Option<bool>,
    /// Unit as specified in the source system.
    pub source_unit: String,
    /// Direct relation to the unit of the time series.
    pub unit: InstanceId,
    /// List of assets to which this file relates.
    pub assets: Option<Vec<InstanceId>>,
    /// List of activities associated with this time series.
    pub activities: Option<Vec<InstanceId>>,
    /// Type of datapoints the time series contains.
    pub r#type: Option<TimeSeriesType>,
}
