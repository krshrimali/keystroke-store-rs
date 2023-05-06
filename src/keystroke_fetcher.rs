use rdev::{Event, EventType};

use crate::kafka_producer;

fn handle_keyboard_events(event_type: EventType) {
    // it's safe to assume that the event will always be either KeyPress or KeyRelease event, this is handled one level up (callback fn)
    let key_context: Option<rdev::Key> = match event_type {
        EventType::KeyPress(key) => Some(key),
        EventType::KeyRelease(_) => None,
        _ => None,
    };
    if let Some(key_) = key_context {
        println!("Key context: {:?}", key_);
        kafka_producer::send_to_kafka(key_);
    }
}

pub fn callback(event: Event) {
    match event.event_type {
        rdev::EventType::KeyPress(_) => {
            handle_keyboard_events(event.event_type);
        }
        rdev::EventType::KeyRelease(_) => {
            handle_keyboard_events(event.event_type);
        }
        _ => (),
    }
}