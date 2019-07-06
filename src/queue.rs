use std::fmt::Debug;

#[typetag::serde(tag = "type", content = "value")]
pub trait Queue: Debug {
    fn enqueue(&self, data: serde_json::Value);
}

#[derive(Serialize, Debug, Deserialize)]
pub struct DemoQueue {
    pub name: String,
}

#[typetag::serde(name = "Abc")]
impl Queue for DemoQueue {
    fn enqueue(&self, data: serde_json::Value) {
        println!("Data Here: {}", data)
    }
}
