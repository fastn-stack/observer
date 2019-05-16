use observer_queue::queue::Queue;

pub struct DummyQueue {
    data_queue: Vec<serde_json::Value>,
}

impl Queue for DummyQueue {
    fn enqueue(&mut self, data: serde_json::Value) {
        self.data_queue.push(data);
    }
}

impl DummyQueue {
    pub fn new() -> DummyQueue {
        DummyQueue { data_queue: vec![] }
    }
}
