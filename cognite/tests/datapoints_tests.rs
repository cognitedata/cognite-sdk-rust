#![cfg(feature = "integration_tests")]

#[cfg(test)]
use cognite::time_series::*;
use cognite::*;

mod common;
pub use common::*;

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
            id: Identity::Id { id: ts.id },
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
        .retrieve_datapoints(DatapointsFilter {
            items: vec![DatapointsQuery {
                id: Identity::Id { id: ts.id },
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
            id: Identity::Id { id: ts.id },
        }])
        .await
        .unwrap();

    // Retrieve the same range and expect us to get 45 less
    let dps = client
        .time_series
        .retrieve_datapoints(DatapointsFilter {
            items: vec![DatapointsQuery {
                id: Identity::Id { id: ts.id },
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
            id: Identity::Id { id: ts.id },
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
        .retrieve_datapoints(DatapointsFilter {
            items: vec![DatapointsQuery {
                id: Identity::Id { id: ts.id },
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
            id: Identity::Id { id: ts.id },
        }])
        .await
        .unwrap();

    // Retrieve the same range and expect us to get 45 less
    let dps = client
        .time_series
        .retrieve_datapoints(DatapointsFilter {
            items: vec![DatapointsQuery {
                id: Identity::Id { id: ts.id },
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
            id: Identity::Id { id: ts.id },
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
                id: Identity::Id { id: ts.id },
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
        latest
            .datapoints
            .numeric()
            .unwrap()
            .first()
            .unwrap()
            .value
            .unwrap()
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
            id: Identity::Id { id: ts.id },
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
        .retrieve_datapoints(DatapointsFilter {
            items: vec![DatapointsQuery {
                id: Identity::from(ts.id),
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

    delete_test_ts(&client, ts.id).await;
}
