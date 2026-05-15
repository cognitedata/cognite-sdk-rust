//! In-memory mock CogniteClient for use in tests.
//!
//! Mirrors the `FileClient` pattern from extractor-2: same field names as the
//! real `CogniteClient` but backed by in-memory stores instead of HTTP.
//!
//! # Usage
//! ```rust,ignore
//! use cognite::assets::AddAsset;
//! use cognite_mock::MockCogniteClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = MockCogniteClient::new();
//!     let _assets = client.assets.create(&[AddAsset {
//!         name: "Pump".to_string(),
//!         external_id: Some("pump-1".to_string()),
//!         ..Default::default()
//!     }]).await.unwrap();
//!     client.reset().await;
//! }
//! ```

mod patch;

pub mod client;
pub mod error;
pub mod resource;
pub mod store;

pub use error::MockError;
