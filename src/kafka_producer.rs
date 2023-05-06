use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};
use rdev::Key;
use std::time::Duration;

pub fn send_to_kafka(key_msg: Key) {
    // env_logger::init();
    let broker = "localhost:9092";
    let topic = "test";

    let key_data = serde_json::to_string(&key_msg).unwrap();
    let msg: String = key_data + " " + &chrono::offset::Local::now().to_string();

    if let Err(e) = produce_message(msg.as_bytes(), topic, vec![broker.to_owned()]) {
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
