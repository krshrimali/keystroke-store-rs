extern crate env_logger;
extern crate log;

use rdev::listen;
use rdev::Event;
use rdev::EventType;
use rdev::Key;

use log::{info, warn};

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
pub struct EventHandler {}

impl EventHandler {
    fn send_key(key: Key) {
        kafka_producer::send_to_kafka(key);
    }

    fn _handle_keyboard_press_event(key: Option<Key>) {
        info!("Key pressed: {:?}", key);
        // TODO:
        // Consider if a key is just pressed and not released immediate after, which could mean possible repetitions of keys for some Operating Systems like Linux (In OSX, I believe you need to change some settings)
        if let Some(key_pressed) = key {
            EventHandler::send_key(key_pressed);
            kafka_producer::send_to_kafka(key_pressed);
        }
    }

    fn _handle_keyboard_release_event(key: Option<Key>) {
        info!("Key released: {:?}", key);
        if let Some(key_released) = key {
            EventHandler::send_key(key_released);
            kafka_producer::send_to_kafka(key_released);
        }
    }

    fn handle_keyboard_events(event_type: EventType, key: Option<Key>) {
        // it's safe to assume that the event will always be either KeyPress or KeyRelease event, this is handled one level up (NextStep::start_listening fn)
        match event_type {
            EventType::KeyPress(key) => EventHandler::_handle_keyboard_press_event(Some(key)),
            EventType::KeyRelease(key) => EventHandler::_handle_keyboard_release_event(Some(key)),
            event => {
                warn!("Unexpected event {:?} found", event);
            }
        };
    }

    fn skip_mouse_events(event_type: EventType) {
        info!("Event Type skipped: {:?}", event_type);
    }

    fn skip_button_events(event_type: EventType) {
        info!("Event Type skipped: {:?}", event_type);
    }
}

impl NextStep {
    pub fn start_listening(event: Event) {
        match event.event_type {
            rdev::EventType::KeyPress(key_pressed) => {
                EventHandler::handle_keyboard_events(event.event_type, key_pressed.into());
            }
            rdev::EventType::KeyRelease(key_released) => {
                EventHandler::handle_keyboard_events(event.event_type, key_released.into());
            }
            rdev::EventType::Wheel {
                delta_x: _,
                delta_y: _,
            } => {
                EventHandler::skip_mouse_events(event.event_type);
            }
            rdev::EventType::MouseMove { x: _, y: _ } => {
                EventHandler::skip_mouse_events(event.event_type);
            }
            rdev::EventType::ButtonPress(button_pressed) => {
                EventHandler::skip_button_events(event.event_type);
            }
            rdev::EventType::ButtonRelease(button_released) => {
                EventHandler::skip_button_events(event.event_type);
            }
        }
    }

    pub fn start() {
        env_logger::init();

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
