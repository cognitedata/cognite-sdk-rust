#![cfg(feature = "integration_tests")]

#[cfg(test)]
use cognite::time_series::*;
use cognite::*;
use models::instances::{
    CogniteExtendable, CogniteTimeseries, InstanceId, NodeAndEdgeCreateCollection,
    NodeOrEdgeCreate, NodeOrEdgeSpecification, TimeSeriesType, Timeseries, WithInstance, WithView,
};
use models::ItemId;

mod common;
pub use common::*;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct StateSetState {
    numeric_value: i32,
    string_value: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct CogniteStateSetProperties {
    name: Option<String>,
    states: Vec<StateSetState>,
}

impl WithView for CogniteStateSetProperties {
    const SPACE: &'static str = "cdf_cdm";
    const EXTERNAL_ID: &'static str = "CogniteStateSet";
    const VERSION: &'static str = "v1";
}

type CogniteStateSet = CogniteExtendable<CogniteStateSetProperties>;

async fn beta_upsert_instances<TProperties>(
    client: &CogniteClient,
    items: Vec<NodeOrEdgeCreate<TProperties>>,
) -> cognite::Result<()>
where
    TProperties: Serialize + Send + Sync,
{
    let collection = NodeAndEdgeCreateCollection {
        items,
        auto_create_direct_relations: Some(true),
        ..Default::default()
    };
    client
        .api_client
        .clone_with_api_version("beta")
        .post::<::serde_json::Value, _>("models/instances", &collection)
        .await?;
    Ok(())
}

async fn beta_delete_instances(
    client: &CogniteClient,
    items: &[NodeOrEdgeSpecification],
) -> cognite::Result<()> {
    client
        .api_client
        .clone_with_api_version("beta")
        .post::<::serde_json::Value, _>("models/instances/delete", &Items::new(items))
        .await?;
    Ok(())
}

async fn create_test_ts(client: &CogniteClient, is_string: bool, idx: i32) -> TimeSeries {
    let ts = AddTimeSeries {
        external_id: Some(format!("{}-ts-{}", PREFIX.as_str(), idx)),
        is_string,
        name: Some(format!("Test ts {idx}")),
        ..Default::default()
    };

    client
        .time_series
        .create(&vec![ts])
        .await
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

async fn delete_test_ts(client: &CogniteClient, id: i64) {
    client
        .time_series
        .delete(&[Identity::Id { id }], true)
        .await
        .unwrap();
}

#[tokio::test]
async fn create_retrieve_delete_double_datapoints() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let start = since_the_epoch.as_millis() as i64;

    let client = get_client();

    let ts = create_test_ts(&client, false, 1).await;

    // Create 100 dps
    client
        .time_series
        .insert_datapoints(vec![AddDatapoints {
            id: ts.id.into(),
            datapoints: DatapointsEnumType::NumericDatapoints(
                (0..100)
                    .map(|i| DatapointDouble {
                        timestamp: start + i * 1000,
                        value: Some(i as f64),
                        status: None,
                    })
                    .collect(),
            ),
        }])
        .await
        .unwrap();

    // Retrieve 90 of them
    let dps = client
        .time_series
        .retrieve_datapoints(&DatapointsFilter {
            items: vec![DatapointsQuery {
                id: ts.id.into(),
                ..Default::default()
            }],
            start: Some((start + 5000).into()),
            end: Some((start + 95000).into()),
            limit: Some(1000),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(1, dps.len());
    let resp = dps.into_iter().next().unwrap();
    assert_eq!(90, resp.datapoints.numeric().unwrap().len());

    // Delete half
    client
        .time_series
        .delete_datapoints(&[DeleteDatapointsQuery {
            inclusive_begin: start,
            exclusive_end: start + 50000,
            id: ts.id.into(),
        }])
        .await
        .unwrap();

    // Retrieve the same range and expect us to get 45 less
    let dps = client
        .time_series
        .retrieve_datapoints(&DatapointsFilter {
            items: vec![DatapointsQuery {
                id: ts.id.into(),
                ..Default::default()
            }],
            start: Some((start + 5000).into()),
            end: Some((start + 95000).into()),
            limit: Some(1000),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(1, dps.len());
    let resp = dps.into_iter().next().unwrap();
    assert_eq!(45, resp.datapoints.numeric().unwrap().len());

    delete_test_ts(&client, ts.id).await;
}

#[tokio::test]
async fn create_retrieve_delete_string_datapoints() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let start = since_the_epoch.as_millis() as i64;

    let client = get_client();

    let ts = create_test_ts(&client, true, 2).await;

    // Create 100 dps
    client
        .time_series
        .insert_datapoints(vec![AddDatapoints {
            id: IdentityOrInstance::Identity(Identity::Id { id: ts.id }),
            datapoints: DatapointsEnumType::StringDatapoints(
                (0..100)
                    .map(|i| DatapointString {
                        timestamp: start + i * 1000,
                        value: Some(format!("{i}-dp")),
                        status: None,
                    })
                    .collect(),
            ),
        }])
        .await
        .unwrap();

    // Retrieve 90 of them
    let dps = client
        .time_series
        .retrieve_datapoints(&DatapointsFilter {
            items: vec![DatapointsQuery {
                id: ts.id.into(),
                ..Default::default()
            }],
            start: Some((start + 5000).into()),
            end: Some((start + 95000).into()),
            limit: Some(1000),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(1, dps.len());
    let resp = dps.into_iter().next().unwrap();
    assert_eq!(90, resp.datapoints.string().unwrap().len());

    // Delete half
    client
        .time_series
        .delete_datapoints(&[DeleteDatapointsQuery {
            inclusive_begin: start,
            exclusive_end: start + 50000,
            id: ts.id.into(),
        }])
        .await
        .unwrap();

    // Retrieve the same range and expect us to get 45 less
    let dps = client
        .time_series
        .retrieve_datapoints(&DatapointsFilter {
            items: vec![DatapointsQuery {
                id: ts.id.into(),
                ..Default::default()
            }],
            start: Some((start + 5000).into()),
            end: Some((start + 95000).into()),
            limit: Some(1000),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(1, dps.len());
    let resp = dps.into_iter().next().unwrap();
    assert_eq!(45, resp.datapoints.string().unwrap().len());

    delete_test_ts(&client, ts.id).await;
}

#[tokio::test]
async fn create_retrieve_latest_delete_state_datapoints() {
    let space = std::env::var("CORE_DM_TEST_SPACE")
        .expect("CORE_DM_TEST_SPACE must be set to a writable data modeling space");
    let state_set_external_id = format!("{}-state-set", uuid::Uuid::new_v4());
    let ts_external_id = format!("{}-state-ts", uuid::Uuid::new_v4());
    let client = get_client();

    let state_set = CogniteStateSet::new(
        space.clone(),
        state_set_external_id.clone(),
        CogniteStateSetProperties {
            name: Some("Rust SDK state datapoint test states".to_string()),
            states: vec![
                StateSetState {
                    numeric_value: 0,
                    string_value: "OFF".to_string(),
                },
                StateSetState {
                    numeric_value: 1,
                    string_value: "RUNNING".to_string(),
                },
                StateSetState {
                    numeric_value: 2,
                    string_value: "STANDBY".to_string(),
                },
            ],
            ..Default::default()
        },
    );
    beta_upsert_instances(&client, vec![state_set.instance()])
        .await
        .unwrap();

    let mut timeseries =
        CogniteTimeseries::new(space.clone(), ts_external_id.clone(), Timeseries::new(true));
    timeseries.properties.r#type = TimeSeriesType::State;
    timeseries.properties.state_set = Some(InstanceId {
        space: space.clone(),
        external_id: state_set_external_id.clone(),
    });
    beta_upsert_instances(&client, vec![timeseries.instance()])
        .await
        .unwrap();

    let ts_instance_id = InstanceId {
        space: space.clone(),
        external_id: ts_external_id.clone(),
    };
    let start = 1_735_689_600_000;

    client
        .time_series
        .insert_datapoints(vec![AddDatapoints {
            id: ts_instance_id.clone().into(),
            datapoints: DatapointsEnumType::StateDatapoints(vec![
                DatapointState {
                    timestamp: start,
                    numeric_value: Some(0),
                    string_value: Some("OFF".to_string()),
                    status: None,
                },
                DatapointState {
                    timestamp: start + 3_600_000,
                    numeric_value: Some(1),
                    string_value: Some("RUNNING".to_string()),
                    status: None,
                },
                DatapointState {
                    timestamp: start + 7_200_000,
                    numeric_value: Some(2),
                    string_value: Some("STANDBY".to_string()),
                    status: None,
                },
            ]),
        }])
        .await
        .unwrap();

    let raw = client
        .time_series
        .retrieve_datapoints(&DatapointsFilter {
            items: vec![DatapointsQuery {
                id: ts_instance_id.clone().into(),
                limit: Some(10),
                ..Default::default()
            }],
            start: Some(start.into()),
            end: Some((start + 10_800_000).into()),
            ..Default::default()
        })
        .await
        .unwrap();

    let raw_response = raw.into_iter().next().unwrap();
    assert_eq!(Some(CoreTimeSeriesType::State), raw_response.r#type);
    assert_eq!(Some(ts_instance_id.clone()), raw_response.instance_id);
    let raw_states = raw_response.datapoints.state().unwrap();
    assert_eq!(3, raw_states.len());
    assert_eq!(Some(0), raw_states[0].numeric_value);
    assert_eq!(Some("OFF".to_string()), raw_states[0].string_value);
    assert_eq!(Some(1), raw_states[1].numeric_value);
    assert_eq!(Some("RUNNING".to_string()), raw_states[1].string_value);

    let latest = client
        .time_series
        .retrieve_latest_datapoints(
            &[LatestDatapointsQuery {
                id: ts_instance_id.clone().into(),
                before: Some((start + 10_800_000).to_string()),
                ..Default::default()
            }],
            false,
        )
        .await
        .unwrap();
    assert_eq!(Some(CoreTimeSeriesType::State), latest[0].r#type);
    assert_eq!(Some(ts_instance_id.clone()), latest[0].instance_id);
    let latest_state = latest[0].datapoint.as_ref().unwrap().state().unwrap();
    assert_eq!(Some(2), latest_state.numeric_value);
    assert_eq!(Some("STANDBY".to_string()), latest_state.string_value);

    client
        .time_series
        .delete_datapoints(&[DeleteDatapointsQuery::new(
            ts_instance_id.clone(),
            start,
            start + 10_800_000,
        )])
        .await
        .unwrap();

    let raw_after_delete = client
        .time_series
        .retrieve_datapoints(&DatapointsFilter {
            items: vec![DatapointsQuery {
                id: ts_instance_id.clone().into(),
                limit: Some(10),
                ..Default::default()
            }],
            start: Some(start.into()),
            end: Some((start + 10_800_000).into()),
            ..Default::default()
        })
        .await
        .unwrap();
    let deleted_states = raw_after_delete
        .into_iter()
        .next()
        .map(|response| response.datapoints.state().unwrap_or_default())
        .unwrap_or_default();
    assert!(deleted_states.is_empty());

    let _ = beta_delete_instances(
        &client,
        &[
            NodeOrEdgeSpecification::Node(ItemId {
                space: space.clone(),
                external_id: ts_external_id,
            }),
            NodeOrEdgeSpecification::Node(ItemId {
                space,
                external_id: state_set_external_id,
            }),
        ],
    )
    .await;
}

#[tokio::test]
async fn retrieve_latest() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let start = since_the_epoch.as_millis() as i64;

    let client = get_client();

    let ts = create_test_ts(&client, false, 3).await;

    // Create 100 dps
    client
        .time_series
        .insert_datapoints(vec![AddDatapoints {
            id: IdentityOrInstance::Identity(Identity::Id { id: ts.id }),
            datapoints: DatapointsEnumType::NumericDatapoints(
                (0..100)
                    .map(|i| DatapointDouble {
                        timestamp: start + i * 1000,
                        value: Some(i as f64),
                        status: None,
                    })
                    .collect(),
            ),
        }])
        .await
        .unwrap();

    let latest = client
        .time_series
        .retrieve_latest_datapoints(
            &[LatestDatapointsQuery {
                before: Some(format!("{}", start + 200_000)),
                id: Identity::Id { id: ts.id }.into(),
                ..Default::default()
            }],
            false,
        )
        .await
        .unwrap();

    assert_eq!(1, latest.len());
    let latest = latest.into_iter().next().unwrap();
    assert_eq!(
        99.0,
        latest.datapoint.unwrap().numeric().unwrap().value.unwrap()
    );

    delete_test_ts(&client, ts.id).await;
}

#[tokio::test]
async fn create_retrieve_double_datapoints_with_status() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let start = since_the_epoch.as_millis() as i64;

    let client = get_client();

    let ts = create_test_ts(&client, false, 4).await;

    client
        .time_series
        .insert_datapoints(vec![AddDatapoints {
            id: IdentityOrInstance::Identity(Identity::Id { id: ts.id }),
            datapoints: DatapointsEnumType::NumericDatapoints(vec![
                DatapointDouble {
                    timestamp: start + 1000,
                    value: Some(123.0),
                    status: Some(StatusCode::try_parse("GoodClamped").unwrap()),
                },
                DatapointDouble {
                    timestamp: start + 2000,
                    value: None,
                    status: Some(StatusCode::try_parse("Bad").unwrap()),
                },
                DatapointDouble {
                    timestamp: start + 3000,
                    value: Some(f64::NAN),
                    status: Some(StatusCode::try_parse("Bad").unwrap()),
                },
                DatapointDouble {
                    timestamp: start + 4000,
                    value: Some(f64::INFINITY),
                    status: Some(StatusCode::try_parse("Bad").unwrap()),
                },
            ]),
        }])
        .await
        .unwrap();

    let dps = client
        .time_series
        .retrieve_datapoints(&DatapointsFilter {
            items: vec![DatapointsQuery {
                id: ts.id.into(),
                include_status: Some(true),
                ignore_bad_data_points: Some(false),
                ..Default::default()
            }],
            start: Some(start.into()),
            end: Some((start + 5000).into()),
            ..Default::default()
        })
        .await
        .unwrap();

    let dpl = dps.into_iter().next().unwrap();
    let dpl = match dpl.datapoints {
        DatapointsEnumType::NumericDatapoints(d) => d,
        _ => unreachable!(),
    };

    assert_eq!(4, dpl.len());

    assert_eq!("GoodClamped", dpl[0].status.as_ref().unwrap().to_string());
    assert!(dpl[1].value.is_none());
    assert_eq!("Bad", dpl[1].status.as_ref().unwrap().to_string());
    assert!(dpl[2].value.unwrap().is_nan());
    assert_eq!("Bad", dpl[2].status.as_ref().unwrap().to_string());
    assert!(dpl[3].value.unwrap().is_infinite());
    assert_eq!("Bad", dpl[3].status.as_ref().unwrap().to_string());

    // Retrieve latest
    let latest = client
        .time_series
        .retrieve_latest_datapoints(
            &[LatestDatapointsQuery {
                before: Some((start + 10000).to_string()),
                include_status: Some(true),
                ignore_bad_data_points: Some(false),
                id: Identity::from(ts.id).into(),
                ..Default::default()
            }],
            false,
        )
        .await
        .unwrap();

    let dpl = latest.into_iter().next().unwrap();
    let dpl = match dpl.datapoint {
        Some(LatestDatapoint::Numeric(d)) => d,
        d => panic!("Unexpected variant {d:?}"),
    };

    assert_eq!("Bad", dpl.status.as_ref().unwrap().to_string());
    assert!(dpl.value.unwrap().is_infinite());

    delete_test_ts(&client, ts.id).await;
}

async fn stream_test_timeseries(client: &CogniteClient, idx: i32) -> (TimeSeries, TimeSeries) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();

    // Put the start sometime in the past...
    let start = since_the_epoch.as_millis() as i64 - 1_000_000;

    let ts = vec![
        AddTimeSeries {
            external_id: Some(format!("{}-ts-stream-{idx}-1", PREFIX.as_str())),
            is_string: false,
            name: Some(format!("Test ts stream 1")),
            ..Default::default()
        },
        AddTimeSeries {
            external_id: Some(format!("{}-ts-stream-{idx}-2", PREFIX.as_str())),
            is_string: true,
            name: Some(format!("Test ts stream 2")),
            ..Default::default()
        },
    ];

    let timeseries = client.time_series.create(&ts).await.unwrap();
    let ts1 = &timeseries[0];
    let ts2 = &timeseries[1];

    let client = get_client();
    // Create 50 dps in each timeseries
    client
        .time_series
        .insert_datapoints(vec![
            AddDatapoints {
                id: ts1.id.into(),
                datapoints: DatapointsEnumType::NumericDatapoints(
                    (0..50)
                        .map(|i| DatapointDouble {
                            timestamp: start + i * 1000,
                            value: Some(i as f64),
                            status: None,
                        })
                        .collect(),
                ),
            },
            AddDatapoints {
                id: ts2.id.into(),
                datapoints: DatapointsEnumType::StringDatapoints(
                    (0..50)
                        .map(|i| DatapointString {
                            timestamp: start + i * 1000,
                            value: Some(format!("{i}-dp")),
                            status: None,
                        })
                        .collect(),
                ),
            },
        ])
        .await
        .unwrap();

    (ts1.clone(), ts2.clone())
}

#[tokio::test]
async fn stream_datapoints() {
    let client = get_client();

    let (ts1, ts2) = stream_test_timeseries(&client, 1).await;

    // Stream them back
    let streamed = client
        .time_series
        .stream_datapoints(
            DatapointsFilter {
                items: vec![
                    DatapointsQuery {
                        id: ts1.id.into(),
                        limit: Some(30),
                        ..Default::default()
                    },
                    DatapointsQuery {
                        id: ts2.external_id.clone().unwrap().into(),
                        limit: Some(30),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            DatapointsStreamOptions {
                batch_size: 100,
                parallelism: 2,
            },
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(100, streamed.len());
    let mut count_ts1 = 0;
    let mut count_ts2 = 0;
    for dp in &streamed {
        if dp.id() == ts1.id {
            let v = dp.as_numeric().unwrap().value.unwrap() as i32;
            assert_eq!(v, count_ts1);
            count_ts1 += 1;
        } else if dp.id() == ts2.id {
            let v = dp.as_string().unwrap().value.as_ref().unwrap();
            assert_eq!(v, &format!("{}-dp", count_ts2));
            count_ts2 += 1;
        } else {
            panic!("Unexpected timeseries id {}", dp.id());
        }
    }
    assert_eq!(50, count_ts1);
    assert_eq!(50, count_ts2);

    // Cleanup
    client
        .time_series
        .delete(
            &[Identity::Id { id: ts1.id }, Identity::Id { id: ts2.id }],
            true,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn stream_datapoints_ignore_missing() {
    let client = get_client();

    let (ts1, ts2) = stream_test_timeseries(&client, 2).await;

    // Stream them back
    let streamed = client
        .time_series
        .stream_datapoints(
            DatapointsFilter {
                ignore_unknown_ids: Some(true),
                items: vec![
                    DatapointsQuery {
                        id: ts1.id.into(),
                        limit: Some(30),
                        ..Default::default()
                    },
                    DatapointsQuery {
                        id: Identity::ExternalId {
                            external_id: "non-existing-ts".to_string(),
                        }
                        .into(),
                        limit: Some(30),
                        ..Default::default()
                    },
                    DatapointsQuery {
                        id: ts2.external_id.clone().unwrap().into(),
                        limit: Some(30),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            DatapointsStreamOptions {
                batch_size: 100,
                parallelism: 2,
            },
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(100, streamed.len());
    let mut count_ts1 = 0;
    let mut count_ts2 = 0;
    for dp in &streamed {
        if dp.id() == ts1.id {
            let v = dp.as_numeric().unwrap().value.unwrap() as i32;
            assert_eq!(v, count_ts1);
            count_ts1 += 1;
        } else if dp.id() == ts2.id {
            let v = dp.as_string().unwrap().value.as_ref().unwrap();
            assert_eq!(v, &format!("{}-dp", count_ts2));
            count_ts2 += 1;
        } else {
            panic!("Unexpected timeseries id {}", dp.id());
        }
    }
    assert_eq!(50, count_ts1);
    assert_eq!(50, count_ts2);

    // Cleanup
    client
        .time_series
        .delete(
            &[Identity::Id { id: ts1.id }, Identity::Id { id: ts2.id }],
            true,
        )
        .await
        .unwrap();
}
