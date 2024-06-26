#![cfg(feature = "integration_tests")]

#[cfg(test)]
mod common;
pub use common::*;

use cognite::time_series::*;
use cognite::*;

#[tokio::test]
async fn create_and_delete_time_series() {
    let id = format!("{}-ts1", PREFIX.as_str());
    let time_serie = TimeSeries {
        name: Some("name".to_string()),
        external_id: Some(id),
        is_string: false,
        is_step: true,
        description: Some("description".to_string()),
        ..Default::default()
    };
    let client = get_client();
    let mut time_series = client
        .time_series
        .create_from(&vec![time_serie])
        .await
        .unwrap();
    assert_eq!(time_series.len(), 1);
    for time_serie in time_series.iter_mut() {
        time_serie.description = Some(String::from("changed"));
    }

    let time_series = client.time_series.update_from(&time_series).await.unwrap();

    let id_list: Vec<Identity> = time_series
        .iter()
        .map(|ts| Identity::Id { id: ts.id })
        .collect();
    client.time_series.delete(&id_list, true).await.unwrap();
}
