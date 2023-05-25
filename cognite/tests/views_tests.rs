#[cfg(test)]
use cognite::models::*;
use cognite::*;

use wiremock::matchers::{body_json_string, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
pub use common::*;

mod fixtures;
pub use fixtures::*;

#[tokio::test]
async fn list_views() {
    let project = "my_project";

    let mock_server = MockServer::start().await;

    // mock create instance
    Mock::given(method("GET"))
        .and(query_param("space", "MySpace"))
        .and(path(get_path("", project, "models/views")))
        .respond_with(ResponseTemplate::new(200).set_body_string(get_views_list_views_response()))
        .mount(&mock_server)
        .await;

    // create instances
    let client = get_client_for_mocking(&mock_server.uri(), project);

    let result = &client
        .models
        .views
        .list(Some(ViewQuery {
            space: Some("MySpace".to_string()),
            ..Default::default()
        }))
        .await
        .unwrap()
        .items[0];

    assert_eq!(result.external_id, "MyView".to_string());
    assert_eq!(result.space, "MySpace".to_string());
    assert_eq!(result.last_updated_time, 1679040460082i64);
}

#[tokio::test]
async fn retrieve_views() {
    let project = "my_project";

    let mock_server = MockServer::start().await;

    // mock create instance
    Mock::given(method("POST"))
        .and(body_json_string(get_views_retrieve_views_request()))
        .and(path(get_path("", project, "models/views/byids")))
        .respond_with(ResponseTemplate::new(200).set_body_string(get_views_list_views_response()))
        .mount(&mock_server)
        .await;

    // create instances
    let client = get_client_for_mocking(&mock_server.uri(), project);

    let result = &client
        .models
        .views
        .retrieve(&[ItemIdOptionalVersion {
            space: "MySpace".to_string(),
            external_id: "MyView".to_string(),
            version: Some("1".to_string()),
        }])
        .await
        .unwrap()[0];

    assert_eq!(result.external_id, "MyView".to_string());
    assert_eq!(result.space, "MySpace".to_string());
    assert_eq!(result.version, "1".to_string());
    assert_eq!(result.last_updated_time, 1679040460082i64);
}

#[tokio::test]
async fn delete_views() {
    let project = "my_project";

    let mock_server = MockServer::start().await;

    // mock create instance
    Mock::given(method("POST"))
        .and(body_json_string(get_views_retrieve_views_request()))
        .and(path(get_path("", project, "models/views/delete")))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(get_views_retrieve_views_request()),
        )
        .mount(&mock_server)
        .await;

    // create instances
    let client = get_client_for_mocking(&mock_server.uri(), project);

    let result = &client
        .models
        .views
        .delete(&[ItemIdWithVersion {
            space: "MySpace".to_string(),
            external_id: "MyView".to_string(),
            version: "1".to_string(),
        }])
        .await
        .unwrap()
        .items[0];

    assert_eq!(result.external_id, "MyView".to_string());
    assert_eq!(result.space, "MySpace".to_string());
    assert_eq!(result.version, "1".to_string());
}
