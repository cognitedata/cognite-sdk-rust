use bytes::Bytes;
use cognite::{
    assets::{AssetQuery, FilterAssetsRequest},
    FilterWithRequest, List,
};
use futures::{future, stream, TryStreamExt};
use serde_json::{json, Value};
use wiremock::{
    matchers::{body_json_string, body_string, method, path, query_param, query_param_is_missing},
    Mock, MockServer, Request, ResponseTemplate,
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

#[tokio::test]
async fn test_no_auth_header_for_untrusted_urls() {
    let mock_server = MockServer::start().await;
    let project = "my_project";

    let api_base_url = format!("{}/base", mock_server.uri());
    let untrusted_url = format!("{}/untrusted", mock_server.uri());

    let has_no_auth_headers = |req: &Request| {
        ["authorization", "auth-ticket"]
            .into_iter()
            .all(|header| !req.headers.contains_key(header))
    };

    Mock::given(method("GET"))
        .and(path("/untrusted"))
        .and(has_no_auth_headers)
        .respond_with(ResponseTemplate::new(200).set_body_string("abcdef"))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/untrusted"))
        .and(has_no_auth_headers)
        .and(body_string("qwerty"))
        .respond_with(ResponseTemplate::new(200).set_body_string("abcdef"))
        .expect(3)
        .mount(&mock_server)
        .await;

    let client = get_client_for_mocking(&api_base_url, project);

    assert_eq!(
        client
            .api_client
            .get_stream(&untrusted_url)
            .await
            .unwrap()
            .try_collect::<Vec<_>>()
            .await
            .unwrap(),
        vec![Bytes::from_static(b"abcdef")]
    );

    client
        .api_client
        .put_stream(
            &untrusted_url,
            "text/plain",
            stream::once(future::ready(anyhow::Ok(Bytes::from_static(b"qwerty")))),
            false,
            Some(6),
        )
        .await
        .unwrap();

    client
        .api_client
        .put_stream(
            &untrusted_url,
            "text/plain",
            stream::once(future::ready(anyhow::Ok(Bytes::from_static(b"qwerty")))),
            true,
            None,
        )
        .await
        .unwrap();

    client
        .api_client
        .put_blob(&untrusted_url, "text/plain", Bytes::from_static(b"qwerty"))
        .await
        .unwrap();

    mock_server.verify().await;
}
