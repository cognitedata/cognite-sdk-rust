#![cfg(feature = "integration_tests")]

mod common;

#[cfg(test)]
use cognite::assets::*;
use cognite::{utils::lease::ResourceLease, *};
pub use common::*;

#[tokio::test]
async fn create_and_delete_asset() {
    let asset_id = format!("{}-asset1", PREFIX.as_str());
    let client = get_client();
    let new_asset: Asset = Asset::new("asset1", "description", Some(asset_id), None, None, None);
    let mut lease = ResourceLease::new_println(client.assets.clone());
    let assets = client.assets.create_from(&vec![new_asset]).await.unwrap();
    lease.add_resources(assets.clone());
    assert_eq!(assets.len(), 1);
    let asset = assets.first().unwrap();
    assert_eq!(asset.name, "asset1");

    lease.clean().await.unwrap();
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

    let mut lease = ResourceLease::new_println(client.assets.clone());
    lease.add_resources(assets.clone());
    assert_eq!(assets.len(), 2);
    for asset in assets.iter_mut() {
        asset.description = Some(String::from("changed"));
    }

    let assets = client.assets.update_from(&assets).await.unwrap();
    for asset in assets.iter() {
        assert_eq!(asset.description, Some(String::from("changed")));
    }

    lease.clean().await.unwrap();
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
    let mut lease = ResourceLease::new_println(client.assets.clone());
    lease.add_resources(client.assets.create_from(&mod_assets[..1]).await.unwrap());

    client
        .assets
        .update_from_ignore_unknown_ids(&mod_assets)
        .await
        .unwrap();

    lease.clean().await.unwrap();
}

#[tokio::test]
async fn upsert_assets() {
    let asset_id = format!("{}-asset6", PREFIX.as_str());
    let mut new_asset = Asset::new("asset6", "desc", Some(asset_id.clone()), None, None, None);

    let client = get_client();

    let mut lease = ResourceLease::new_println(client.assets.clone());
    let res = client
        .assets
        .upsert(
            &[new_asset.clone().into()],
            &UpsertOptions::default().ignore_nulls(true),
        )
        .await
        .unwrap();
    lease.add_resources(res.clone());
    assert_eq!(res[0].description.as_ref().unwrap(), "desc");

    new_asset.description = Some("desc 2".to_owned());

    let res = client
        .assets
        .upsert(
            &[new_asset.into()],
            &UpsertOptions::default().ignore_nulls(true),
        )
        .await
        .unwrap();
    lease.add_resources(res.clone());

    assert_eq!(res[0].description.as_ref().unwrap(), "desc 2");

    lease.clean().await.unwrap();
}
