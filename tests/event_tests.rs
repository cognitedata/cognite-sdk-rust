mod common;
use common::*;

use cognite::events::*;
use cognite::*;

#[tokio::test]
async fn create_update_and_delete_events() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let id = format!("{}-event1", PREFIX.as_str());

    let new_event: Event = Event::new(
        None,
        Some(since_the_epoch.as_millis() as i64),
        Some((since_the_epoch.as_millis() + 1) as i64),
        None,
        None,
        Some(id),
        Some("description".to_owned()),
        None,
        None,
        Some("source".to_owned()),
    );
    let mut events = COGNITE_CLIENT
        .events
        .create_from(&vec![new_event])
        .await
        .unwrap();
    for event in events.iter_mut() {
        event.description = Some(String::from("changed"));
    }

    COGNITE_CLIENT.events.update_from(&events).await.unwrap();

    let event_ids: Vec<Identity> = events.iter().map(|e| Identity::Id { id: e.id }).collect();
    COGNITE_CLIENT
        .events
        .delete(&event_ids, false)
        .await
        .unwrap();
}
