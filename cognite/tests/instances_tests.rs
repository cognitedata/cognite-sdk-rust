#[cfg(test)]
use cognite::models::*;
use cognite::*;

use serde_json::json;
use wiremock::matchers::{body_json_string, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
pub use common::*;

mod fixtures;
pub use fixtures::*;

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

    assert_eq!(
        result
            .iter()
            .filter(|&x| match x {
                SlimNodeOrEdge::Node(_) => true,
                _ => false,
            })
            .count(),
        node_external_ids.len()
    );

    assert_eq!(
        result
            .iter()
            .filter(|&x| match x {
                SlimNodeOrEdge::Edge(_) => true,
                _ => false,
            })
            .count(),
        edge_external_ids.len()
    );

    // and delete the instances
    let instances_delete = get_mock_instances_delete(space, &node_external_ids, &edge_external_ids);

    let results = client
        .models
        .instances
        .delete(&instances_delete)
        .await
        .unwrap()
        .items;

    assert_eq!(
        results
            .iter()
            .filter(|&x| matches!(x, NodeOrEdgeSpecification::Node(_)))
            .count(),
        node_external_ids.len()
    );

    assert_eq!(
        results
            .iter()
            .filter(|&x| matches!(x, NodeOrEdgeSpecification::Edge(_)))
            .count(),
        edge_external_ids.len()
    );
}

#[test]
fn test_filter_serialization() {
    let filter = FdmFilter::equals(&["prop"], 15)
        .and(FdmFilter::not(FdmFilter::equals(&["other_prop"], 15)))
        .and(FdmFilter::exists(&["thing", "third_prop"]))
        .or(FdmFilter::contains_any(&["test"], &["value1", "value2"]));

    let json = json!(filter);
    assert_eq!(
        json!({
            "or": [{
                "and": [
                    {
                        "equals": {
                            "property": ["prop"],
                            "value": 15
                        }
                    },
                    {
                        "not": {
                            "equals": {
                                "property": ["other_prop"],
                                "value": 15
                            }
                        },
                    },
                    {
                        "exists": {
                            "property": ["thing", "third_prop"]
                        }
                    }
                ]
            }, {
                "containsAny": {
                    "property": ["test"],
                    "values": ["value1", "value2"]
                }
            }]
        }),
        json
    );
}
