use rdev::listen;
use rdev::Event;
use rdev::EventType;
use rdev::Key;

use crate::kafka_producer;

#[allow(unused)]
pub enum KeyEvent {
    ValidKey(),
    StopperKey(),
    MouseClick(),
    MouseDrag(),
    Other(),
}

pub struct NextStep {}

fn handle_keyboard_events(event_type: EventType) {
    // it's safe to assume that the event will always be either KeyPress or KeyRelease event, this is handled one level up (callback fn)
    let key_context: Option<rdev::Key> = match event_type {
        EventType::KeyPress(key) => Some(key),
        EventType::KeyRelease(_) => None,
        _ => None,
    };
    if let Some(key_) = key_context {
        // Need to write a logger and logger DB
        println!("Key context: {:?}", key_);
        kafka_producer::send_to_kafka(key_);
    }
}

fn skip_mouse_events(event_type: EventType) {
    println!("Event Type skipped: {:?}", event_type);
}

fn skip_button_events(event_type: EventType) {
    println!("Event Type skipped: {:?}", event_type);
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

impl NextStep {
    pub fn start_listening(event: Event) {
        match event.event_type {
            rdev::EventType::KeyPress(key_pressed) => {
                handle_keyboard_events(event.event_type);
            }
            rdev::EventType::KeyRelease(key_released) => {
                handle_keyboard_events(event.event_type);
            }
            rdev::EventType::Wheel {
                delta_x: _,
                delta_y: _,
            } => {
                skip_mouse_events(event.event_type);
            }
            rdev::EventType::MouseMove { x: _, y: _ } => {
                skip_mouse_events(event.event_type);
            }
            rdev::EventType::ButtonPress(button_pressed) => {
                skip_button_events(event.event_type);
            }
            rdev::EventType::ButtonRelease(button_released) => {
                skip_button_events(event.event_type);
            }
        }
    }

    pub fn start() {
        /* Start fetching the first event */
        if let Err(error) = listen(NextStep::start_listening) {
            println!("Error: {:?}", error);
        }
    }

    #[allow(unused)]
    fn continue_fetch(key: &Key) {
        /* Continue fetching keystroke events */
    }

    #[allow(unused)]
    fn break_fetch(key: &Key, id_event: &i32) {
        /* Break word split */
    }

    #[allow(unused)]
    fn fail(id_event: &i32) {
        /* Fail here and log along with the id_event */
    }
}

/* Algorithm is recorded here: */
#[allow(unused)]
pub fn keystroke_combinator(event: KeyEvent, key_pressed: Key, id_event: i32) {
    match event {
        KeyEvent::ValidKey() => NextStep::continue_fetch(&key_pressed),
        KeyEvent::StopperKey() | KeyEvent::MouseClick() | KeyEvent::MouseDrag() => {
            NextStep::break_fetch(&key_pressed, &id_event)
        }
        _ => NextStep::fail(&id_event),
    };
}
