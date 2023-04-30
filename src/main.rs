use rdev::{listen, Event};

fn callback(event: Event) {
    // println!("My callback {:?}", event);
    match event.event_type {
        rdev::EventType::KeyPress(_) => {
            println!("Found keypress event with key: {:?}", event.name.unwrap())
        }
        rdev::EventType::KeyRelease(_) => {
            println!(
                "Found KeyRelease event with key: {:?}",
                event.name.unwrap_or(String::from(""))
            )
        }
        _ => (),
    }
}

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
