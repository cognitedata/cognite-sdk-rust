#[cfg(test)]
mod assets {
  use cognite::*;

  #[test]
  fn create_and_delete_asset() {
    use uuid::Uuid;

    let cognite_client = CogniteClient::new();
    let new_asset_name = Uuid::new_v4().to_hyphenated().to_string();
    let new_asset : Asset = Asset::new(&new_asset_name, "description", None, None);
    match cognite_client.assets.create(vec!(new_asset)) {
      Ok(assets) => {
        assert_eq!(assets.len(), 1);
        let asset = assets.get(0).unwrap();
        assert_eq!(asset.name, new_asset_name);
        let asset_ids : Vec<u64> = assets.into_iter().map(| a | a.id).collect();
        match cognite_client.assets.delete(asset_ids) {
          Ok(_) => assert!(true),
          Err(e) => panic!("{:?}", e)
        }
      },
      Err(e) => panic!("{:?}", e)
    }
  }
}