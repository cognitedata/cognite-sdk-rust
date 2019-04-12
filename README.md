Cognite Rust SDK
==========================

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
  let cognite_client = CogniteClient::new();
  let assets : Vec<Asset> = cognite_client.assets.list_all();
  let asset : Asset = cognite_client.assets.retrieve(<asset_id>);
}
```