use cognite::assets::*;
use cognite::*;
mod common;
pub(crate) use common::*;

#[tokio::test]
async fn create_and_delete_asset() {
    let asset_id = format!("{}-asset1", PREFIX.as_str());
    let client = get_client();
    let new_asset: Asset = Asset::new("asset1", "description", Some(asset_id), None, None, None);
    let assets = client.assets.create_from(&vec![new_asset]).await.unwrap();
    assert_eq!(assets.len(), 1);
    let asset = assets.get(0).unwrap();
    assert_eq!(asset.name, "asset1");

    let asset_ids: Vec<Identity> = assets.iter().map(|a| Identity::from(a.id)).collect();
    client
        .assets
        .delete(&DeleteAssetsRequest {
            items: asset_ids,
            ignore_unknown_ids: true,
            recursive: true,
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn create_update_and_delete_asset() {
    let asset_id = format!("{}-asset2", PREFIX.as_str());
    let new_asset: Asset = Asset::new("asset2", "description 1", Some(asset_id), None, None, None);

    let asset_id_2 = format!("{}-asset3", PREFIX.as_str());
    let new_asset_2: Asset = Asset::new(
        "asset3",
        "description 2",
        Some(asset_id_2),
        None,
        None,
        None,
    );

    let client = get_client();

    let mut assets = client
        .assets
        .create_from(&vec![new_asset, new_asset_2])
        .await
        .unwrap();

    assert_eq!(assets.len(), 2);
    for asset in assets.iter_mut() {
        asset.description = Some(String::from("changed"));
    }

    let assets = client.assets.update_from(&assets).await.unwrap();
    for asset in assets.iter() {
        assert_eq!(asset.description, Some(String::from("changed")));
    }

    let asset_ids: Vec<Identity> = assets.iter().map(|a| Identity::from(a.id)).collect();
    client
        .assets
        .delete(&DeleteAssetsRequest {
            items: asset_ids,
            ignore_unknown_ids: true,
            recursive: true,
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn create_update_ignore_missing() {
    let asset_id = format!("{}-asset4", PREFIX.as_str());
    let new_asset: Asset = Asset::new(
        "asset3",
        "description 1",
        Some(asset_id.clone()),
        None,
        None,
        None,
    );

    let asset_id_2 = format!("{}-asset5", PREFIX.as_str());
    let new_asset_2: Asset = Asset::new(
        "asset3",
        "description 2",
        Some(asset_id_2.clone()),
        None,
        None,
        None,
    );

    let client = get_client();

    let mod_assets = vec![new_asset, new_asset_2];
    client.assets.create_from(&mod_assets[..1]).await.unwrap();

    client
        .assets
        .update_from_ignore_unknown_ids(&mod_assets)
        .await
        .unwrap();

    let ids = vec![
        Identity::ExternalId {
            external_id: asset_id,
        },
        Identity::ExternalId {
            external_id: asset_id_2,
        },
    ];
    client
        .assets
        .delete(&DeleteAssetsRequest {
            items: ids,
            ignore_unknown_ids: true,
            recursive: true,
        })
        .await
        .unwrap();
}
