#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum QueueEnum {
    DummyQueue,
    KafkaQueue,
}


//pub trait Queue {
//    fn enqueue(&mut self, data: serde_json::Value);
//}
