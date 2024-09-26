use cognite::{
    models::{
        data_models::{CogniteExtractorFile, FileProperties, RetrieveExtendedCollection, UpsertExtendedCollection},
        instances::{NodeOrEdgeSpecification, SourceReferenceInternal},
        views::ViewReference,
        ItemId,
    },
    CogniteClient, RetrieveWithRequest,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let client = CogniteClient::new_oidc("testing_instances", None).unwrap();
    let external_id = Uuid::new_v4().to_string();
    let space = "core-dm-test".to_string();
    let name = "random".to_string();
    let col = CogniteExtractorFile::new(space.clone(), external_id, name);
    let res = client
        .models
        .files
        .upsert(vec![col], None, None, None, None, None)
        .await
        .unwrap();
    let external_id = "01483fc4-e4d6-4ed9-8950-96f100baccf2".to_string();
    let res: Vec<CogniteExtractorFile> = client
        .models
        .files
        .retrieve(vec![NodeOrEdgeSpecification::Node(ItemId {
            space,
            external_id,
        })])
        .await
        .unwrap();
    println!("{res:#?}");
}
