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
  AssetFilter,
  AssetSearch,
};

fn main() {
  let cognite_client = CogniteClient::new().unwrap();

  // Filter all assets

  let mut filter : AssetFilter = AssetFilter::new();
  filter.name.replace("aker".to_string());
  filter.asset_subtrees.replace(vec!(...));
  ...
  let assets : Vec<Asset> = cognite_client.assets.filter_all(filter).unwrap();
  
  // Retrieve asset
  match cognite_client.assets.retrieve(&vec!(6687602007296940_u64)) {
    Ok(asset) => println!("{:?}", asset),
    Err(e) => println!("{:?}", e)
  }

  // Search asset
  let mut asset_search : AssetSearch = AssetSearch::new();
  asset_search.description.replace("aker".to_string());
  asset_search.name.replace("...".to_string());
  ...
  let mut asset_filter : AssetFilter = AssetFilter::new();
  asset_filter.source.replace("...".to_string());
  ...
  let assets_search_result : Vec<Asset> = cognite_client.assets.search(asset_filter, asset_search).unwrap();
}
```

## Run examples

```bash
cargo run --example client
```