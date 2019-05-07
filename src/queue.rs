use crate::frame::Frame;
use std::fmt::Debug;

pub trait Queue: Debug {
    fn enqueue(self, frame: Frame);
}
