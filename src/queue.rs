use crate::context::Frame;
use crate::{context::Context, AResult};

pub trait Queue {
    fn en_queue(self, frame: Frame);
}

#[derive(Debug, Clone)]
pub enum QueueEnum {
    Kafka,
}
