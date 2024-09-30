#[cfg(test)]
use cognite::models::instances::*;
use cognite::*;

use models::{
    data_models::{CogniteExtractorFile, RetrieveExtendedCollection, UpsertExtendedCollection},
    ItemId,
};
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json_string, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
pub use common::*;

mod fixtures;
pub use fixtures::*;

#[tokio::test]
async fn create_and_delete_file_instance() {
    let client = CogniteClient::new_oidc("testing_instances", None).unwrap();
    let external_id = Uuid::new_v4().to_string();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();
    let name = "random".to_string();
    let col = CogniteExtractorFile::new(space.to_string(), external_id.to_string(), name);
    let res = client
        .models
        .files
        .upsert(vec![col], None, None, None, None, None)
        .await
        .unwrap();
    let res = res.first().unwrap();
    assert!(matches!(res, SlimNodeOrEdge::Node(_)));

    let res_node = match res {
        cognite::models::instances::SlimNodeOrEdge::Node(slim_node_definition) => {
            slim_node_definition
        }
        cognite::models::instances::SlimNodeOrEdge::Edge(_) => {
            panic!("Invalid type received.")
        }
    };
    assert_eq!(external_id.to_string(), res_node.external_id);

    let node_specs = NodeOrEdgeSpecification::Node(ItemId {
        space: space.to_string(),
        external_id: external_id.to_string(),
    });
    let res_retrieve: Vec<CogniteExtractorFile> = client
        .models
        .files
        .retrieve(vec![node_specs.clone()])
        .await
        .unwrap();
    let file = &res_retrieve[0];
    assert_eq!(external_id.to_string(), file.external_id);

    let res_delete = client.models.instances.delete(&[node_specs]).await.unwrap();
    let res_delete = res_delete.items.first().unwrap();
    assert!(matches!(res_delete, NodeOrEdgeSpecification::Node(_)));
}

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
            .filter(|&x| matches!(x, SlimNodeOrEdge::Node(_)))
            .count(),
        node_external_ids.len()
    );

    assert_eq!(
        result
            .iter()
            .filter(|&x| matches!(x, SlimNodeOrEdge::Edge(_)))
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
    use cognite::filter::*;
    let filter = equals(["prop"], 15)
        .and(not(equals(["other_prop"], 15)))
        .and(exists(["thing", "third_prop"]))
        .and(range(["test"], 1..5))
        .or(contains_any(["test"], &["value1", "value2"]));

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
                    },
                    {
                        "range": {
                            "property": ["test"],
                            "gte": 1,
                            "lt": 5,
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
