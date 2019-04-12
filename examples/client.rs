extern crate cognite;

use cognite::{
  CogniteClient,
  Asset,
};

fn main() {
  let cognite_client = CogniteClient::new();
  let assets : Vec<Asset> = cognite_client.assets.list_all();
  println!("{} assets retrieved.", assets.len());
  let asset : Asset = cognite_client.assets.retrieve(6687602007296940);
  println!("{:?}", asset);
}