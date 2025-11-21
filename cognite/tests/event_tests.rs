#![cfg(feature = "integration_tests")]

#[cfg(test)]
mod common;

use cognite::utils::lease::ResourceLease;
pub use common::*;

use cognite::events::*;
use cognite::*;

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
    let mut lease = ResourceLease::new_println(client.events.clone());
    let mut events = client.events.create_from(&vec![new_event]).await.unwrap();
    lease.add_resources(events.clone());

    for event in events.iter_mut() {
        event.description = Some(String::from("changed"));
    }

    client.events.update_from(&events).await.unwrap();

    lease.clean().await.unwrap();
}

#[tokio::test]
async fn upsert_events() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let id = format!("{}-event2", PREFIX.as_str());

    // Call the upsert method twice

    let client = get_client();
    let mut lease = ResourceLease::new_println(client.events.clone());

    let mut new_event = AddEvent {
        start_time: Some(since_the_epoch.as_millis() as i64),
        end_time: Some((since_the_epoch.as_millis() + 1) as i64),
        external_id: Some(id.clone()),
        ..Default::default()
    };

    let events = client
        .events
        .upsert(
            &[new_event.clone()],
            &UpsertOptions::default().ignore_nulls(true),
        )
        .await
        .unwrap();
    assert_eq!(1, events.len());
    lease.add_resources(events.clone());

    new_event.description = Some("description".to_owned());

    let events = client
        .events
        .upsert(
            &[new_event.clone()],
            &UpsertOptions::default().ignore_nulls(true),
        )
        .await
        .unwrap();
    lease.add_resources(events.clone());

    assert_eq!(1, events.len());

    let evt = events.into_iter().next().unwrap();
    assert_eq!(Some("description".to_owned()), evt.description);

    lease.clean().await.unwrap();
}
