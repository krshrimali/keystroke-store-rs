use rdev::{listen, Event, EventType};

fn handle_keyboard_events(event_type: EventType) {
    // it's safe to assume that the event will always be either KeyPress or KeyRelease event, this is handled one level up (callback fn)
    let key_context: Option<rdev::Key> = match event_type {
        EventType::KeyPress(key) => Some(key),
        EventType::KeyRelease(key) => Some(key),
        _ => None,
    };
    println!("key context: {:?}", key_context.unwrap());
}

fn callback(event: Event) {
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

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
