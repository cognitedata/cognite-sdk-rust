#![cfg(feature = "integration_tests")]

mod common;

use cognite::models::*;

use cognite::*;

use common::*;
use instances::{
    CogniteExtractorFile, CogniteTimeseries, ExtractorFileObject, NodeOrEdgeSpecification,
    SlimNodeOrEdge, Timeseries,
};
use uuid::Uuid;

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
