use cognite::{
    models::{
        instances::{CogniteExtractorFile, FileObject, NodeOrEdgeSpecification},
        ItemId,
    },
    CogniteClient, DeleteWithResponse,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let client = CogniteClient::new_oidc("testing_instances", None).unwrap();
    let external_id = Uuid::new_v4().to_string();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();
    let col = CogniteExtractorFile::new(
        space.to_string(),
        external_id,
        FileObject {
            ..Default::default()
        },
    );
    let res = client
        .models
        .instances
        .apply(&[col], None, None, None, None, false)
        .await
        .unwrap();
    let external_id = match res.first().unwrap() {
        cognite::models::instances::SlimNodeOrEdge::Node(slim_node_definition) => {
            &slim_node_definition.external_id
        }
        cognite::models::instances::SlimNodeOrEdge::Edge(_) => {
            panic!("Invalid type received.")
        }
    };
    let node_specs = NodeOrEdgeSpecification::Node(ItemId {
        space: space.to_string(),
        external_id: external_id.clone(),
    });
    let res: Vec<CogniteExtractorFile> = client
        .models
        .instances
        .fetch(&[node_specs.clone()], None)
        .await
        .unwrap();
    println!("{res:#?}");
    client.models.instances.delete(&[node_specs]).await.unwrap();
}
