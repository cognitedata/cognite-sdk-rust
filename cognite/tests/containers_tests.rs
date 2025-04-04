#![cfg(feature = "integration_tests")]

use std::collections::HashMap;

#[cfg(test)]
use cognite::models::*;
use cognite::{models::containers::ContainerQuery, *};

use containers::{
    ContainerCreate, ContainerIndex, ContainerPropertyDefinition, ContainerPropertyType,
};

mod common;
pub use common::*;

mod fixtures;
pub use fixtures::*;

#[tokio::test]
async fn create_retrieve_delete_container() {
    let container_external_id = PREFIX.as_str().replace('-', "_").to_string();
    let new_container = ContainerCreate {
        constraints: HashMap::new(),
        space: std::env::var("CORE_DM_TEST_SPACE").unwrap(),
        description: None,
        external_id: container_external_id.clone(),
        name: Some("Container1-Name".to_string()),
        used_for: Some(cognite::models::UsedFor::Node),
        indexes: HashMap::from([(
            "index1".to_string(),
            ContainerIndex::Btree {
                properties: vec!["field1".to_string()],
                cursorable: None,
            },
        )]),
        properties: HashMap::from([(
            "field1".to_string(),
            ContainerPropertyDefinition {
                name: None,
                nullable: Some(true),
                default_value: None,
                description: None,
                auto_increment: None,
                r#type: ContainerPropertyType::Text(TextProperty {
                    list: None,
                    collation: None,
                }),
            },
        )]),
    };
    let client = get_client();
    let container_created = client
        .models
        .containers
        .create(&[new_container])
        .await
        .unwrap();
    assert!(container_created.len() == 1);

    let containers_retrieved = client
        .models
        .containers
        .retrieve(&[ItemId {
            space: std::env::var("CORE_DM_TEST_SPACE").unwrap(),
            external_id: container_external_id.clone(),
        }])
        .await
        .unwrap();

    assert!(containers_retrieved.first().unwrap().name == Some("Container1-Name".to_string()));

    let container_deleted = client
        .models
        .containers
        .delete(&[ItemId {
            space: std::env::var("CORE_DM_TEST_SPACE").unwrap(),
            external_id: container_external_id,
        }])
        .await
        .unwrap();

    assert!(container_deleted.items.len() == 1);
}

#[tokio::test]
async fn test_list_containers() {
    let client = get_client();
    let containers = client
        .models
        .containers
        .list(Some(ContainerQuery {
            limit: Some(15),
            include_global: Some(true),
            ..Default::default()
        }))
        .await
        .unwrap();
    // There are more than 15 system containers, which are included due to include_global.
    assert_eq!(15, containers.items.len());
}
