// #![cfg(feature = "integration_tests")]

mod common;

use cognite::models::data_models::DataModelCreate;
use cognite::models::data_models::DataModelId;
use cognite::models::views::{ViewCreateOrReference, ViewReference};

use cognite::models::*;

use cognite::*;

use common::*;
use instances::{
    CogniteExtractorFile, CogniteTimeseries, ExtractorFileObject, NodeOrEdgeSpecification,
    SlimNodeOrEdge, Timeseries,
};
use uuid::Uuid;

#[tokio::test]
async fn create_delete_data_models() {
    let _permit = CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let client = get_client();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();
    let data_model_1 = DataModelCreate {
        space: space.clone(),
        external_id: "DM1".to_string(),
        version: "1".to_string(),
        name: None,
        description: None,
        views: Some(vec![ViewCreateOrReference::Reference(
            ViewReference {
                space: "cdf_cdm".to_string(),
                external_id: "CogniteAsset".to_string(),
                version: "v1".to_string(),
            }
            .into(),
        )]),
    };

    let data_model_2 = DataModelCreate {
        space: space.clone(),
        external_id: "DM2".to_string(),
        version: "1".to_string(),
        name: None,
        description: None,
        views: None,
    };
    let data_model_created = client
        .models
        .data_models
        .create(&[data_model_1, data_model_2])
        .await
        .unwrap();
    assert_eq!(data_model_created.len(), 2);

    let data_model_deleted = client
        .models
        .data_models
        .delete(&[
            DataModelId {
                space: space.clone(),
                external_id: "DM1".to_string(),
                version: Some("1".to_string()),
            },
            DataModelId {
                space: space.clone(),
                external_id: "DM2".to_string(),
                version: Some("1".to_string()),
            },
        ])
        .await
        .unwrap();
    assert_eq!(data_model_deleted.items.len(), 2);
}

#[tokio::test]
async fn create_and_delete_file_instance() {
    let _permit = CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let client = CogniteClient::new_oidc("testing_instances", None).unwrap();
    let external_id = Uuid::new_v4().to_string();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();
    let col = CogniteExtractorFile::new(
        space.to_string(),
        external_id.to_string(),
        ExtractorFileObject::new(),
    );
    let res = client
        .models
        .instances
        .apply(&[col], None, None, None, None, false)
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

    let mut backoff = Backoff::default();
    let mut deleted: Option<ItemsVec<NodeOrEdgeSpecification>> = None;
    for _ in 0..10 {
        match client.models.instances.delete(&[node_specs.clone()]).await {
            Ok(res) => {
                deleted = Some(res);
                break;
            }
            Err(_) => {
                tokio::time::sleep(backoff.next().unwrap()).await;
                continue;
            }
        }
    }
    let deleted = deleted.unwrap();
    let deleted = deleted.items.first().unwrap();
    assert!(matches!(deleted, NodeOrEdgeSpecification::Node(_)));
}

#[tokio::test]
async fn create_and_delete_timeseries_instance() {
    let _permit = CDM_CONCURRENCY_PERMITS.acquire().await.unwrap();
    let client = CogniteClient::new_oidc("testing_instances", None).unwrap();
    let external_id = Uuid::new_v4().to_string();
    let space = std::env::var("CORE_DM_TEST_SPACE").unwrap();

    let timeseries = CogniteTimeseries::new(
        space.to_string(),
        external_id.to_string(),
        Timeseries::new(false),
    );
    let timeseries_res = client
        .models
        .instances
        .apply(&[timeseries], None, None, None, None, false)
        .await
        .unwrap();
    let timeseries_res = timeseries_res.first().unwrap();
    assert!(matches!(timeseries_res, SlimNodeOrEdge::Node(_)));

    let node_specs = NodeOrEdgeSpecification::Node(ItemId {
        space: space.to_string(),
        external_id: external_id.to_string(),
    });
    let timeseries_retrieve: Vec<CogniteTimeseries> = client
        .models
        .instances
        .fetch(&[node_specs.clone()], None)
        .await
        .unwrap();
    let timeseries = timeseries_retrieve.first().unwrap();
    assert_eq!(external_id.to_string(), timeseries.id.external_id);

    let mut backoff = Backoff::default();
    let mut deleted: Option<ItemsVec<NodeOrEdgeSpecification>> = None;
    for _ in 0..10 {
        match client.models.instances.delete(&[node_specs.clone()]).await {
            Ok(res) => {
                deleted = Some(res);
                break;
            }
            Err(_) => {
                tokio::time::sleep(backoff.next().unwrap()).await;
                continue;
            }
        }
    }
    let deleted = deleted.unwrap();
    let deleted = deleted.items.first().unwrap();
    assert!(matches!(deleted, NodeOrEdgeSpecification::Node(_)));
}
