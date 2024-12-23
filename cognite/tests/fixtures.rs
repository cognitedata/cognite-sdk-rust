#[cfg(test)]
use std::collections::HashMap;

use cognite::models::{
    instances::{
        EdgeOrNodeData, EdgeWrite, InstanceId, NodeOrEdgeCreate, NodeOrEdgeSpecification, NodeWrite,
    },
    views::ViewReference,
    ItemId, SourceReference,
};

fn get_mock_properties() -> HashMap<String, String> {
    let mut properties = std::collections::HashMap::new();
    properties.insert("key1".to_string(), "value1".to_string());
    properties.insert("key2".to_string(), "value2".to_string());
    properties
}

pub fn get_mock_instances(
    space: &str,
    node_external_id: &[&str],
    edge_external_id: &[&str],
) -> Vec<NodeOrEdgeCreate<HashMap<String, String>>> {
    let properties = get_mock_properties();

    let mut mock_instances: Vec<NodeOrEdgeCreate<HashMap<String, String>>> = Vec::new();

    // add nodes
    mock_instances.extend(
        node_external_id
            .iter()
            .map(|id| {
                NodeOrEdgeCreate::Node(NodeWrite {
                    space: space.to_owned(),
                    external_id: id.to_string(),
                    sources: Some(vec![EdgeOrNodeData {
                        source: SourceReference::View(ViewReference {
                            space: space.to_owned(),
                            external_id: "some_view".to_string(),
                            version: "1".to_string(),
                        }),
                        properties: properties.clone(),
                    }]),
                    ..Default::default()
                })
            })
            .collect::<Vec<_>>(),
    );
    mock_instances.extend(
        edge_external_id
            .iter()
            .map(|id| {
                NodeOrEdgeCreate::Edge(EdgeWrite {
                    r#type: InstanceId {
                        space: space.to_owned(),
                        external_id: id.to_string(),
                    },
                    space: space.to_owned(),
                    external_id: id.to_string(),
                    start_node: InstanceId {
                        space: space.to_owned(),
                        external_id: "start_node".to_string(),
                    },
                    end_node: InstanceId {
                        space: space.to_owned(),
                        external_id: "end_node".to_string(),
                    },
                    ..Default::default()
                })
            })
            .collect::<Vec<_>>(),
    );
    mock_instances
}

pub fn get_instances_create_request_string(
    space: &str,
    node_external_ids: &[&str],
    edge_external_ids: &[&str],
) -> String {
    let properties = serde_json::to_string(&get_mock_properties()).unwrap();
    let mut items = "".to_string();
    node_external_ids.iter().for_each(|id| {
        items.push_str(&format!(
            r#"{{"instanceType": "node", "space": "{space}", "externalId": "{id}", "sources": [{{"source": {{ "type": "view", "space": "{space}", "externalId": "some_view", "version": "1" }}, "properties": {properties}  }}] }},"#,
        ));
    });
    edge_external_ids.iter().for_each(|id| {
        items.push_str(&format!(
            r#"{{"instanceType": "edge", "space": "{space}", "externalId": "{id}", "type": {{ "space": "{space}", "externalId": "{id}" }}, "startNode": {{ "space": "{space}", "externalId": "start_node" }}, "endNode": {{ "space": "{space}", "externalId": "end_node" }} }},"#,
        ));
    });

    items.pop(); // remove last comma

    let req = format!(r#"{{"items":[{items}]}}"#);
    req
}

pub fn get_instances_create_response_string(
    space: &str,
    node_external_ids: &[&str],
    edge_external_ids: &[&str],
) -> String {
    let base_response = format!(
        r#""version": 1, "space": "{space}", "wasModified": true,  "createdTime": 0, "lastUpdatedTime": 0"#
    );
    let mut items = "".to_string();
    node_external_ids.iter().for_each(|id| {
        items.push_str(&format!(
            r#"{{"instanceType": "node", "externalId": "{id}", {base_response} }},"#,
        ));
    });
    edge_external_ids.iter().for_each(|id| {
        items.push_str(&format!(
            r#"{{"instanceType": "edge", "externalId": "{id}", {base_response} }},"#,
        ));
    });
    items.pop(); // remove last comma

    let res = format!(r#"{{"items": [ {items} ] }}"#);
    res
}

pub fn get_mock_instances_delete(
    space: &str,
    node_external_ids: &[&str],
    edge_external_ids: &[&str],
) -> Vec<NodeOrEdgeSpecification> {
    let mut instances = Vec::new();
    node_external_ids.iter().for_each(|id| {
        instances.push(NodeOrEdgeSpecification::Node(ItemId {
            space: space.to_owned(),
            external_id: id.to_string(),
        }));
    });
    edge_external_ids.iter().for_each(|id| {
        instances.push(NodeOrEdgeSpecification::Edge(ItemId {
            space: space.to_owned(),
            external_id: id.to_string(),
        }));
    });
    instances
}

pub fn get_instances_delete_request_string(
    space: &str,
    node_external_ids: &[&str],
    edge_external_ids: &[&str],
) -> String {
    let mut items = "".to_string();
    node_external_ids.iter().for_each(|id| {
        items.push_str(&format!(
            r#"{{"instanceType": "node", "externalId": "{id}", "space": "{space}" }},"#,
        ));
    });
    edge_external_ids.iter().for_each(|id| {
        items.push_str(&format!(
            r#"{{"instanceType": "edge", "externalId": "{id}", "space": "{space}" }},"#,
        ));
    });
    items.pop(); // remove last comma

    format!(r#"{{"items": [ {items} ] }}"#)
}

pub fn get_instances_delete_response_string(
    space: &str,
    node_external_ids: &[&str],
    edge_external_ids: &[&str],
) -> String {
    let mut items = "".to_string();
    node_external_ids.iter().for_each(|id| {
        items.push_str(&format!(
            r#"{{"instanceType": "node", "externalId": "{id}", "space": "{space}" }},"#,
        ));
    });
    edge_external_ids.iter().for_each(|id| {
        items.push_str(&format!(
            r#"{{"instanceType": "edge", "externalId": "{id}", "space": "{space}" }},"#,
        ));
    });
    items.pop(); // remove last comma

    format!(r#"{{"items": [ {items} ] }}"#)
}

pub fn get_edge_create_request(space: &str) -> String {
    format!(
        r#"
        {{
          "items": [
            {{
              "instanceType": "edge",
              "space": "{space}",
              "type": {{ "space": "{space}", "externalId": "typeNode" }},
              "externalId": "edge1",
              "startNode": {{ "space": "{space}", "externalId": "startNode" }},
              "endNode": {{ "space": "{space}", "externalId": "endNode" }},
              "sources": [
                {{
                  "source": {{
                    "type": "view",
                    "space": "{space}",
                    "externalId": "View",
                    "version": "1"
                  }},
                  "properties": {{ "string_field": "string value", "numeric_field": 42 }}
                }}
              ]
            }}
          ]
        }}
        "#
    )
}

pub fn get_edge_create_response(space: &str, external_id: &str) -> String {
    format!(
        r#"
          {{

            "items": 

        [

                {{
                    "instanceType": "edge",
                    "version": 0,
                    "wasModified": true,
                    "space": "{space}",
                    "externalId": "{external_id}",
                    "createdTime": 1730204346000,
                    "lastUpdatedTime": 1730204346000
                }}
            ]

        }}  
        "#
    )
}

pub fn get_edge_query_request() -> String {
    format!(
        r#"
            {{
              "with": {{ "edge_query": {{ "edges": {{}} }} }},
              "select": {{
                "edge_query": {{
                  "sources": [
                    {{
                      "source": {{
                        "type": "view",
                        "space": "my_space",
                        "externalId": "View",
                        "version": "1"
                      }},
                      "properties": ["*"]
                    }}
                  ]
                }}
              }}
            }}
        "#
    )
}

pub fn get_edge_query_response() -> String {
    format!(
        r#"
            {{
              "items": {{
                "edge_query": [
                  {{
                    "instanceType": "edge",
                    "version": 1,
                    "type": {{
                      "space": "my_space",
                      "externalId": "typeNode"
                    }},
                    "space": "my_space",
                    "externalId": "edge1",
                    "createdTime": 1734513651563,
                    "lastUpdatedTime": 1734514198150,
                    "startNode": {{
                      "space": "my_space",
                      "externalId": "startNode"
                    }},
                    "endNode": {{
                      "space": "my_space",
                      "externalId": "endNode"
                    }},
                    "properties": {{
                      "my_space": {{
                        "View/1": {{
                          "string_field": "string_value",
                          "numeric_field": 42
                        }}
                      }}
                    }}
                  }}
                ]
              }},
              "nextCursor": {{
                "my_query": "Z0FBQUFBQ"
              }}
            }}
        "#
    )
}

pub fn get_views_list_views_response() -> &'static str {
    r#"{
        "items": [
          {
            "externalId": "MyView",
            "space": "MySpace",
            "version": "1",
            "createdTime": 1679040460082,
            "lastUpdatedTime": 1679040460082,
            "writable": true,
            "usedFor": "node",
            "properties": {
              "name": {
                "type": {
                  "type": "text",
                  "list": false,
                  "collation": "ucs_basic"
                },
                "container": {
                  "type": "container",
                  "space": "APM_Config",
                  "externalId": "InRobotConfiguration"
                },
                "containerPropertyIdentifier": "name",
                "nullable": true,
                "autoIncrement": false,
                "name": "name"
              },
              "roboticsSpaceVersion": {
                "type": {
                  "type": "int32",
                  "list": false
                },
                "container": {
                  "type": "container",
                  "space": "APM_Config",
                  "externalId": "InRobotConfiguration"
                },
                "containerPropertyIdentifier": "roboticsSpaceVersion",
                "nullable": false,
                "autoIncrement": false,
                "name": "roboticsSpaceVersion"
              }
            },
            "name": "MyView",
            "implements": []
          }
        ]
      }"#
}

pub fn get_views_retrieve_views_request() -> &'static str {
    r#"{
        "items": [
            {
                "externalId": "MyView", 
                "space":"MySpace",
                "version": "1"
            }
        ]
    }"#
}
