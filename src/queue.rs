use crate::frame::Frame;
use std::fmt::Debug;

pub trait Queue: Debug {
    fn enqueue(self, frame: Frame);
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum QueueEnum {
    DummyQueue, KafkaQueue
}
