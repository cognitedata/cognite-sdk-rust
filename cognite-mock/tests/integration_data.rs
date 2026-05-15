use cognite::assets::{AddAsset, AssetFilter, AssetSearch, FilterAssetsRequest};
use cognite::events::{AddEvent, EventFilter, EventFilterQuery, EventSearch};
use cognite::files::AddFile;
use cognite::time_series::{AddTimeSeries, TimeSeriesFilter, TimeSeriesFilterRequest};
use cognite::{Identity, Patch};
use cognite_mock::MockCogniteClient;

// ---------------------------------------------------------------------------
// Assets
// ---------------------------------------------------------------------------

#[tokio::test]
async fn assets_crud() {
    let c = MockCogniteClient::new();

    let created = c
        .assets
        .create(&[
            AddAsset {
                name: "Pump A".into(),
                external_id: Some("pump-a".into()),
                ..Default::default()
            },
            AddAsset {
                name: "Valve B".into(),
                external_id: Some("valve-b".into()),
                ..Default::default()
            },
        ])
        .await
        .unwrap();
    assert_eq!(created.len(), 2);
    assert_eq!(created[0].name, "Pump A");

    // retrieve by external id
    let got = c
        .assets
        .retrieve(&[Identity::external_id("pump-a")], false)
        .await
        .unwrap();
    assert_eq!(got[0].name, "Pump A");

    // duplicate external_id → error
    let err = c
        .assets
        .create(&[AddAsset {
            name: "Dup".into(),
            external_id: Some("pump-a".into()),
            ..Default::default()
        }])
        .await;
    assert!(err.is_err());

    // update
    let id = created[0].id;
    let mut patch: Patch<cognite::assets::PatchAsset> = created[0].clone().into();
    patch.update.description = Some(cognite::UpdateSetNull::Set {
        set: "updated".into(),
    });
    let updated = c.assets.update(&[patch]).await.unwrap();
    assert_eq!(updated[0].description.as_deref(), Some("updated"));

    // filter
    let results = c
        .assets
        .filter(FilterAssetsRequest {
            filter: Some(AssetFilter {
                name: Some("Pump A".into()),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(results.items.len(), 1);

    // search
    let results = c
        .assets
        .search(
            AssetFilter::default(),
            AssetSearch {
                name: Some("pump".into()),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(results.len(), 1);

    // delete unknown without ignore → error
    let err = c
        .assets
        .delete(&[Identity::external_id("nonexistent")], false)
        .await;
    assert!(err.is_err());

    // delete with ignore_unknown
    c.assets
        .delete(&[Identity::external_id("nonexistent")], true)
        .await
        .unwrap();

    // delete for real
    c.assets
        .delete(&[Identity::Id { id }], false)
        .await
        .unwrap();
    let missing = c
        .assets
        .retrieve(&[Identity::Id { id }], true)
        .await
        .unwrap();
    assert!(missing.is_empty());
}

#[tokio::test]
async fn assets_filter_all_pagination() {
    let c = MockCogniteClient::new();
    let adds: Vec<AddAsset> = (0..25)
        .map(|i| AddAsset {
            name: format!("Asset {}", i),
            external_id: Some(format!("a-{}", i)),
            ..Default::default()
        })
        .collect();
    c.assets.create(&adds).await.unwrap();

    let all = c
        .assets
        .filter_all(FilterAssetsRequest::default())
        .await
        .unwrap();
    assert_eq!(all.len(), 25);
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

#[tokio::test]
async fn events_crud() {
    let c = MockCogniteClient::new();

    let created = c
        .events
        .create(&[AddEvent {
            external_id: Some("evt-1".into()),
            r#type: Some("alarm".into()),
            ..Default::default()
        }])
        .await
        .unwrap();
    assert_eq!(created.len(), 1);

    // retrieve
    let got = c
        .events
        .retrieve(&[Identity::external_id("evt-1")], false)
        .await
        .unwrap();
    assert_eq!(got[0].r#type.as_deref(), Some("alarm"));

    // filter
    let page = c
        .events
        .filter(EventFilterQuery {
            filter: Some(EventFilter {
                r#type: Some("alarm".into()),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(page.items.len(), 1);

    // search by description — event has no description, so no match
    let results = c
        .events
        .search(
            EventFilter::default(),
            EventSearch {
                description: Some("alarm".into()),
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(results.len(), 0);

    // delete
    c.events
        .delete(&[Identity::external_id("evt-1")], false)
        .await
        .unwrap();
    let gone = c
        .events
        .retrieve(&[Identity::external_id("evt-1")], true)
        .await
        .unwrap();
    assert!(gone.is_empty());
}

// ---------------------------------------------------------------------------
// Time series
// ---------------------------------------------------------------------------

#[tokio::test]
async fn time_series_crud() {
    let c = MockCogniteClient::new();

    let created = c
        .time_series
        .create(&[AddTimeSeries {
            external_id: Some("ts-1".into()),
            name: Some("Pressure".into()),
            is_string: false,
            is_step: false,
            ..Default::default()
        }])
        .await
        .unwrap();
    assert_eq!(created[0].name.as_deref(), Some("Pressure"));

    // filter
    let page = c
        .time_series
        .filter(TimeSeriesFilterRequest {
            filter: Some(TimeSeriesFilter {
                name: Some("Pressure".into()),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(page.items.len(), 1);

    // search
    let results = c
        .time_series
        .search(
            TimeSeriesFilter::default(),
            cognite::time_series::TimeSeriesSearch {
                name: Some("press".into()),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
    assert_eq!(results.len(), 1);

    // delete
    let id = created[0].id;
    c.time_series
        .delete(&[Identity::Id { id }], false)
        .await
        .unwrap();
}

// ---------------------------------------------------------------------------
// Files
// ---------------------------------------------------------------------------

#[tokio::test]
async fn files_upload_download() {
    let c = MockCogniteClient::new();

    let meta = c
        .files
        .create_metadata(&[AddFile {
            name: "report.pdf".into(),
            external_id: Some("report-1".into()),
            ..Default::default()
        }])
        .await
        .unwrap();
    assert!(!meta[0].uploaded);

    let identity = Identity::external_id("report-1");
    c.files
        .upload(&identity, b"PDF content".to_vec())
        .await
        .unwrap();

    let got = c.files.retrieve(&[identity.clone()], false).await.unwrap();
    assert!(got[0].uploaded);

    let bytes = c.files.download(&identity).await.unwrap();
    assert_eq!(bytes, b"PDF content");

    // download before upload → error
    let c2 = MockCogniteClient::new();
    c2.files
        .create_metadata(&[AddFile {
            name: "empty.txt".into(),
            external_id: Some("e-1".into()),
            ..Default::default()
        }])
        .await
        .unwrap();
    let err = c2.files.download(&Identity::external_id("e-1")).await;
    assert!(err.is_err());
}
