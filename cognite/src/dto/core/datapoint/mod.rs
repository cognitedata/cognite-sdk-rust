mod filter;
#[allow(clippy::all)]
mod proto {
    include!(concat!(
        env!("OUT_DIR"),
        "/com.cognite.v1.timeseries.proto.rs"
    ));
}

pub use self::filter::*;
pub use self::proto::data_point_insertion_item::DatapointType as InsertDatapointType;
pub use self::proto::data_point_insertion_item::IdOrExternalId;
pub use self::proto::data_point_list_item::DatapointType as ListDatapointType;
pub use self::proto::*;

use serde::{Deserialize, Serialize};

use crate::Identity;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum DatapointsEnumType {
    NumericDatapoints(Vec<DatapointDouble>),
    StringDatapoints(Vec<DatapointString>),
    AggregateDatapoints(Vec<DatapointAggregate>),
}

impl DatapointsEnumType {
    pub fn numeric(self) -> Option<Vec<DatapointDouble>> {
        match self {
            Self::NumericDatapoints(x) => Some(x),
            _ => None,
        }
    }

    pub fn string(self) -> Option<Vec<DatapointString>> {
        match self {
            Self::StringDatapoints(x) => Some(x),
            _ => None,
        }
    }

    pub fn aggregate(self) -> Option<Vec<DatapointAggregate>> {
        match self {
            Self::AggregateDatapoints(x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointDouble {
    pub timestamp: i64,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointString {
    pub timestamp: i64,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatapointAggregate {
    pub timestamp: i64,
    pub average: f64,
    pub max: f64,
    pub min: f64,
    pub count: f64,
    pub sum: f64,
    pub interpolation: f64,
    pub step_interpolation: f64,
    pub continuous_variance: f64,
    pub discrete_variance: f64,
    pub total_variation: f64,
}

impl From<NumericDatapoint> for DatapointDouble {
    fn from(dp: NumericDatapoint) -> DatapointDouble {
        DatapointDouble {
            timestamp: dp.timestamp,
            value: dp.value,
        }
    }
}

impl From<DatapointDouble> for NumericDatapoint {
    fn from(dp: DatapointDouble) -> NumericDatapoint {
        NumericDatapoint {
            timestamp: dp.timestamp,
            value: dp.value,
        }
    }
}

impl From<StringDatapoint> for DatapointString {
    fn from(dp: StringDatapoint) -> DatapointString {
        DatapointString {
            timestamp: dp.timestamp,
            value: dp.value,
        }
    }
}

impl From<DatapointString> for StringDatapoint {
    fn from(dp: DatapointString) -> StringDatapoint {
        StringDatapoint {
            timestamp: dp.timestamp,
            value: dp.value,
        }
    }
}

impl From<AggregateDatapoint> for DatapointAggregate {
    fn from(dp: AggregateDatapoint) -> DatapointAggregate {
        DatapointAggregate {
            timestamp: dp.timestamp,
            average: dp.average,
            max: dp.max,
            min: dp.min,
            count: dp.count,
            sum: dp.sum,
            interpolation: dp.interpolation,
            step_interpolation: dp.step_interpolation,
            continuous_variance: dp.continuous_variance,
            discrete_variance: dp.discrete_variance,
            total_variation: dp.total_variation,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatapointsListResponse {
    pub items: Vec<DatapointsResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatapointsResponse {
    pub id: i64,
    pub external_id: Option<String>,
    pub datapoints: DatapointsEnumType,
    pub unit: Option<String>,
    #[serde(default)]
    pub is_step: bool,
    #[serde(default)]
    pub is_string: bool,
}

pub struct AddDatapoints {
    pub id: Identity,
    pub datapoints: DatapointsEnumType,
}

impl AddDatapoints {
    pub fn new(time_serie_id: i64, datapoints: DatapointsEnumType) -> AddDatapoints {
        AddDatapoints {
            id: Identity::Id { id: time_serie_id },
            datapoints,
        }
    }
    pub fn new_external_id(external_id: &str, datapoints: DatapointsEnumType) -> AddDatapoints {
        AddDatapoints {
            id: Identity::ExternalId {
                external_id: external_id.to_string(),
            },
            datapoints,
        }
    }
}

impl From<Identity> for IdOrExternalId {
    fn from(idt: Identity) -> IdOrExternalId {
        match idt {
            Identity::Id { id } => IdOrExternalId::Id(id),
            Identity::ExternalId {
                external_id: ext_id,
            } => IdOrExternalId::ExternalId(ext_id),
        }
    }
}

impl From<IdOrExternalId> for Identity {
    fn from(idt: IdOrExternalId) -> Identity {
        match idt {
            IdOrExternalId::Id(id) => Identity::Id { id },
            IdOrExternalId::ExternalId(ext_id) => Identity::ExternalId {
                external_id: ext_id,
            },
        }
    }
}

impl From<DataPointListItem> for DatapointsResponse {
    fn from(req: DataPointListItem) -> DatapointsResponse {
        DatapointsResponse {
            id: req.id,
            external_id: if req.external_id.is_empty() {
                None
            } else {
                Some(req.external_id)
            },
            unit: if req.unit.is_empty() {
                None
            } else {
                Some(req.unit)
            },
            is_step: req.is_step,
            is_string: req.is_string,
            datapoints: match req.datapoint_type {
                Some(dps) => match dps {
                    data_point_list_item::DatapointType::NumericDatapoints(num_dps) => {
                        DatapointsEnumType::NumericDatapoints(
                            num_dps
                                .datapoints
                                .into_iter()
                                .map(DatapointDouble::from)
                                .collect(),
                        )
                    }
                    data_point_list_item::DatapointType::StringDatapoints(str_dps) => {
                        DatapointsEnumType::StringDatapoints(
                            str_dps
                                .datapoints
                                .into_iter()
                                .map(DatapointString::from)
                                .collect(),
                        )
                    }
                    data_point_list_item::DatapointType::AggregateDatapoints(aggr_dps) => {
                        DatapointsEnumType::AggregateDatapoints(
                            aggr_dps
                                .datapoints
                                .into_iter()
                                .map(DatapointAggregate::from)
                                .collect(),
                        )
                    }
                },
                None => DatapointsEnumType::NumericDatapoints(Vec::<DatapointDouble>::new()),
            },
        }
    }
}

impl From<AddDatapoints> for DataPointInsertionItem {
    fn from(req: AddDatapoints) -> DataPointInsertionItem {
        DataPointInsertionItem {
            id_or_external_id: Some(IdOrExternalId::from(req.id)),
            datapoint_type: match req.datapoints {
                DatapointsEnumType::NumericDatapoints(dps) => Some(
                    self::proto::data_point_insertion_item::DatapointType::NumericDatapoints(
                        NumericDatapoints {
                            datapoints: dps.into_iter().map(NumericDatapoint::from).collect(),
                        },
                    ),
                ),
                DatapointsEnumType::StringDatapoints(dps) => Some(
                    self::proto::data_point_insertion_item::DatapointType::StringDatapoints(
                        StringDatapoints {
                            datapoints: dps.into_iter().map(StringDatapoint::from).collect(),
                        },
                    ),
                ),
                DatapointsEnumType::AggregateDatapoints(_) => {
                    panic!("Cannot insert aggregate datapoints")
                }
            },
        }
    }
}

impl From<Vec<AddDatapoints>> for DataPointInsertionRequest {
    fn from(items: Vec<AddDatapoints>) -> DataPointInsertionRequest {
        DataPointInsertionRequest {
            items: items
                .into_iter()
                .map(DataPointInsertionItem::from)
                .collect(),
        }
    }
}

impl From<DataPointListResponse> for DatapointsListResponse {
    fn from(resp: DataPointListResponse) -> DatapointsListResponse {
        DatapointsListResponse {
            items: resp
                .items
                .into_iter()
                .map(DatapointsResponse::from)
                .collect(),
        }
    }
}
