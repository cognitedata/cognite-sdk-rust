use cognite::models::*;
use cognite::*;

use crate::common::get_client_for_mocking;

#[tokio::test]
async fn create_and_delete_instances() {
    let space = "my_space".to_string();
    let node_external_id = "my_node".to_string();
    let view_external_id = "my_view".to_string();
    let edge_external_id = "my_edge".to_string();

    let client = get_client_for_mocking();

    let space = "space".to_string();

    let mut hm = std::collections::HashMap::new();
    hm.insert("key1", "value1");
    hm.insert("key2", "value2");

    let upsert_collection = NodeAndEdgeCreateCollection {
        items: vec![
            NodeOrEdgeCreate::Node(NodeWrite {
                instance_type: InstanceType::Node,
                space: space.to_owned(),
                external_id: node_external_id,
                sources: Some(vec![EdgeOrNodeData {
                    source: SourceReference {
                        r#type: "type".to_string(),
                        space: space.to_owned(),
                        external_id: view_external_id,
                        version: "1".to_string(),
                    },
                    properties: hm.clone(),
                }]),
                ..Default::default()
            }),
            NodeOrEdgeCreate::Edge(EdgeWrite {
                instance_type: InstanceType::Edge,
                r#type: EdgeType {
                    space: space.to_owned(),
                    external_id: edge_external_id,
                },
                space: space.to_owned(),
                external_id: edge_external_id.to_string(),
                start_node: DirectRelationReference {
                    space: space.to_owned(),
                    external_id: node_external_id.to_string(),
                },
                end_node: DirectRelationReference {
                    space: space.to_owned(),
                    external_id: node_external_id.to_string(),
                },
                ..Default::default()
            }),
        ],
        ..Default::default()
    };

    println!("{}", serde_json::to_string_pretty(&upserts).unwrap());

    // let result = client
    //     .models
    //     .instances
    //     .upsert(&upserts)
    //     .await
    //     .unwrap();

    // let id = format!("{}-ts1", PREFIX.as_str());
    // let time_serie = TimeSerie {
    //     name: Some("name".to_string()),
    //     external_id: Some(id),
    //     is_string: false,
    //     is_step: true,
    //     description: Some("description".to_string()),
    //     ..Default::default()
    // };
    // let client = get_client();
    // let mut time_series = client
    //     .time_series
    //     .create_from(&vec![time_serie])
    //     .await
    //     .unwrap();
    // assert_eq!(time_series.len(), 1);
    // for time_serie in time_series.iter_mut() {
    //     time_serie.description = Some(String::from("changed"));
    // }

    // let time_series = client.time_series.update_from(&time_series).await.unwrap();

    // let id_list: Vec<Identity> = time_series
    //     .iter()
    //     .map(|ts| Identity::Id { id: ts.id })
    //     .collect();
    // client.time_series.delete(&id_list, true).await.unwrap();
}
