use crate::context::LOCAL_FILE_SYSTEM_DIRECTORY;
use crate::queue::QueueEnum;
use crate::queue::QueueEnum::DummyQueue;
use chrono::prelude::*;
use core::borrow::BorrowMut;
use observer_dqueue::dummy_queue::DummyQueue as DQueue;
use observer_queue::queue::Queue;
use std::collections::HashMap;
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Frame {
    key: String,
    frame_id: String,
    breadcrumbs: Option<HashMap<String, serde_json::Value>>,
    start_ts: DateTime<Utc>,
    pub success: Option<bool>,
    pub result: Option<String>,
    pub end_ts: Option<DateTime<Utc>>,
    pub sub_frames: Vec<Frame>,
}

impl Frame {
    pub fn new(frame_id: String) -> Frame {
        Frame {
            key: uuid::Uuid::new_v4().to_string(),
            frame_id,
            breadcrumbs: None,
            result: None,
            success: None,
            start_ts: chrono::Utc::now(),
            end_ts: None,
            sub_frames: vec![],
        }
    }

    pub fn get_data(&self) -> serde_json::value::Value {
        json!({
            "f_key" : self.key,
            "frame_id" : self.frame_id,
            "breadcrumbs" : self.breadcrumbs,
            "result" : self.result,
            "success" : self.success,
            "start_ts" : self.start_ts,
            "end_ts" : self.end_ts
        })
    }

    pub fn get_key(&self) -> String {
        self.clone().key
    }

    pub fn save(&self, critical: bool, queue: QueueEnum) {
        if critical {
            self.enqueue(queue)
        } else {
            self.save_on_local()
        }
    }

    pub fn save_on_local(&self) {
        let mut result;
        let destination_folder = LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + self.frame_id.as_str();

        if !Path::new(destination_folder.as_str()).exists() {
            create_dir(destination_folder.clone()).unwrap(); // TODO
        }
        let mut file = File::create(destination_folder + "/" + self.get_key().as_str()).unwrap();
        let data = self.get_data();
        result = file.write(data.to_string().as_bytes());

        if let Err(e) = result {
            println!("Error while saving on the local file system: {:?}", e);
        }
    }

    pub fn enqueue(&self, queue: QueueEnum) {
        match queue {
            QueueEnum::DummyQueue => {
                let dq = &mut DQueue::new();
                dq.enqueue(self.get_data());
            }
            QueueEnum::KafkaQueue => unimplemented!(),
        }
    }

    //adds the name and value to breadcrums
    pub fn add_value(&mut self, name: &str, value: serde_json::Value) {
        let mut current_breadcrumbs = self.clone().breadcrumbs.unwrap_or(HashMap::new());
        current_breadcrumbs.insert(name.to_string(), value);
        self.breadcrumbs = Some(current_breadcrumbs);
    }
}
