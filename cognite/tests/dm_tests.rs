#![cfg(feature = "integration_tests")]

#[cfg(test)]
use cognite::{
    models::{
        data_models::{CogniteExtractorFile, RetrieveExtendedCollection, UpsertExtendedCollection},
        instances::{NodeOrEdgeSpecification, SlimNodeOrEdge},
        ItemId,
    },
    CogniteClient, DeleteWithResponse,
};
use uuid::Uuid;

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
