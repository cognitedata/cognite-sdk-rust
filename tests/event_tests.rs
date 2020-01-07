#[cfg(test)]
mod event_tests {
    use cognite::*;

    #[tokio::test]
    async fn create_update_and_delete_events() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();

        let cognite_client = CogniteClient::new("TestApp").unwrap();
        let new_event: Event = Event::new(
            Some(since_the_epoch.as_millis() as i64),
            Some((since_the_epoch.as_millis() + 1) as i64),
            None,
            None,
            None,
            Some("description".to_owned()),
            None,
            None,
            Some("source".to_owned()),
        );
        match cognite_client.events.create(&vec![new_event]).await {
            Ok(mut events) => {
                for event in events.iter_mut() {
                    event.description = Some(String::from("changed"));
                }

                match cognite_client.events.update(&events).await {
                    Ok(_) => (),
                    Err(e) => panic!("{:?}", e),
                };

                let event_ids: Vec<u64> = events.iter().map(|e| e.id).collect();
                match cognite_client.events.delete(&event_ids).await {
                    Ok(_) => assert!(true),
                    Err(e) => panic!("{:?}", e),
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
