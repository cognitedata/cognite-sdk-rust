#![cfg(feature = "integration_tests")]

mod common;
use std::collections::HashMap;
use std::sync::LazyLock;

use cognite::models::records::{
    LastUpdatedTimeFilter, PropertiesPerContainer, RecordCursor, RecordData, RecordWrite,
    RecordsPropertySort, RecordsRetrieveRequest, RecordsSyncRequest, StreamWrite,
};
use cognite::models::{SortDirection, TaggedContainerReference};
use cognite::{filter, Create, List, RawValue};
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
                }
            ]
        })))
        .and(header("cdf-version", "beta"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{
                "externalId": external_id,
                "createdTime": 123456789,
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
            }]
        })))
        .mount(&mock_server)
        .await;
    // Register mock for deleting a stream
    Mock::given(method("DELETE"))
        .and(path(get_path(
            "",
            project,
            &format!("streams/{}", external_id),
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&mock_server)
        .await;

    let client = get_client_for_mocking(&mock_server.uri(), project);

    let stream = client
        .models
        .streams
        .create(&[StreamWrite {
            external_id: external_id.to_owned(),
        }])
        .await
        .unwrap();

    assert_eq!(stream.len(), 1);
    let stream = &stream[0];
    assert_eq!(stream.external_id, external_id);

    let stream_retrieve = client.models.streams.retrieve(&external_id).await.unwrap();
    assert_eq!(stream_retrieve.external_id, external_id);

    let stream_list = client.models.streams.list(None).await.unwrap();
    assert_eq!(stream_list.items.len(), 1);
    assert_eq!(stream_list.items[0].external_id, external_id);

    client.models.streams.delete(&external_id).await.unwrap();
}

static STREAM_ENSURE_LOCK: LazyLock<Mutex<bool>> = LazyLock::new(|| tokio::sync::Mutex::new(false));

async fn ensure_stream(client: &cognite::CogniteClient, external_id: &str) -> cognite::Result<()> {
    let ensured = STREAM_ENSURE_LOCK.lock().await;
    if *ensured {
        return Ok(());
    }
    match client.models.streams.retrieve(external_id).await {
        Ok(_) => return Ok(()),
        Err(cognite::Error::NotFound(_)) => {
            client
                .models
                .streams
                .create(&[StreamWrite {
                    external_id: external_id.to_owned(),
                }])
                .await?;
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[tokio::test]
async fn test_retrieve_stream() {
    let client = get_client();
    let stream_external_id = "rust-sdk-test-stream";
    ensure_stream(&client, stream_external_id).await.unwrap();
    let stream = client
        .models
        .streams
        .retrieve(stream_external_id)
        .await
        .unwrap();
    assert_eq!(stream.external_id, stream_external_id);
}

#[tokio::test]
async fn test_ingest_records() {
    let client = get_client();
    let stream_external_id = "rust-sdk-test-stream";
    ensure_stream(&client, stream_external_id).await.unwrap();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();

    let records = vec![RecordWrite {
        space: space.clone(),
        external_id: Uuid::new_v4().to_string(),
        sources: vec![RecordData {
            source: TaggedContainerReference::new("cdf_cdm", "CogniteDescribable"),
            properties: json!({
                "name": "test",
                "description": "test test",
                "tags": ["tag1", "tag2"],
            }),
        }],
    }];

    client
        .models
        .records
        .ingest(stream_external_id, records)
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
    let stream_external_id = "rust-sdk-test-stream";
    ensure_stream(&client, stream_external_id).await.unwrap();

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
                filter: Some(filter::equals(
                    ["cdf_cdm", "CogniteDescribable", "name"],
                    "test",
                )),
                sources: Some(vec![PropertiesPerContainer {
                    source: TaggedContainerReference::new("cdf_cdm", "CogniteDescribable"),
                    properties: vec!["name".to_string(), "description".to_string()],
                }]),
                limit: Some(5),
                sort: Some(vec![RecordsPropertySort::new(
                    ["cdf_cdm", "CogniteDescribable", "name"],
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
                filter: Some(filter::equals(
                    ["cdf_cdm", "CogniteDescribable", "name"],
                    "test",
                )),
                sources: Some(vec![PropertiesPerContainer {
                    source: TaggedContainerReference::new("cdf_cdm", "CogniteDescribable"),
                    properties: vec!["name".to_string(), "description".to_string()],
                }]),
                limit: Some(5),
                cursor: RecordCursor::InitializeCursor("1h-ago".to_owned()),
            },
        )
        .await
        .unwrap();

    assert!(!res.extra_fields.next_cursor.is_empty());
}
