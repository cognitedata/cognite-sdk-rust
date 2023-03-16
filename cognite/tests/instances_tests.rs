use std::collections::HashMap;

#[cfg(test)]
use cognite::models::*;
use cognite::*;

use wiremock::matchers::{body_json_string, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
pub(crate) use common::*;

mod fixtures;
pub(crate) use fixtures::*;

#[tokio::test]
async fn create_and_delete_instances() {
    let project = "my_project";
    let space = "my_space";
    let node_external_ids = vec!["node1", "node2"];
    let edge_external_ids = vec!["edge1"];

    let mock_server = MockServer::start().await;

    // mock create instance
    Mock::given(method("POST"))
        .and(body_json_string(get_instances_create_request_string(
            space,
            &node_external_ids,
            &edge_external_ids,
        )))
        .and(path(get_path("", project, "models/instances")))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            get_instances_create_response_string(space, &node_external_ids, &edge_external_ids),
        ))
        .mount(&mock_server)
        .await;

    // mock delete instance
    Mock::given(method("POST"))
        .and(path(get_path("", project, "models/instances/delete")))
        .and(body_json_string(get_instances_delete_request_string(
            space,
            &node_external_ids,
            &edge_external_ids,
        )))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            get_instances_delete_response_string(space, &node_external_ids, &edge_external_ids),
        ))
        .mount(&mock_server)
        .await;

    // create instances
    let client = get_client_for_mocking(&mock_server.uri(), project);

    let mock_instances = get_mock_instances(space, &node_external_ids, &edge_external_ids);
    let upsert_collection = NodeAndEdgeCreateCollection {
        items: mock_instances.clone(),
        ..Default::default()
    };

    let result = client
        .models
        .instances
        .upsert(&upsert_collection)
        .await
        .unwrap();

    assert_eq!(result.len(), mock_instances.len());

    // and delete the instances
    let instances_delete = get_mock_instances_delete(space, &node_external_ids, &edge_external_ids);

    let results = client
        .models
        .instances
        .delete(&instances_delete)
        .await
        .unwrap()
        .items;

    assert_eq!(results.len(), instances_delete.len());
}
