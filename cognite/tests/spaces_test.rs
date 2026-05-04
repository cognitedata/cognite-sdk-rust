#![cfg(feature = "integration_tests")]

#[cfg(test)]
use cognite::models::*;
use cognite::{
    models::spaces::{SpaceCreate, SpaceQuery},
    CogniteClient,
};
use cognite::{Create, List, Retrieve};

mod common;

use cognite::*;
mod fixtures;
pub use fixtures::*;

#[tokio::test]
async fn create_retrieve_delete_spaces() {
    let _permit = common::CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let space_id = format!("{}-space-1", common::PREFIX.as_str());
    let client = common::get_client();
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
async fn list_non_global_spaces() {
    let _permit = common::CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let client: CogniteClient = common::get_client();
    let spaces = client
        .models
        .spaces
        .list(Some(SpaceQuery::default()))
        .await
        .unwrap();
    let spaces_list = client.models.spaces.list(None).await.unwrap();
    assert_eq!(spaces.items.len(), spaces_list.items.len());
}

#[tokio::test]
async fn list_global_spaces() {
    let _permit = common::CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let client: CogniteClient = common::get_client();
    let spaces = client
        .models
        .spaces
        .list(Some(SpaceQuery {
            limit: Some(10),
            include_global: Some(true),
            cursor: None,
        }))
        .await
        .unwrap();
    assert_eq!(spaces.items.len(), 10);
}

#[tokio::test]
async fn cursoring_global_spaces() {
    let _permit = common::CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let client: CogniteClient = common::get_client();
    let spaces = client
        .models
        .spaces
        .list_all(SpaceQuery {
            limit: Some(10),
            include_global: Some(true),
            cursor: None,
        })
        .await
        .unwrap();
    assert!(spaces.len() > 11);
}
