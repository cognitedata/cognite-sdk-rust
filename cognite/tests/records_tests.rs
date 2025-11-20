#![cfg(feature = "integration_tests")]

mod common;
use std::collections::HashMap;
use std::sync::LazyLock;

use cognite::models::records::{
    LastUpdatedTimeFilter, PropertiesPerContainer, RecordCursor, RecordData, RecordWrite,
    RecordsPropertySort, RecordsRetrieveRequest, RecordsSyncRequest, StreamSettingsWrite,
    StreamTemplate, StreamWrite,
};
use cognite::models::{
    PrimitiveProperty, SortDirection, TaggedContainerReference, TextProperty, UsedFor,
};
use cognite::{filter, Create, List, RawValue, Retrieve};
use common::*;

use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_and_delete_stream_instance() {
    // It may be possible to make this an integration test in the future.
    // For now, streams/records team recommends not creating streams automatically in tests.

    let mock_server = MockServer::start().await;
    let project = "test";
    let external_id = "test-stream";

    // Register mock for creating a stream
    Mock::given(method("POST"))
        .and(path(get_path("", project, "streams")))
        .and(body_json(json!({
            "items": [
                {
                    "externalId": external_id,
                    "settings": {
                        "template": {
                            "name": "BasicLiveData"
                        }
                    }
                }
            ]
        })))
        .and(header("cdf-version", "beta"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{
                "externalId": external_id,
                "createdTime": 123456789,
                "createdFromTemplate": "BasicLiveData",
                "type": "Mutable",
            }]
        })))
        .mount(&mock_server)
        .await;
    // Register mock for retrieving a stream
    Mock::given(method("GET"))
        .and(path(get_path(
            "",
            project,
            &format!("streams/{}", external_id),
        )))
        .and(header("cdf-version", "beta"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "externalId": external_id,
            "createdTime": 123456789,
            "createdFromTemplate": "BasicLiveData",
                "type": "Mutable",
        })))
        .mount(&mock_server)
        .await;
    // Register mock for listing streams
    Mock::given(method("GET"))
        .and(path(get_path("", project, "streams")))
        .and(header("cdf-version", "beta"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{
                "externalId": external_id,
                "createdTime": 123456789,
                "createdFromTemplate": "BasicLiveData",
                "type": "Mutable",
            }]
        })))
        .mount(&mock_server)
        .await;
    // Register mock for deleting a stream
    Mock::given(method("POST"))
        .and(path(get_path("", project, "streams/delete")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&mock_server)
        .await;

    let client = get_client_for_mocking(&mock_server.uri(), project);

    let stream = client
        .models
        .streams
        .create(&[StreamWrite {
            external_id: external_id.to_owned(),
            settings: StreamSettingsWrite {
                template: StreamTemplate {
                    name: "BasicLiveData".to_owned(),
                },
            },
        }])
        .await
        .unwrap();

    assert_eq!(stream.len(), 1);
    let stream = &stream[0];
    assert_eq!(stream.external_id, external_id);

    let stream_retrieve = client
        .models
        .streams
        .retrieve(&external_id, false)
        .await
        .unwrap();
    assert_eq!(stream_retrieve.external_id, external_id);

    let stream_list = client.models.streams.list(None).await.unwrap();
    assert_eq!(stream_list.items.len(), 1);
    assert_eq!(stream_list.items[0].external_id, external_id);

    client.models.streams.delete(&[&external_id]).await.unwrap();
}

static STREAM_ENSURE_LOCK: LazyLock<Mutex<bool>> = LazyLock::new(|| tokio::sync::Mutex::new(false));

async fn ensure_stream(
    client: &cognite::CogniteClient,
    external_id: &str,
    mutable: bool,
) -> cognite::Result<()> {
    let ensured = STREAM_ENSURE_LOCK.lock().await;
    if *ensured {
        return Ok(());
    }
    match client.models.streams.retrieve(external_id, false).await {
        Ok(_) => return Ok(()),
        Err(cognite::Error::NotFound(_)) => {
            client
                .models
                .streams
                .create(&[StreamWrite {
                    external_id: external_id.to_owned(),
                    settings: StreamSettingsWrite {
                        template: StreamTemplate {
                            name: if mutable {
                                "BasicLiveData".to_owned()
                            } else {
                                "ImmutableTestStream".to_owned()
                            },
                        },
                    },
                }])
                .await?;
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

async fn ensure_test_container(
    client: &cognite::CogniteClient,
    external_id: &str,
) -> cognite::Result<()> {
    match client
        .models
        .containers
        .retrieve(&[cognite::models::ItemId {
            space: std::env::var("CORE_DM_TEST_SPACE").unwrap(),
            external_id: external_id.to_owned(),
        }])
        .await
    {
        Ok(r) if !r.is_empty() => Ok(()),
        _ => {
            client
                .models
                .containers
                .create(&[cognite::models::containers::ContainerCreate {
                    space: std::env::var("CORE_DM_TEST_SPACE").unwrap(),
                    external_id: external_id.to_owned(),
                    name: Some("Test Container".to_owned()),
                    description: None,
                    properties: [
                        (
                            "name".to_string(),
                            cognite::models::containers::ContainerPropertyDefinition {
                                name: Some("Name".to_owned()),
                                nullable: Some(true),
                                r#type: cognite::models::containers::ContainerPropertyType::Text(
                                    TextProperty::default(),
                                ),
                                auto_increment: None,
                                default_value: None,
                                description: None,
                            },
                        ),
                        (
                            "message".to_string(),
                            cognite::models::containers::ContainerPropertyDefinition {
                                name: Some("Message".to_owned()),
                                nullable: Some(true),
                                r#type: cognite::models::containers::ContainerPropertyType::Text(
                                    TextProperty::default(),
                                ),
                                auto_increment: None,
                                default_value: None,
                                description: None,
                            },
                        ),
                        (
                            "key".to_string(),
                            cognite::models::containers::ContainerPropertyDefinition {
                                name: Some("Key".to_owned()),
                                nullable: Some(false),
                                r#type: cognite::models::containers::ContainerPropertyType::Int64(
                                    PrimitiveProperty::default(),
                                ),
                                auto_increment: None,
                                default_value: None,
                                description: None,
                            },
                        ),
                    ]
                    .into(),
                    used_for: Some(UsedFor::Record),
                    constraints: HashMap::new(),
                    indexes: HashMap::new(),
                }])
                .await?;
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_retrieve_stream() {
    let client = get_client();
    let stream_external_id = "rust-sdk-test-stream-immutable";
    ensure_stream(&client, stream_external_id, false)
        .await
        .unwrap();
    let stream = client
        .models
        .streams
        .retrieve(stream_external_id, false)
        .await
        .unwrap();
    assert_eq!(stream.external_id, stream_external_id);
}

fn record_ext_id() -> String {
    format!("rust-sdk-test-record-{}", Uuid::new_v4()).replace("-", "_")
}

#[tokio::test]
async fn test_ingest_records() {
    let client = get_client();
    let stream_external_id = "rust-sdk-test-stream-immutable";
    ensure_stream(&client, stream_external_id, false)
        .await
        .unwrap();
    let container_id = "rust_sdk_records_test_container";
    ensure_test_container(&client, container_id).await.unwrap();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();

    let records = vec![RecordWrite {
        space: space.clone(),
        external_id: record_ext_id(),
        sources: vec![RecordData {
            source: TaggedContainerReference::new(space, container_id),
            properties: json!({
                "name": "test",
                "message": "test test",
                "key": 123,
            }),
        }],
    }];

    client
        .models
        .records
        .ingest(stream_external_id, &records)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_upsert_delete_records() {
    let client = get_client();
    let stream_external_id = "rust-sdk-test-stream-mutable";
    ensure_stream(&client, stream_external_id, true)
        .await
        .unwrap();
    let container_id = "rust_sdk_records_test_container";
    ensure_test_container(&client, container_id).await.unwrap();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();

    let record_id = record_ext_id();
    let records = vec![RecordWrite {
        space: space.clone(),
        external_id: record_id.clone(),
        sources: vec![RecordData {
            source: TaggedContainerReference::new(&space, container_id),
            properties: json!({
                "name": "test",
                "message": "test test",
                "key": 123,
            }),
        }],
    }];

    client
        .models
        .records
        .upsert(stream_external_id, &records)
        .await
        .unwrap();

    // Delete the record
    client
        .models
        .records
        .delete(
            stream_external_id,
            &[cognite::models::ItemId {
                space: space.clone(),
                external_id: record_id,
            }],
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_records() {
    // This just checks that the requests are accepted. Records is eventually consistent,
    // so verifying the response is annoying, and there's no consistent way to
    // produce records at a specific time, so we can't generate them in a way
    // that works for other projects either.
    // This verifies that the request is correctly formed and accepted.
    let client = get_client();
    let stream_external_id = "rust-sdk-test-stream-immutable";
    ensure_stream(&client, stream_external_id, false)
        .await
        .unwrap();
    let container_id = "rust_sdk_records_test_container";
    ensure_test_container(&client, container_id).await.unwrap();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();

    client
        .models
        .records
        .retrieve::<HashMap<String, RawValue>>(
            stream_external_id,
            &RecordsRetrieveRequest {
                last_updated_time: LastUpdatedTimeFilter {
                    gte: Some(0.into()),
                    lte: Some(10000.into()),
                    ..Default::default()
                },
                filter: Some(filter::equals([&space, container_id, "name"], "test")),
                sources: Some(vec![PropertiesPerContainer {
                    source: TaggedContainerReference::new(&space, container_id),
                    properties: vec!["name".to_string(), "message".to_string()],
                }]),
                limit: Some(5),
                sort: Some(vec![RecordsPropertySort::new(
                    [&space, container_id, "name"],
                    SortDirection::Ascending,
                )]),
            },
        )
        .await
        .unwrap();

    // Test sync too, this should produce a cursor.
    let res = client
        .models
        .records
        .sync::<HashMap<String, RawValue>>(
            stream_external_id,
            &RecordsSyncRequest {
                filter: Some(filter::equals([&space, container_id, "name"], "test")),
                sources: Some(vec![PropertiesPerContainer {
                    source: TaggedContainerReference::new(&space, container_id),
                    properties: vec!["name".to_string(), "message".to_string()],
                }]),
                limit: Some(5),
                cursor: RecordCursor::InitializeCursor("1h-ago".to_owned()),
            },
        )
        .await
        .unwrap();

    assert!(!res.extra_fields.next_cursor.is_empty());
}

#[test]
fn test_records_aggregate_request_ser() {
    // Tests for serializing and deserializing aggregate requests.
    use cognite::models::records::aggregates::*;
    use cognite::models::records::LastUpdatedTimeFilter;

    let v = serde_json::to_value(RecordsAggregateRequest {
        last_updated_time: LastUpdatedTimeFilter {
            gte: Some("1d-ago".into()),
            lt: Some(5000000i64.into()),
            ..Default::default()
        },
        filter: None,
        aggregates: [
            ("my_average", RecordsAggregate::average(["my", "property"])),
            (
                "my_unique_values",
                RecordsAggregate::unique_values(["my", "property"], Some(15), None),
            ),
            (
                "my_number_histogram",
                RecordsAggregate::number_histogram(
                    ["my", "property"],
                    15.0,
                    Some(Bounds {
                        min: Some(10.0),
                        max: Some(20.0),
                    }),
                    None,
                ),
            ),
            (
                "my_time_histogram",
                RecordsAggregate::time_histogram(
                    ["my", "property"],
                    TimeHistogramInterval::CalendarInterval(CalendarInterval::Month),
                    None,
                    None,
                ),
            ),
            (
                "my_time_histogram_2",
                RecordsAggregate::time_histogram(
                    ["my", "property"],
                    TimeHistogramInterval::FixedInterval("400h".to_owned()),
                    None,
                    None,
                ),
            ),
            (
                "my_moving_function",
                RecordsAggregate::moving_function("test>path", 5, MovingFunction::Sum),
            ),
            (
                "my_filters",
                RecordsAggregate::filters(vec![filter::equals(["my", "property"], 5)], None),
            ),
        ]
        .into_iter()
        .map(|(a, b)| (a.to_string(), b))
        .collect(),
    })
    .unwrap();
    assert_eq!(
        v,
        json!({
            "lastUpdatedTime": {
                "gte": "1d-ago",
                "lt": 5000000
            },
            "aggregates": {
                "my_average": {
                    "avg": {
                        "property": ["my", "property"]
                    }
                },
                "my_unique_values": {
                    "uniqueValues": {
                        "property": ["my", "property"],
                        "size": 15
                    }
                },
                "my_number_histogram": {
                    "numberHistogram": {
                        "property": ["my", "property"],
                        "interval": 15.0,
                        "hardBounds": {
                            "min": 10.0,
                            "max": 20.0
                        }
                    }
                },
                "my_time_histogram": {
                    "timeHistogram": {
                        "calendarInterval": "1M",
                        "property": ["my", "property"],
                    }
                },
                "my_time_histogram_2": {
                    "timeHistogram": {
                        "fixedInterval": "400h",
                        "property": ["my", "property"],
                    }
                },
                "my_moving_function":{
                    "movingFunction": {
                        "window": 5,
                        "function": "MovingFunctions.sum",
                        "bucketsPath": "test>path"
                    }
                },
                "my_filters": {
                    "filters": {
                        "filters": [
                            {
                                "equals": {
                                    "property": ["my", "property"],
                                    "value": 5
                                }
                            }
                        ]
                    }
                }
            }
        })
    );
}

macro_rules! assert_is {
    ($value:expr, $v:ident, $p:pat) => {
        match $value {
            $p => $v,
            _ => panic!("Expected {}, got {:?}", stringify!($p), $value),
        }
    };
}

#[test]
fn test_records_aggregate_response_de() {
    use cognite::models::records::aggregates::*;
    let json = json!({
        "aggregates": {
            "my_average": {
                "avg": 15.0,
            },
            "my_count": {
                "count": 10,
            },
            "my_sum": {
                "sum": 100.0,
            },
            "my_min": {
                "min": 5.0,
            },
            "my_max": {
                "max": 20.0,
            },
            "my_unique_values": {
                "buckets": [{
                    "value": 5,
                    "count": 3,
                }, {
                    "value": "hello",
                    "count": 2,
                }, {
                    "value": true,
                    "count": 1,
                }]
            },
            "my_number_histogram": {
                "buckets": [{
                    "count": 10,
                    "intervalStart": 0.0,
                }, {
                    "count": 5,
                    "intervalStart": 10.0,
                }]
            },
            "my_time_histogram": {
                "buckets": [{
                    "count": 10,
                    "intervalStart": "2023-01-01T00:00:00Z",
                }, {
                    "count": 5,
                    "intervalStart": "2023-02-01T00:00:00Z",
                }]
            },
        }
    });

    let deser = serde_json::from_value::<RecordsAggregateResult>(json).unwrap();
    assert_eq!(deser.aggregates.len(), 8);
    let v = assert_is!(
        deser.aggregates.get("my_average").unwrap(),
        v,
        AggregateResult::Avg(v)
    );
    assert_eq!(*v, 15.0);
    let v = assert_is!(
        deser.aggregates.get("my_count").unwrap(),
        v,
        AggregateResult::Count(v)
    );
    assert_eq!(*v, 10);
    let v = assert_is!(
        deser.aggregates.get("my_sum").unwrap(),
        v,
        AggregateResult::Sum(v)
    );
    assert_eq!(*v, 100.0);
    let v = assert_is!(
        deser.aggregates.get("my_min").unwrap(),
        v,
        AggregateResult::Min(v)
    );
    assert_eq!(*v, 5.0);
    let v = assert_is!(
        deser.aggregates.get("my_max").unwrap(),
        v,
        AggregateResult::Max(v)
    );
    assert_eq!(*v, 20.0);
    let v = assert_is!(
        deser.aggregates.get("my_unique_values").unwrap(),
        v,
        AggregateResult::Buckets(AggregateBuckets::UniqueValues(v))
    );
    assert_eq!(v.len(), 3);
    assert_eq!(v[0].value, 5.into());
    assert_eq!(v[0].count, 3);
    assert_eq!(v[1].value, "hello".into());
    assert_eq!(v[1].count, 2);
    assert_eq!(v[2].value, true.into());
    assert_eq!(v[2].count, 1);
    let v = assert_is!(
        deser.aggregates.get("my_number_histogram").unwrap(),
        v,
        AggregateResult::Buckets(AggregateBuckets::NumberHistogram(v))
    );
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].count, 10);
    assert_eq!(v[0].interval_start, 0.0);
    assert_eq!(v[1].count, 5);
    assert_eq!(v[1].interval_start, 10.0);
    let v = assert_is!(
        deser.aggregates.get("my_time_histogram").unwrap(),
        v,
        AggregateResult::Buckets(AggregateBuckets::TimeHistogram(v))
    );
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].count, 10);
    assert_eq!(v[0].interval_start, "2023-01-01T00:00:00Z");
    assert_eq!(v[1].count, 5);
    assert_eq!(v[1].interval_start, "2023-02-01T00:00:00Z");
}
