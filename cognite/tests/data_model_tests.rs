use cognite::models::*;
use cognite::*;
mod common;
use common::*;

#[tokio::test]
async fn create_retrieve_delete_spaces() {
    let space_id = format!("{}-space-1", PREFIX.as_str());
    let client = get_client();
    let created = create_space(&client, &space_id).await;

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

    let deleted = delete_space(&client, &space_id).await;
    assert_eq!(deleted.len(), 1);
    let space = &deleted[0];
    assert_eq!(space_id, space.space);
}
