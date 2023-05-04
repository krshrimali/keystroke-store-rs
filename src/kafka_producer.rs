use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};
use rdev::Key;
use std::time::Duration;

pub fn send_to_kafka(msg: Key) {
    // env_logger::init();
    let broker = "localhost:9092";
    let topic = "test";

    let data = serde_json::to_string(&msg).unwrap();

    if let Err(e) = produce_message(data.as_bytes(), topic, vec![broker.to_owned()]) {
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
