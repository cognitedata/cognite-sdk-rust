use cognite::models::*;
use cognite::*;

use crate::common::*;

#[tokio::test]
async fn create_and_delete_instances() {
    // let client = get_client();

    let space = "space".to_string();

    let mut hm = std::collections::HashMap::new();
    hm.insert("key1", "value1");
    hm.insert("key2", "value2");

    let upserts = vec![
        NodeOrEdgeCreate::Node(NodeWrite {
            space: space.to_owned(),
            external_id: "my_node".to_string(),
            sources: vec![EdgeOrNodeData {
                source: SourceReference {
                    r#type: "type".to_string(),
                    space: space.to_owned(),
                    external_id: "my_view".to_string(),
                    version: "1".to_string(),
                },
                properties: hm.clone(),
            }],
            instance_type: InstanceType::Node,
        }),
        NodeOrEdgeCreate::Edge(EdgeWrite {
            r#type: EdgeType {
                space: space.to_owned(),
                external_id: "my_edge".to_string(),
            },
            space: space.to_owned(),
            external_id: "my_node".to_string(),
            sources: vec![EdgeOrNodeData {
                source: SourceReference {
                    r#type: "type".to_string(),
                    space: space.to_owned(),
                    external_id: "my_view".to_string(),
                    version: "1".to_string(),
                },
                properties: hm,
            }],
            instance_type: InstanceType::Edge,
            start_node: DirectRelationReference {
                space: space.to_owned(),
                external_id: "my_node".to_string(),
            },
            end_node: DirectRelationReference {
                space: space.to_owned(),
                external_id: "my_node".to_string(),
            },
        }),
    ];

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
