use cognite::assets::{AddAsset, FilterAssetsRequest};
use cognite::datasets::AddDataSet;
use cognite::labels::{AddLabel, LabelFilter};
use cognite::raw::RawRowCreate;
use cognite::relationships::{AddRelationship, RelationshipVertexType};
use cognite::Identity;
use cognite_mock::resource::relationships::RelationshipFilter;
use cognite_mock::MockCogniteClient;

// ---------------------------------------------------------------------------
// Labels
// ---------------------------------------------------------------------------

#[tokio::test]
async fn labels_crud() {
    let c = MockCogniteClient::new();

    c.labels
        .create(&[
            AddLabel {
                external_id: "lbl-a".into(),
                name: "Alpha".into(),
                ..Default::default()
            },
            AddLabel {
                external_id: "lbl-b".into(),
                name: "Beta".into(),
                ..Default::default()
            },
        ])
        .await
        .unwrap();

    let page = c
        .labels
        .list(
            Some(LabelFilter {
                external_id_prefix: Some("lbl-".into()),
                ..Default::default()
            }),
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(page.items.len(), 2);

    c.labels
        .delete(&["lbl-a".to_string()], false)
        .await
        .unwrap();

    let page2 = c.labels.list(None, None, None).await.unwrap();
    assert_eq!(page2.items.len(), 1);
    assert_eq!(page2.items[0].name, "Beta");
}

// ---------------------------------------------------------------------------
// Relationships
// ---------------------------------------------------------------------------

#[tokio::test]
async fn relationships_crud() {
    let c = MockCogniteClient::new();

    c.relationships
        .create(&[AddRelationship {
            external_id: "rel-1".into(),
            source_external_id: "asset-a".into(),
            source_type: RelationshipVertexType::Asset,
            target_external_id: "asset-b".into(),
            target_type: RelationshipVertexType::Asset,
            ..Default::default()
        }])
        .await
        .unwrap();

    let got = c
        .relationships
        .retrieve(&["rel-1".to_string()], false)
        .await
        .unwrap();
    assert_eq!(got[0].source_external_id, "asset-a");

    // filter
    let page = c
        .relationships
        .filter(
            Some(RelationshipFilter {
                source_external_ids: Some(vec!["asset-a".into()]),
                ..Default::default()
            }),
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(page.items.len(), 1);

    // duplicate → error
    let err = c
        .relationships
        .create(&[AddRelationship {
            external_id: "rel-1".into(),
            source_external_id: "x".into(),
            source_type: RelationshipVertexType::Asset,
            target_external_id: "y".into(),
            target_type: RelationshipVertexType::Asset,
            ..Default::default()
        }])
        .await;
    assert!(err.is_err());

    c.relationships
        .delete(&["rel-1".to_string()], false)
        .await
        .unwrap();

    let gone = c
        .relationships
        .retrieve(&["rel-1".to_string()], true)
        .await
        .unwrap();
    assert!(gone.is_empty());
}

// ---------------------------------------------------------------------------
// Data sets
// ---------------------------------------------------------------------------

#[tokio::test]
async fn datasets_crud() {
    let c = MockCogniteClient::new();

    let created = c
        .data_sets
        .create(&[AddDataSet {
            external_id: Some("ds-1".into()),
            name: Some("My Dataset".into()),
            ..Default::default()
        }])
        .await
        .unwrap();
    assert_eq!(created[0].name.as_deref(), Some("My Dataset"));

    let got = c
        .data_sets
        .retrieve(&[Identity::external_id("ds-1")], false)
        .await
        .unwrap();
    assert_eq!(got[0].external_id.as_deref(), Some("ds-1"));

    let id = created[0].id;
    let page = c.data_sets.filter(None, None, None).await.unwrap();
    assert_eq!(page.items.len(), 1);

    c.data_sets
        .delete(&[Identity::Id { id }], false)
        .await
        .unwrap();
    let empty = c.data_sets.filter(None, None, None).await.unwrap();
    assert!(empty.items.is_empty());
}

// ---------------------------------------------------------------------------
// RAW
// ---------------------------------------------------------------------------

#[tokio::test]
async fn raw_db_table_rows() {
    let c = MockCogniteClient::new();

    c.raw.create_db("mydb").await.unwrap();
    c.raw.create_table("mydb", "t1").await.unwrap();

    let dbs = c.raw.list_dbs().await.unwrap();
    assert!(dbs.contains(&"mydb".to_string()));

    let tables = c.raw.list_tables("mydb").await.unwrap();
    assert!(tables.contains(&"t1".to_string()));

    c.raw
        .insert_rows(
            "mydb",
            "t1",
            &[
                RawRowCreate {
                    key: "k1".into(),
                    columns: serde_json::json!({"x": 1}),
                },
                RawRowCreate {
                    key: "k2".into(),
                    columns: serde_json::json!({"x": 2}),
                },
            ],
        )
        .await
        .unwrap();

    let rows = c
        .raw
        .retrieve_rows("mydb", "t1", &["k1".to_string()])
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].columns["x"], 1);

    let page = c.raw.list_rows("mydb", "t1", None, None).await.unwrap();
    assert_eq!(page.items.len(), 2);

    c.raw
        .delete_rows("mydb", "t1", &["k1".to_string()])
        .await
        .unwrap();
    let after = c.raw.list_rows("mydb", "t1", None, None).await.unwrap();
    assert_eq!(after.items.len(), 1);

    // table not found → error
    let err = c.raw.insert_rows("mydb", "ghost", &[]).await;
    assert!(err.is_err());

    c.raw.delete_table("mydb", "t1", false).await.unwrap();
    c.raw.delete_db("mydb", false).await.unwrap();
    assert!(c.raw.list_dbs().await.unwrap().is_empty());
}

// ---------------------------------------------------------------------------
// Reset
// ---------------------------------------------------------------------------

#[tokio::test]
async fn reset_clears_all_stores() {
    let c = MockCogniteClient::new();
    c.assets
        .create(&[AddAsset {
            name: "X".into(),
            ..Default::default()
        }])
        .await
        .unwrap();
    c.labels
        .create(&[AddLabel {
            external_id: "l".into(),
            name: "L".into(),
            ..Default::default()
        }])
        .await
        .unwrap();

    c.reset().await;

    let assets = c
        .assets
        .filter_all(FilterAssetsRequest::default())
        .await
        .unwrap();
    assert!(assets.is_empty());
    let labels = c.labels.list(None, None, None).await.unwrap();
    assert!(labels.items.is_empty());
}
