use cognite::models::*;
use cognite::*;
mod common;
use cognite::models::spaces::SpaceCreate;
use common::*;

#[tokio::test]
async fn create_retrieve_delete_spaces() {
    let space_id = format!("{}-space-1", PREFIX.as_str());
    let client = get_client();
    let new_space = SpaceCreate {
        space: space_id.clone(),
        description: Some("Some description".to_owned()),
        name: Some("Test space".to_owned()),
        intentionally_breaking: Some(false),
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
