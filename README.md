Cognite Rust SDK
==========================

Rust SDK to ensure excellent user experience for developers and data scientists working with the Cognite Data Fusion.

## Documentation
* [API Documentation](https://docs.cognite.com/api/v1/)

## Prerequisites
 Install rust. See [instructions here](https://rustup.rs/).

Set environment variables:

```bash
$ export COGNITE_BASE_URL="https://api.cognitedata.com"
$ export COGNITE_CLIENT_ID=<your client id>
$ export COGNITE_CLIENT_SECRET=<your client secret>
$ export COGNITE_TOKEN_URL=<your token url>
$ export COGNITE_SCOPES=<space separated list of scopes>
$ export COGNITE_PROJECT=<your project name>
```

## Supported features for API v1

### Core
- Assets
- Events
- Files
  - Without upload/download
- TimeSeries
  - With protobuf support
- Sequences
### IAM
- ApiKeys
- Groups
- SecurityCategories
- ServiceAccounts
### Data Ingestion
- Extraction pipelines
- Raw
### Data Organization
- Datasets
- Labels
- Relationships

## Example

Since this is not published on crates.io, you'll have to reference the git repository

Cargo.toml:

```TOML
[dependencies]
cognite = { git = "https://github.com/cognitedata/cognite-sdk-rust" }
tokio = { version = "0.2", features = ["full"] }
```

```Rust
use cognite::prelude::*;
use cognite::{Asset, AssetFilter, AssetSearch, CogniteClient};

#[tokio::main]
fn main() {
    let cognite_client = CogniteClient::new("TestApp").unwrap();

    // List all assets
    let mut filter: AssetFilter = AssetFilter::new();
    filter.name.replace("Aker".to_string());
    let assets = cognite_client
        .assets
        .filter(FilterAssetsRequest {
            filter,
            ..Default::default()
        })
        .await
        .unwrap();

    // Retrieve asset
    match cognite_client
        .assets
        .retrieve(&vec![Identity::from(6687602007296940)], false, None)
        .await
        .unwrap();
}
```

## Run examples

```bash
cargo run --example client
```
