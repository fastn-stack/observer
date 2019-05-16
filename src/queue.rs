#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum QueueEnum {
    DummyQueue,
    KafkaQueue,
}
