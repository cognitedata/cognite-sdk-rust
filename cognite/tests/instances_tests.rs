#[cfg(test)]
use cognite::models::instances::{AggregateResult, *};
use cognite::models::{views::ViewReference, SourceReference, TaggedViewReference};
use cognite::*;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use wiremock::matchers::{body_json_string, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
pub use common::*;

mod fixtures;
pub use fixtures::*;

#[tokio::test]
async fn aggregate_instances() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(get_path(
            "",
            "cdf-project",
            "models/instances/aggregate",
        )))
        .and(body_json_string(get_instances_aggregate_request()))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(get_instances_aggregate_response()),
        )
        .mount(&mock_server)
        .await;
    let client = get_client_for_mocking(&mock_server.uri(), "cdf-project");

    let aggregate_request = AggregateInstancesRequest {
        query: None,
        properties: None,
        limit: None,
        group_by: Some(vec!["industry".to_string()]),
        filter: None,
        aggregates: Some(vec![
            InstancesAggregate::Max {
                property: "property_1".to_string(),
            },
            InstancesAggregate::Histogram {
                property: "property_1".to_string(),
                interval: 1_f64,
            },
        ]),
        instance_type: InstanceType::Node,
        view: ViewReference {
            space: "space_1".to_owned(),
            external_id: "view_1".to_owned(),
            version: "1".to_owned(),
        }
        .into(),
    };
    let res = client
        .models
        .instances
        .aggregate(aggregate_request)
        .await
        .unwrap();
    assert!(res.items[0]
        .aggregates
        .iter()
        .any(|x| matches!(x, AggregateResult::Histogram { .. })));
    assert!(res.items[0]
        .aggregates
        .iter()
        .any(|x| matches!(x, AggregateResult::Max(_))));
    // .any(|x| matches!(x, AggregateResult::Max(_))));
    // assert_eq!(res.items.len(), 1);
}

#[derive(Deserialize, Serialize, Debug)]
struct EdgeProperties {
    string_field: String,
    numeric_field: i32,
}

#[tokio::test]
async fn create_edge_with_properties() {
    let project = "my_project";
    let space = "my_space";
    let edge_external_id = "edge1";
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(get_path("", project, "models/instances")))
        .and(body_json_string(get_edge_create_request(space)))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(get_edge_create_response(space, edge_external_id)),
        )
        .mount(&mock_server)
        .await;

    let client = get_client_for_mocking(&mock_server.uri(), project);

    let instances: Vec<NodeOrEdgeCreate<EdgeProperties>> =
        vec![NodeOrEdgeCreate::Edge(EdgeWrite {
            space: space.to_string(),
            r#type: InstanceId {
                space: space.to_string(),
                external_id: "typeNode".to_owned(),
            },
            start_node: InstanceId {
                space: space.to_string(),
                external_id: "startNode".to_owned(),
            },
            end_node: InstanceId {
                space: space.to_string(),
                external_id: "endNode".to_owned(),
            },
            external_id: edge_external_id.to_owned(),
            sources: Some(vec![EdgeOrNodeData {
                source: SourceReference::View(ViewReference {
                    space: space.to_string(),
                    external_id: "View".to_string(),
                    version: "1".to_string(),
                }),
                properties: EdgeProperties {
                    string_field: "string value".to_owned(),
                    numeric_field: 42,
                },
            }]),
            existing_version: None,
        })];

    let result = client
        .models
        .instances
        .upsert(&NodeAndEdgeCreateCollection {
            items: instances,
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(
        result
            .iter()
            .filter(|&x| matches!(x, SlimNodeOrEdge::Edge(_)))
            .count(),
        1
    );
}

#[tokio::test]
async fn query_edge_with_properties() {
    let mock_server = MockServer::start().await;
    let project = "my_project";
    let space = "my_space";

    // mock edge query
    Mock::given(method("POST"))
        .and(body_json_string(get_edge_query_request()))
        .and(path(get_path("", project, "models/instances/query")))
        .respond_with(ResponseTemplate::new(200).set_body_string(get_edge_query_response()))
        .mount(&mock_server)
        .await;
    let client = get_client_for_mocking(&mock_server.uri(), project);
    let query_request = QueryInstancesRequest {
        with: HashMap::from([(
            "edge_query".to_string(),
            QueryTableExpression::Edge(QueryEdgeTableExpression {
                edges: EdgesQuery {
                    ..Default::default()
                },
                ..Default::default()
            }),
        )]),
        cursors: None,
        select: HashMap::from([(
            "edge_query".to_string(),
            SelectExpression {
                sort: None,
                limit: None,
                sources: vec![SourceSelector {
                    source: TaggedViewReference::View(ViewReference {
                        space: space.to_string(),
                        external_id: "View".to_string(),
                        version: "1".to_string(),
                    }),
                    properties: vec!["*".to_string()],
                }],
            },
        )]),
        parameters: None,
    };
    let result: QueryInstancesResponse<EdgeProperties> =
        client.models.instances.query(query_request).await.unwrap();
    assert_eq!(result.items.len(), 1);
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
