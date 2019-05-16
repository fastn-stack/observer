pub trait Queue {
    fn enqueue(&mut self, data: serde_json::Value);
}
