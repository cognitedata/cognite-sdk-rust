use cognite::events::*;
use cognite::*;

mod common;
pub use common::*;

#[tokio::test]
async fn create_update_and_delete_events() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let id = format!("{}-event1", PREFIX.as_str());

    let client = get_client();

    let new_event = Event {
        start_time: Some(since_the_epoch.as_millis() as i64),
        end_time: Some((since_the_epoch.as_millis() + 1) as i64),
        external_id: Some(id),
        description: Some("description".to_string()),
        source: Some("source".to_string()),
        ..Default::default()
    };
    let mut events = client.events.create_from(&vec![new_event]).await.unwrap();

    for event in events.iter_mut() {
        event.description = Some(String::from("changed"));
    }

    client.events.update_from(&events).await.unwrap();

    let event_ids: Vec<Identity> = events.iter().map(|e| Identity::Id { id: e.id }).collect();
    client.events.delete(&event_ids, false).await.unwrap();
}
