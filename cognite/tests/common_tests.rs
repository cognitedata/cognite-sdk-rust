use cognite::{
    assets::{AssetQuery, FilterAssetsRequest},
    FilterWithRequest, List,
};
use futures::TryStreamExt;
use serde_json::{json, Value};
use wiremock::{
    matchers::{body_json_string, method, path, query_param, query_param_is_missing},
    Mock, MockServer, ResponseTemplate,
};

mod common;
pub use common::*;

#[tokio::test]
async fn stream_responses() {
    let mock_server = MockServer::start().await;
    let project = "my_project";

    fn gen_asset(id: i64) -> Value {
        json!({
            "id": id,
            "name": "test",
            "createdTime": 1234,
            "lastUpdatedTime": 1234
        })
    }

    Mock::given(method("POST"))
        .and(body_json_string(json!({}).to_string()))
        .and(path(get_path("", project, "assets/list")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [gen_asset(1), gen_asset(2), gen_asset(3)],
            "nextCursor": "cursor1"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(body_json_string(
            json!({
                "cursor": "cursor1"
            })
            .to_string(),
        ))
        .and(path(get_path("", project, "assets/list")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [gen_asset(4), gen_asset(5), gen_asset(6)],
            "nextCursor": null
        })))
        .mount(&mock_server)
        .await;

    let client = get_client_for_mocking(&mock_server.uri(), project);

    let stream = client
        .assets
        .filter_all_stream(FilterAssetsRequest::default());

    let it: Vec<_> = stream.try_collect().await.unwrap();
    assert_eq!(6, it.len());
    for el in it {
        assert_eq!(el.name, "test");
    }
}

#[tokio::test]
async fn stream_responses_list() {
    let mock_server = MockServer::start().await;
    let project = "my_project";

    fn gen_asset(id: i64) -> Value {
        json!({
            "id": id,
            "name": "test",
            "createdTime": 1234,
            "lastUpdatedTime": 1234
        })
    }

    Mock::given(method("GET"))
        .and(query_param_is_missing("cursor"))
        .and(path(get_path("", project, "assets")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [gen_asset(1), gen_asset(2), gen_asset(3)],
            "nextCursor": "cursor1"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(query_param("cursor", "cursor1"))
        .and(path(get_path("", project, "assets")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [gen_asset(4), gen_asset(5), gen_asset(6)],
            "nextCursor": null
        })))
        .mount(&mock_server)
        .await;

    let client = get_client_for_mocking(&mock_server.uri(), project);

    let stream = client.assets.list_all_stream(AssetQuery::default());

    let it: Vec<_> = stream.try_collect().await.unwrap();
    assert_eq!(6, it.len());
    for el in it {
        assert_eq!(el.name, "test");
    }
}

#[test]
fn test_resource_usage_send() {
    fn assert_send<T: Send>(t: T) -> T {
        t
    }

    let client = get_client();

    // Assert that futures from `Resource` are still send.
    let _ = assert_send(client.assets.list_all(AssetQuery::default()));
    let _ = assert_send(client.time_series.list(None));
}
