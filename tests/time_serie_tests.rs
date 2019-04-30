#[cfg(test)]
mod users {
  use cognite::*;

  #[test]
  fn create_and_delete_time_series() {
    let cognite_client = CogniteClient::new().unwrap();
    let time_serie = TimeSerie::new("name",
                                    false,
                                    None,
                                    None,
                                    None,
                                    true,
                                    "description",
                                    None);
    let mut time_series = vec!(time_serie); 
    match cognite_client.time_series.create(&time_series) {
      Ok(_) => {
        assert_eq!(time_series.len(), 1);
        for time_serie in time_series.iter_mut() {
          time_serie.description = String::from("changed");
        }

        /*
        time_series = match cognite_client.time_series.update(&time_series) {
          Ok(updated_time_series) => {
            updated_time_series
          },
          Err(e) => panic!("{:?}", e)
        };
        */

        for time_serie in time_series.iter() {

          match cognite_client.time_series.delete(&time_serie.name) {
            Ok(_) => assert!(true),
            Err(e) => panic!("{:?}", e)
          };
        }
      },
      Err(e) => panic!("{:?}", e)
    }
  }
}