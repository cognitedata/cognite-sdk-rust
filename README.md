Cognite Rust SDK
==========================

Under development.. 

Rust SDK to ensure excellent user experience for developers and data scientists working with the Cognite Data Fusion.

## Documentation
* [API Documentation](https://doc.cognitedata.com/)
* [API Guide](https://doc.cognitedata.com/guides/api-guide.html)

## Prerequisites
 Install rust. See [instructions here](https://rustup.rs/).

Set environment variables:

```bash
$ export COGNITE_BASE_URL="https://api.cognitedata.com"
$ export COGNITE_API_KEY=<your API key>
```

## Example

Since this is not published on crates.io, then you'll have to clone the repo and reference it locally.

Cargo.toml:

```Rust
[dependencies]
cognite = { path = "../cognite-sdk-rust" }
```

```Rust
extern crate cognite;

use cognite::{
  CogniteClient,
  Asset,
};

fn main() {
  let cognite_client = CogniteClient::new().unwrap();

  // List all assets
  let assets : Vec<Asset> = cognite_client.assets.list_all(None).unwrap();
  
  // Retrieve asset
  let asset : Asset = cognite_client.assets.retrieve(<asset_id>).unwrap();

  // Search asset
  let params = Some(vec!(
    Params::AssetSearch_Name(<asset_name>), 
    Params::AssetSearch_Description(<asset_description>),
    ...
  ));
  let asset_search : Vec<Asset> = cognite_client.assets.search(params).unwrap();

  // Retrieve multiple assets
  let asset_ids = vec!(
    <asset_id>, 
    <asset_id>, 
    <asset_id>,
    ...
  );
  let assets_multiple : Vec<Asset> = cognite_client.assets.retrieve_multiple(asset_ids).unwrap();
}
```

## Run examples

```bash
cargo run --example client
```