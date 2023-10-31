#[cfg(test)]
use cognite::time_series::*;
use cognite::*;

mod common;
pub use common::*;

async fn create_test_ts(client: &CogniteClient, is_string: bool, idx: i32) -> TimeSerie {
    let ts = AddTimeSerie {
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
                        value: i as f64,
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
                        value: format!("{i}-dp"),
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
                        value: i as f64,
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
                before: format!("{}", start + 200_000),
                id: Identity::Id { id: ts.id },
            }],
            false,
        )
        .await
        .unwrap();

    assert_eq!(1, latest.len());
    let latest = latest.into_iter().next().unwrap();
    assert_eq!(
        99.0,
        latest.datapoints.numeric().unwrap().first().unwrap().value
    );

    delete_test_ts(&client, ts.id).await;
}
