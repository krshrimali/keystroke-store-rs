use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};
use rdev::Key;
use std::time::Duration;

pub fn send_to_kafka(output_data: &String) {
    // env_logger::init();
    let broker = "localhost:9092";
    let topic = "test";

    let output_msg: String = serde_json::from_str(output_data).unwrap();
    // let key_data = serde_json::to_string(&key_msg).unwrap();
    // TODO: (@krshrimali)
    // 1. This should not be here, consider moving to the place where events are fetched.
    // 2. Shift to chrono::offset::Utc::now() from Local::now(), local timezone helps me test better for now.
    // let msg: String = output_msg + " " + &chrono::offset::Local::now().to_string();

    if let Err(e) = produce_message(output_msg.as_bytes(), topic, vec![broker.to_owned()]) {
        println!("Failed producing messages: {:?}", e);
    }
    println!("SENT");
}

fn produce_message(data: &[u8], topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    println!("Sending msg: {:?}", data);
    producer.send(&Record {
        topic,
        partition: -1,
        key: (),
        value: data,
    })?;

    Ok(())
}
