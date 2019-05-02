use crate::context::Frame;
use crate::{context::Context, AResult};
use serde_derive::{Serialize, Deserialize};

pub trait Queue {
    fn en_queue(self, frame: Frame);
}

#[derive(Serialize, Deserialize ,Debug, Clone, PartialEq)]
pub enum QueueEnum {
    Kafka,
}
