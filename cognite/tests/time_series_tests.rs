#![cfg(feature = "integration_tests")]

#[cfg(test)]
mod common;
pub use common::*;

use cognite::time_series::*;
use cognite::*;
use models::{
    instances::{
        CogniteTimeseries, InstanceId, NodeOrEdgeSpecification, SlimNodeOrEdge, TimeSeriesType,
        Timeseries,
    },
    ItemId,
};

#[tokio::test]
async fn create_and_delete_time_series() {
    let id = format!("{}-ts1", PREFIX.as_str());
    let time_serie = TimeSeries {
        name: Some("name".to_string()),
        external_id: Some(id),
        is_string: false,
        is_step: true,
        description: Some("description".to_string()),
        ..Default::default()
    };
    let client = get_client();
    let mut time_series = client
        .time_series
        .create_from(&vec![time_serie])
        .await
        .unwrap();
    assert_eq!(time_series.len(), 1);
    for time_serie in time_series.iter_mut() {
        time_serie.description = Some(String::from("changed"));
    }

    let time_series = client.time_series.update_from(&time_series).await.unwrap();

    let id_list: Vec<Identity> = time_series
        .iter()
        .map(|ts| Identity::Id { id: ts.id })
        .collect();
    client.time_series.delete(&id_list, true).await.unwrap();
}

#[tokio::test]
async fn create_retrieve_and_delete_dm_time_series_with_classic() {
    let _permit = CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();
    let external_id = uuid::Uuid::new_v4().to_string();
    let client = get_client();

    let timeseries = CogniteTimeseries::new(
        space.to_string(),
        external_id.to_string(),
        Timeseries {
            ..Default::default()
        },
    );
    let timeseries_res = client
        .models
        .instances
        .apply(&[timeseries], None, None, None, None, false)
        .await
        .unwrap();
    let timeseries_res = timeseries_res.first().unwrap();
    assert!(matches!(timeseries_res, SlimNodeOrEdge::Node(_)));

    let time_series_classic = client
        .time_series
        .retrieve(
            &[IdentityOrInstance::InstanceId {
                instance_id: InstanceId {
                    space: space.to_string(),
                    external_id: external_id.to_string(),
                },
            }],
            false,
        )
        .await
        .unwrap();
    let time_series = time_series_classic.first().unwrap().to_owned();
    let time_series_instance_id = InstanceId {
        space: space.to_string(),
        external_id: external_id.to_string(),
    };
    assert!(
        matches!(time_series.instance_id.clone(), Some(instance_id) if instance_id == time_series_instance_id.clone())
    );
    let data_sets = client
        .data_sets
        .retrieve(
            &[Identity::ExternalId {
                external_id: std::env::var("COGNITE_DATA_SET_EXTERNAL_ID")
                    .unwrap()
                    .to_string(),
            }],
            false,
        )
        .await
        .unwrap();
    let time_series_patch = PatchTimeSeries {
        data_set_id: Some(UpdateSetNull::Set {
            set: data_sets.first().unwrap().id,
        }),
        ..Default::default()
    };

    client
        .time_series
        .update(&[Patch {
            id: IdentityOrInstance::InstanceId {
                instance_id: time_series_instance_id,
            },
            update: time_series_patch,
        }])
        .await
        .unwrap();

    let _ = client
        .models
        .instances
        .delete(&[NodeOrEdgeSpecification::Node(ItemId {
            space: space.to_string(),
            external_id: external_id.to_string(),
        })])
        .await
        .unwrap();
}

#[tokio::test]
async fn create_and_delete_missing() {
    let _permit = CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();
    let external_id_classic = uuid::Uuid::new_v4().to_string();
    let external_id_cdm = uuid::Uuid::new_v4().to_string();
    let add_datapoints = vec![
        AddDatapoints {
            id: IdentityOrInstance::InstanceId {
                instance_id: InstanceId {
                    space: space.to_string(),
                    external_id: external_id_cdm.to_string(),
                },
            },
            datapoints: DatapointsEnumType::StringDatapoints(vec![DatapointString {
                timestamp: 1,
                value: Some("0.5".to_string()),
                status: None,
            }]),
        },
        AddDatapoints {
            id: IdentityOrInstance::Identity(Identity::ExternalId {
                external_id: external_id_classic.to_string(),
            }),
            datapoints: DatapointsEnumType::NumericDatapoints(vec![DatapointDouble {
                timestamp: 1,
                value: Some(0.5),
                status: None,
            }]),
        },
    ];
    let client = get_client();
    client
        .time_series
        .insert_datapoints_create_missing(add_datapoints, &|id_or_instance| {
            id_or_instance
                .iter()
                .map(|v| match v {
                    IdentityOrInstance::Identity(Identity::ExternalId { external_id }) => {
                        AddDmOrTimeSeries::TimeSeries(Box::new(AddTimeSeries {
                            external_id: Some(external_id.to_string()),
                            ..Default::default()
                        }))
                    }
                    IdentityOrInstance::InstanceId { instance_id } => {
                        let mut timeseries = CogniteTimeseries::new(
                            instance_id.space.to_string(),
                            instance_id.external_id.to_string(),
                            Timeseries {
                                ..Default::default()
                            },
                        );
                        timeseries.properties.r#type = TimeSeriesType::String;
                        AddDmOrTimeSeries::Cdm(Box::new(timeseries))
                    }
                    _ => panic!("Invalid identity received."),
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .await
        .unwrap();

    client
        .time_series
        .delete(
            &[Identity::ExternalId {
                external_id: external_id_classic,
            }],
            false,
        )
        .await
        .unwrap();
    let _ = client
        .models
        .instances
        .delete(&[NodeOrEdgeSpecification::Node(ItemId {
            space: space.to_string(),
            external_id: external_id_cdm.to_string(),
        })])
        .await
        .unwrap();
}
