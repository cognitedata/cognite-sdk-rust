#![cfg(feature = "integration_tests")]

mod common;

use cognite::models::spaces::SpaceCreate;

use cognite::models::*;

use cognite::*;

use common::*;
use instances::{
    CogniteExtractorFile, CogniteTimeseries, CogniteUnit, InstanceId, NodeOrEdgeSpecification,
    SlimNodeOrEdge,
};
use uuid::Uuid;

#[tokio::test]
async fn create_retrieve_delete_spaces() {
    let space_id = format!("{}-space-1", PREFIX.as_str());
    let client = get_client();
    let new_space = SpaceCreate {
        space: space_id.clone(),
        description: Some("Some description".to_owned()),
        name: Some("Test space".to_owned()),
    };
    let created = client.models.spaces.create(&[new_space]).await.unwrap();
    assert_eq!(created.len(), 1);
    let space = &created[0];
    assert_eq!(space.name, Some("Test space".to_owned()));

    let retrieved = client
        .models
        .spaces
        .retrieve(&[SpaceId {
            space: space_id.clone(),
        }])
        .await
        .unwrap();
    assert_eq!(retrieved.len(), 1);
    let space = &retrieved[0];
    assert_eq!(space.name, Some("Test space".to_owned()));

    let deleted = client
        .models
        .spaces
        .delete(&[SpaceId {
            space: space_id.clone(),
        }])
        .await
        .unwrap();
    assert_eq!(deleted.items.len(), 1);
    let space = &deleted.items[0];
    assert_eq!(space_id, space.space);
}

#[tokio::test]
async fn create_and_delete_file_instance() {
    let client = CogniteClient::new_oidc("testing_instances", None).unwrap();
    let external_id = Uuid::new_v4().to_string();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();
    let name = "random".to_string();
    let col = CogniteExtractorFile::new(space.to_string(), external_id.to_string(), name);
    let res = client
        .models
        .instances
        .apply(vec![col], None, None, None, None, None)
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
        .instances
        .fetch(&[node_specs.clone()], None)
        .await
        .unwrap();
    let file = &res_retrieve[0];
    assert_eq!(external_id.to_string(), file.id.external_id);

    let res_delete = client.models.instances.delete(&[node_specs]).await.unwrap();
    let res_delete = res_delete.items.first().unwrap();
    assert!(matches!(res_delete, NodeOrEdgeSpecification::Node(_)));
}

#[tokio::test]
async fn create_and_delete_timeseries_instance() {
    let client = CogniteClient::new_oidc("testing_instances", None).unwrap();
    let external_id = Uuid::new_v4().to_string();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();
    let name = "random".to_string();
    let unit = CogniteUnit::new(space.to_string(), external_id.to_string(), name.to_string());
    let unit_res = client
        .models
        .instances
        .apply(vec![unit], None, None, None, None, None)
        .await
        .unwrap();
    let unit_res = unit_res.first().unwrap();
    assert!(matches!(unit_res, SlimNodeOrEdge::Node(_)));

    let unit_spec = NodeOrEdgeSpecification::Node(ItemId {
        space: space.to_string(),
        external_id: external_id.to_string(),
    });
    let unit_retrieve: Vec<CogniteUnit> = client
        .models
        .instances
        .fetch(&[unit_spec.clone()], None)
        .await
        .unwrap();
    let unit_retrieve = unit_retrieve.first().unwrap();
    println!("{:?}", unit_retrieve);
    assert_eq!(external_id.to_string(), unit_retrieve.id.external_id);

    let mut timeseries = CogniteTimeseries::new(
        space.to_string(),
        external_id.to_string(),
        name.to_string(),
        None,
    );
    let unit = match unit_res {
        SlimNodeOrEdge::Node(slim_node_definition) => slim_node_definition,
        SlimNodeOrEdge::Edge(_) => panic!("Invalid type for unit"),
    };
    timeseries.timeseries.unit = InstanceId {
        space: space.clone(),
        external_id: unit.external_id.to_owned(),
    };
    let timeseries_res = client
        .models
        .instances
        .apply(vec![timeseries], None, None, None, None, None)
        .await
        .unwrap();
    let timeseries_res = timeseries_res.first().unwrap();
    assert!(matches!(timeseries_res, SlimNodeOrEdge::Node(_)));

    let timeseries_spec = NodeOrEdgeSpecification::Node(ItemId {
        space: space.to_string(),
        external_id: external_id.to_string(),
    });
    let timeseries_retrieve: Vec<CogniteTimeseries> = client
        .models
        .instances
        .fetch(&[timeseries_spec.clone()], None)
        .await
        .unwrap();
    let timeseries = timeseries_retrieve.first().unwrap();
    assert_eq!(external_id.to_string(), timeseries.id.external_id);

    let timeseries_del = client
        .models
        .instances
        .delete(&[timeseries_spec])
        .await
        .unwrap();
    let timeseries_del = timeseries_del.items.first().unwrap();
    assert!(matches!(timeseries_del, NodeOrEdgeSpecification::Node(_)));

    // No need to ensure unit delete because it gets deleted with timeseries apparently.
}
