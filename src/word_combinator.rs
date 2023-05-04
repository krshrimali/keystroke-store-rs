use rdev::listen;
use rdev::Key;

use crate::keystroke_fetcher;

#[allow(unused)]
pub enum KeyEvent {
    ValidKey(),
    StopperKey(),
    MouseClick(),
    MouseDrag(),
    Other(),
}

pub struct NextStep {}

impl NextStep {
    pub fn start() {
        /* Start fetching the first event */
        if let Err(error) = listen(keystroke_fetcher::callback) {
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
