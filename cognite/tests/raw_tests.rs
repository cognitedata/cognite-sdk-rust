#![cfg(feature = "integration_tests")]

#[cfg(test)]
mod common;

use cognite::raw::*;
pub use common::*;

use futures::{StreamExt, TryStreamExt};
use serde_json::json;

#[tokio::test]
async fn create_delete_raw_rows_and_tables() {
    let client = get_client();
    let db_name = format!("{}-test-db-1", PREFIX.as_str());
    let db = client
        .raw
        .create_databases(&[Database {
            name: db_name.clone(),
        }])
        .await
        .unwrap();
    assert_eq!(1, db.len());
    let db = db.into_iter().next().unwrap();
    assert_eq!(db_name, db.name);

    let table = client
        .raw
        .create_tables(
            &db_name,
            false,
            &[Table {
                name: "test-table-1".to_owned(),
            }],
        )
        .await
        .unwrap();

    assert_eq!(1, table.len());
    let table = table.into_iter().next().unwrap();
    assert_eq!(table.name, "test-table-1");

    client
        .raw
        .insert_rows(
            &db_name,
            "test-table-1",
            false,
            &[RawRowCreate {
                key: "key-1".to_owned(),
                columns: json!({
                    "hello": "123",
                    "world": "321"
                }),
            }],
        )
        .await
        .unwrap();

    client
        .raw
        .delete_tables(
            &db_name,
            &[Table {
                name: "test-table-1".to_owned(),
            }],
        )
        .await
        .unwrap();

    client
        .raw
        .delete_databases(&DeleteDatabasesRequest {
            items: vec![Database {
                name: db_name.clone(),
            }],
            recursive: false,
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn retrieve_raw_rows_stream() {
    let client = get_client();
    let db_name = format!("{}-test-db-2", PREFIX.as_str());
    client
        .raw
        .insert_rows(
            &db_name,
            "test-table-1",
            true,
            &(0..1000)
                .map(|r| RawRowCreate {
                    key: format!("key-{r}"),
                    columns: json!({
                        "value": r,
                    }),
                })
                .collect::<Vec<_>>(),
        )
        .await
        .unwrap();

    let streamed: Vec<_> = client
        .raw
        .retrieve_all_rows_stream(
            &db_name,
            "test-table-1",
            Some(RetrieveRowsQuery {
                limit: Some(200),
                ..Default::default()
            }),
        )
        .take(300)
        .try_collect()
        .await
        .unwrap();

    assert_eq!(300, streamed.len());

    let streamed_partitioned: Vec<_> = client
        .raw
        .retrieve_all_rows_partitioned_stream(
            &db_name,
            "test-table-1",
            RetrieveAllPartitionedQuery {
                limit: Some(200),
                number_of_cursors: Some(3),
                ..Default::default()
            },
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(1000, streamed_partitioned.len());

    client
        .raw
        .delete_databases(&DeleteDatabasesRequest {
            items: vec![Database {
                name: db_name.clone(),
            }],
            recursive: true,
        })
        .await
        .unwrap();
}
