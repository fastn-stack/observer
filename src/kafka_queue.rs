use crate::context::Frame;
use crate::queue::Queue;
use kafka::{
    error::Error as KafkaError,
    producer::{Producer, Record, RequiredAcks},
};
use std::time::Duration;

pub static KAFKA_TOPIC: &str = "my-topic";

pub static KAFKA_BROKER: &str = "localhost:9092";

pub struct KafkaQueue {}

impl Queue for KafkaQueue {
    fn en_queue(self, frame: Frame) {
        let data = frame.get_data();
        let result = produce_message(
            data.to_string().as_bytes(),
            KAFKA_TOPIC,
            vec![KAFKA_BROKER.to_string()],
        );
        if let Err(e) = result {
            println!("Error while sending message to kafka queue: {:?}", e);
        }
    }
}

impl KafkaQueue {
    pub fn new() -> KafkaQueue {
        KafkaQueue {}
    }
}

pub fn produce_message(data: &[u8], topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    producer.send(&Record::from_value(topic, data))
}
