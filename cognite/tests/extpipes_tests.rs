use cognite::extpipes::{AddExtPipeConfig, ExtPipeConfigQuery, ExtPipeConfigRevisionsQuery};
use serde_json::{json, Value};
use wiremock::{
    matchers::{body_json_string, method, path, query_param, query_param_is_missing},
    Mock, MockServer, ResponseTemplate,
};

mod common;
pub use common::*;

const PROJECT: &str = "my_project";
const PIPELINE: &str = "my-pipeline";

fn config_json(revision: i32) -> Value {
    json!({
        "externalId": PIPELINE,
        "config": format!("version: {revision}"),
        "revision": revision,
        "createdTime": 1730204346000_i64,
        "description": format!("Revision {revision}")
    })
}

#[tokio::test]
async fn retrieve_latest_ext_pipe_config() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(get_path("", PROJECT, "extpipes/config")))
        .and(query_param("externalId", PIPELINE))
        .and(query_param_is_missing("revision"))
        .and(query_param_is_missing("activeAtTime"))
        .respond_with(ResponseTemplate::new(200).set_body_json(config_json(3)))
        .expect(1)
        .mount(&mock_server)
        .await;
    let client = get_client_for_mocking(&mock_server.uri(), PROJECT);

    let config = client
        .ext_pipe_configs
        .retrieve(ExtPipeConfigQuery {
            external_id: PIPELINE.to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(config.external_id, PIPELINE);
    assert_eq!(config.revision, 3);
    assert_eq!(config.created_time, 1730204346000);
    assert_eq!(config.config.as_deref(), Some("version: 3"));
    assert_eq!(config.description.as_deref(), Some("Revision 3"));
}

#[tokio::test]
async fn retrieve_ext_pipe_config_by_revision_and_time() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(get_path("", PROJECT, "extpipes/config")))
        .and(query_param("externalId", PIPELINE))
        .and(query_param("revision", "2"))
        .and(query_param("activeAtTime", "1730204346000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(config_json(2)))
        .expect(1)
        .mount(&mock_server)
        .await;
    let client = get_client_for_mocking(&mock_server.uri(), PROJECT);

    let config = client
        .ext_pipe_configs
        .retrieve(ExtPipeConfigQuery {
            external_id: PIPELINE.to_string(),
            revision: Some(2),
            active_at_time: Some(1730204346000),
        })
        .await
        .unwrap();

    assert_eq!(config.revision, 2);
}

#[tokio::test]
async fn list_ext_pipe_config_revisions() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(get_path("", PROJECT, "extpipes/config/revisions")))
        .and(query_param("externalId", PIPELINE))
        .and(query_param("limit", "2"))
        .and(query_param("cursor", "cursor1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [
                {
                    "externalId": PIPELINE,
                    "revision": 2,
                    "createdTime": 1730204346000_i64,
                    "description": "Revision 2"
                },
                {
                    "externalId": PIPELINE,
                    "revision": 1,
                    "createdTime": 1730204345000_i64
                }
            ],
            "nextCursor": "cursor2"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;
    let client = get_client_for_mocking(&mock_server.uri(), PROJECT);

    let revisions = client
        .ext_pipe_configs
        .list_revisions(ExtPipeConfigRevisionsQuery {
            external_id: PIPELINE.to_string(),
            limit: Some(2),
            cursor: Some("cursor1".to_string()),
        })
        .await
        .unwrap();

    let numbers: Vec<i32> = revisions.items.iter().map(|r| r.revision).collect();
    assert_eq!(numbers, vec![2, 1]);
    assert_eq!(revisions.items[1].description, None);
    assert_eq!(
        revisions.extra_fields.next_cursor.as_deref(),
        Some("cursor2")
    );
}

#[tokio::test]
async fn create_ext_pipe_config() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(get_path("", PROJECT, "extpipes/config")))
        .and(body_json_string(
            json!({
                "externalId": PIPELINE,
                "config": "version: 4",
                "description": "Revision 4"
            })
            .to_string(),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(config_json(4)))
        .expect(1)
        .mount(&mock_server)
        .await;
    let client = get_client_for_mocking(&mock_server.uri(), PROJECT);

    let config = client
        .ext_pipe_configs
        .create(&AddExtPipeConfig {
            external_id: PIPELINE.to_string(),
            config: Some("version: 4".to_string()),
            description: Some("Revision 4".to_string()),
        })
        .await
        .unwrap();

    assert_eq!(config.revision, 4);
}

#[tokio::test]
async fn revert_ext_pipe_config() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(get_path("", PROJECT, "extpipes/config/revert")))
        .and(body_json_string(
            json!({
                "externalId": PIPELINE,
                "revision": 2
            })
            .to_string(),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(config_json(5)))
        .expect(1)
        .mount(&mock_server)
        .await;
    let client = get_client_for_mocking(&mock_server.uri(), PROJECT);

    let config = client.ext_pipe_configs.revert(PIPELINE, 2).await.unwrap();

    assert_eq!(config.revision, 5);
    assert_eq!(config.config.as_deref(), Some("version: 5"));
}
