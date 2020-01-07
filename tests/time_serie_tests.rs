#[cfg(test)]
mod time_serie_tests {
    use cognite::*;

    #[tokio::test]
    async fn create_and_delete_time_series() {
        let cognite_client = CogniteClient::new("TestApp").unwrap();
        let time_serie = TimeSerie::new(
            "name",
            None,
            false,
            None,
            None,
            None,
            true,
            "description",
            None,
        );
        match cognite_client.time_series.create(&vec![time_serie]).await {
            Ok(mut time_series) => {
                assert_eq!(time_series.len(), 1);
                for time_serie in time_series.iter_mut() {
                    time_serie.description = String::from("changed");
                }

                time_series = match cognite_client.time_series.update(&time_series).await {
                    Ok(updated_time_series) => updated_time_series,
                    Err(e) => panic!("{:?}", e),
                };

                let id_list: Vec<u64> = time_series.iter().map(|ts| ts.id).collect();
                match cognite_client.time_series.delete(&id_list).await {
                    Ok(_) => assert!(true),
                    Err(e) => panic!("{:?}", e),
                };
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
