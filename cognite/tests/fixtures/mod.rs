use std::collections::HashMap;

use cognite::models::{
    DirectRelationReference, EdgeOrNodeData, EdgeWrite, InstanceInfo, InstanceType,
    NodeOrEdgeCreate, NodeWrite, SourceReference, SourceReferenceType,
};

fn get_mock_properties() -> HashMap<String, String> {
    let mut properties = std::collections::HashMap::new();
    properties.insert("key1".to_string(), "value1".to_string());
    properties.insert("key2".to_string(), "value2".to_string());
    properties
}

pub(crate) fn get_mock_instances(
    space: &str,
    node_external_id: &[&str],
    edge_external_id: &[&str],
) -> Vec<NodeOrEdgeCreate<HashMap<std::string::String, std::string::String>>> {
    let properties = get_mock_properties();

    let mut mock_instances: Vec<NodeOrEdgeCreate<HashMap<String, String>>> = Vec::new();

    // add nodes
    mock_instances.extend(
        node_external_id
            .iter()
            .map(|id| {
                NodeOrEdgeCreate::Node(NodeWrite {
                    instance_type: InstanceType::Node,
                    space: space.to_owned(),
                    external_id: id.to_string(),
                    sources: Some(vec![EdgeOrNodeData {
                        source: SourceReference {
                            r#type: SourceReferenceType::View,
                            space: space.to_owned(),
                            external_id: "some_view".to_string(),
                            version: "1".to_string(),
                        },
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
                    instance_type: InstanceType::Edge,
                    r#type: DirectRelationReference {
                        space: space.to_owned(),
                        external_id: id.to_string(),
                    },
                    space: space.to_owned(),
                    external_id: id.to_string(),
                    start_node: DirectRelationReference {
                        space: space.to_owned(),
                        external_id: "start_node".to_string(),
                    },
                    end_node: DirectRelationReference {
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

pub(crate) fn get_instances_create_request_string(
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

    let req = format!(r#"{{"items": [{items}] }}"#);
    req
}

pub(crate) fn get_instances_create_response_string(
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

pub(crate) fn get_mock_instances_delete(
    space: &str,
    node_external_ids: &[&str],
    edge_external_ids: &[&str],
) -> Vec<InstanceInfo> {
    let mut instances = Vec::new();
    node_external_ids.iter().for_each(|id| {
        instances.push(InstanceInfo {
            instance_type: InstanceType::Node,
            space: space.to_owned(),
            external_id: id.to_string(),
        });
    });
    edge_external_ids.iter().for_each(|id| {
        instances.push(InstanceInfo {
            instance_type: InstanceType::Edge,
            space: space.to_owned(),
            external_id: id.to_string(),
        });
    });
    instances
}

pub(crate) fn get_instances_delete_request_string(
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

pub(crate) fn get_instances_delete_response_string(
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
