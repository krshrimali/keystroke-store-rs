extern crate env_logger;
extern crate log;

use rdev::listen;
use rdev::Event;
use rdev::EventType;
use rdev::Key;
use serde::Deserialize;
use std::fmt;

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

#[derive(Default)]
struct DataArray {
    data_arr: Vec<Data>,
}

#[derive(Default, Deserialize)]
struct Data {
    id: i32,
    key: String,
    timestamp: String,
}

impl DataArray {
    fn get_from_id(&self, input_id: &i32) -> Data {
        let mut new_data = Data::default();
        for item in self.data_arr.iter() {
            if &item.id == input_id {
                new_data.id = item.id;
                new_data.key = item.key.clone();
                new_data.timestamp = item.timestamp.clone();
            }
        }
        new_data
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, Key: {}, TS: {}", self.id, self.key, self.timestamp)
    }
}

pub struct NextStep {
    counter: i32,
}

#[derive(Default)]
pub struct EventHandler {
    output_data: Data,
}

impl EventHandler {
    fn send_key_with_data(&self) {
        kafka_producer::send_to_kafka(&self.output_data.to_string());
    }

    fn _hydrate_data(&mut self, counter: i32, key_in_context: &Key) {
        let utc_curr_ts = chrono::Utc::now();
        let msg_json = format!(
            r#"{{
                "id": {counter},
                "key": "{key_in_context:?}",
                "timestamp": "{utc_curr_ts}"
            }}"#
        );
        self.output_data = serde_json::from_str(&msg_json).unwrap();
    }

    fn _handle_keyboard_press_event(&mut self, counter: i32, key: Option<Key>) {
        info!("Key pressed: {:?}", key);
        // TODO:
        // Consider if a key is just pressed and not released immediate after, which could mean possible repetitions of keys for some Operating Systems like Linux (In OSX, I believe you need to change some settings)
        if let Some(key_pressed) = key {
            self._hydrate_data(counter, &key_pressed);
            self.send_key_with_data();
        }
    }

    fn _handle_keyboard_release_event(&self, key: Option<Key>) {
        info!("Key released: {:?}", key);
        if key.is_some() {
            self.send_key_with_data();
        }
    }

    fn handle_keyboard_events(&mut self, counter: i32, event_type: EventType, _key: Option<Key>) {
        // it's safe to assume that the event will always be either KeyPress or KeyRelease event, this is handled one level up (NextStep::start_listening fn)
        match event_type {
            EventType::KeyPress(key) => self._handle_keyboard_press_event(counter, Some(key)),
            EventType::KeyRelease(key) => self._handle_keyboard_release_event(Some(key)),
            event => {
                warn!("Unexpected event {:?} found", event);
            }
        };
    }

    fn skip_mouse_events(&self, event_type: EventType) {
        info!("Event Type skipped: {:?}", event_type);
    }

    fn skip_button_events(&self, event_type: EventType) {
        info!("Event Type skipped: {:?}", event_type);
    }
}

impl NextStep {
    pub fn new() -> Self {
        Self::new()
    }

    pub fn start_listening(event: Event) {
        match event.event_type {
            rdev::EventType::KeyPress(key_pressed) => {
                self.counter += 1;
                EventHandler::default()
                    .handle_keyboard_events(self.counter, event.event_type, key_pressed.into());
            }
            rdev::EventType::KeyRelease(key_released) => {
                EventHandler::default()
                    .handle_keyboard_events(self.counter, event.event_type, key_released.into());
            }
            rdev::EventType::Wheel {
                delta_x: _,
                delta_y: _,
            } => {
                EventHandler::default().skip_mouse_events(event.event_type);
            }
            rdev::EventType::MouseMove { x: _, y: _ } => {
                EventHandler::default().skip_mouse_events(event.event_type);
            }
            rdev::EventType::ButtonPress(_button_pressed) => {
                EventHandler::default().skip_button_events(event.event_type);
            }
            rdev::EventType::ButtonRelease(_button_released) => {
                EventHandler::default().skip_button_events(event.event_type);
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
