mod common;
use common::*;

use cognite::time_series::*;
use cognite::*;

#[tokio::test]
async fn create_and_delete_time_series() {
    let id = format!("{}-ts1", PREFIX.as_str());
    let time_serie = TimeSerie::new(
        "name",
        Some(id),
        false,
        None,
        None,
        None,
        true,
        "description",
        None,
    );
    let mut time_series = COGNITE_CLIENT
        .time_series
        .create_from(&vec![time_serie])
        .await
        .unwrap();
    assert_eq!(time_series.len(), 1);
    for time_serie in time_series.iter_mut() {
        time_serie.description = Some(String::from("changed"));
    }

    let time_series = COGNITE_CLIENT
        .time_series
        .update_from(&time_series)
        .await
        .unwrap();

    let id_list: Vec<Identity> = time_series
        .iter()
        .map(|ts| Identity::Id { id: ts.id })
        .collect();
    COGNITE_CLIENT
        .time_series
        .delete(&id_list, true)
        .await
        .unwrap();
}
