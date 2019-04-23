extern crate cognite;

use cognite::{
  CogniteClient,
  Asset,
  Params,
};

fn main() {
  let cognite_client = CogniteClient::new();

  // List all assets
  let assets : Vec<Asset> = cognite_client.assets.list_all(None);
  println!("{} assets retrieved.", assets.len());
  
  // Retrieve asset
  let asset : Asset = cognite_client.assets.retrieve(6687602007296940);
  println!("{:?}", asset);

  // Search asset
  let params = Some(vec!(Params::AssetSearchName("Aker".to_owned()), Params::AssetSearchDescription("Aker".to_owned())));
  let asset_search : Vec<Asset> = cognite_client.assets.search(params);
  println!("{:?}", asset_search);

  // Retrieve multiple assets
  let assets_multiple : Vec<Asset> = cognite_client.assets.retrieve_multiple(vec!(6687602007296940));
  println!("{:?}", assets_multiple);
}