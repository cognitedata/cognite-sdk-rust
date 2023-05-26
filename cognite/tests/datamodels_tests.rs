use cognite::models::*;
use cognite::*;
mod common;
use common::*;

#[tokio::test]
async fn create_retrieve_delete_datamodels() {
    let client = get_client();
    // create space
    let space_id = format!("{}-space-1", PREFIX.as_str());
    let space = create_space(&client, &space_id).await;
    let space = &space[0];

    // create datamodel
    let datamodel_id = "rust_sdk_test_datamodel_1".to_string();
    let datamodel_name = "Test datamodel".to_string();
    let datamodel_version = "1".to_string();
    let datamodelcreate = DataModelCreate {
        space: space.space.clone(),
        external_id: datamodel_id.clone(),
        description: Some("Test datamodel".to_string()),
        name: Some(datamodel_name.clone()),
        version: datamodel_version.clone(),
        views: None,
    };
    let created = client
        .models
        .datamodels
        .create(&[datamodelcreate])
        .await
        .unwrap();

    assert_eq!(created.len(), 1);
    let datamodel = &created[0];
    assert_eq!(datamodel.name, Some(datamodel_name.clone()));

    let retrieved = client
        .models
        .datamodels
        .retrieve(&[ItemIdOptionalVersion {
            space: space.space.clone(),
            external_id: datamodel_id.clone(),
            version: None,
        }])
        .await
        .unwrap();
    assert_eq!(retrieved.len(), 1);
    let datamodel = &retrieved[0];
    assert_eq!(datamodel.name, Some(datamodel_name));

    // cleanup
    let deleted = client
        .models
        .datamodels
        .delete(&[ItemIdWithVersion {
            space: space.space.clone(),
            external_id: datamodel_id,
            version: datamodel_version,
        }])
        .await
        .unwrap();
    assert_eq!(deleted.items.len(), 1);
    let space = &deleted.items[0];
    assert_eq!(space_id, space.space);

    delete_space(&client, &space_id).await;
}
