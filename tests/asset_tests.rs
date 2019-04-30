#[cfg(test)]
mod assets {
  use cognite::*;

  #[test]
  fn create_update_and_delete_asset() {
    use uuid::Uuid;

    let cognite_client = CogniteClient::new().unwrap();
    let new_asset_name = Uuid::new_v4().to_hyphenated().to_string();
    let new_asset : Asset = Asset::new(&new_asset_name, "description", None, None, None, None);
    match cognite_client.assets.create(&vec!(new_asset)) {
      Ok(mut assets) => {
        assert_eq!(assets.len(), 1);
        let mut asset = assets.pop().unwrap();
        assert_eq!(asset.name, new_asset_name);

        asset.description = String::from("new description");
        asset = match cognite_client.assets.update_single(&asset) {
          Ok(updated_asset) => {
            updated_asset
          },
          Err(e) => panic!("{:?}", e)
        };
        assert_eq!(asset.description, String::from("new description"));

        assets.push(asset);
        let asset_ids : Vec<u64> = assets.iter().map(| a | a.id).collect();
        match cognite_client.assets.delete(&asset_ids) {
          Ok(_) => assert!(true),
          Err(e) => panic!("{:?}", e)
        }
      },
      Err(e) => panic!("{:?}", e)
    }
  }

  #[test]
  fn create_update_multiple_and_delete_asset() {
    use uuid::Uuid;

    let cognite_client = CogniteClient::new().unwrap();
    let new_asset_name = Uuid::new_v4().to_hyphenated().to_string();
    let new_asset : Asset = Asset::new(&new_asset_name, "description 1", None, None, None, None);

    let new_asset_2_name = Uuid::new_v4().to_hyphenated().to_string();
    let new_asset_2 : Asset = Asset::new(&new_asset_2_name, "description 2", None, None, None, None);

    match cognite_client.assets.create(&vec!(new_asset, new_asset_2)) {
      Ok(mut assets) => {
        assert_eq!(assets.len(), 2);
        for asset in assets.iter_mut() {
          asset.description = String::from("changed");
        }

        assets = match cognite_client.assets.update(&assets) {
          Ok(updated_assets) => {
            updated_assets
          },
          Err(e) => panic!("{:?}", e)
        };
        for asset in assets.iter() {
          assert_eq!(asset.description, String::from("changed"));
        }

        let asset_ids : Vec<u64> = assets.iter().map(| a | a.id).collect();
        match cognite_client.assets.delete(&asset_ids) {
          Ok(_) => assert!(true),
          Err(e) => panic!("{:?}", e)
        }
      },
      Err(e) => panic!("{:?}", e)
    }
  }
}