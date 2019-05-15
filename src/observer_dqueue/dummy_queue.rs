use crate::queue::Queue;
use crate::frame::Frame;


pub struct DummyQueue {
    data_queue: Vec<serde_json::Value>
}

impl Queue for DummyQueue {
    fn enqueue(&mut self, frame: &Frame) {
        self.data_queue.push(frame.get_data())
    }
}

impl DummyQueue {
    pub fn new() -> DummyQueue {
        DummyQueue{
            data_queue : vec![]
        }
    }
}