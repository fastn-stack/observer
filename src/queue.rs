use crate::context::Frame;
use serde_derive::{Deserialize, Serialize};

pub trait Queue {
    fn en_queue(self, frame: Frame);
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum QueueEnum {
    Kafka,
}
