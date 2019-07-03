use crate::context::LOCAL_FILE_SYSTEM_DIRECTORY;
use crate::queue::Queue;
use chrono::prelude::*;
use core::borrow::BorrowMut;
use std::collections::HashMap;
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Frame {
    id: String,
    key: String,
    pub breadcrumbs: Option<HashMap<String, serde_json::Value>>,
    pub success: Option<bool>,
    pub result: Option<serde_json::Value>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub sub_frames: Vec<Frame>,
}

impl Frame {
    pub fn new(id: String) -> Frame {
        Frame {
            id,
            key: uuid::Uuid::new_v4().to_string(),
            breadcrumbs: None,
            success: None,
            result: None,
            start_time: Utc::now(),
            end_time: None,
            sub_frames: vec![],
        }
    }

    pub fn start(&mut self) -> &mut Self {
        self.start_time = Utc::now();
        self
    }

    pub fn end(&mut self) -> &mut Self {
        self.end_time = Some(Utc::now());
        self
    }

    pub fn set_result(&mut self, result: serde_json::Value) -> &mut Self {
        self.result = Some(result);
        self
    }

    pub fn set_success(&mut self, is_success: bool) -> &mut Self {
        self.success = Some(is_success);
        self
    }

    pub fn add_sub_frame(&mut self, frame: Frame) {
        self.sub_frames.push(frame);
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn save(&self, critical: bool, queue: &Box<dyn Queue>) {
        if critical {
            self.enqueue(queue)
        } else {
            self.save_on_local()
        }
    }

    pub fn save_on_local(&self) {
        let destination_folder = LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + self.id.as_str();

        if !Path::new(destination_folder.as_str()).exists() {
            create_dir(destination_folder.clone()).unwrap(); // TODO
        }
        let mut file = File::create(destination_folder + "/" + self.id.as_str()).unwrap();
        let data = json!(self);
        let result = file.write(data.to_string().as_bytes());

        if let Err(e) = result {
            println!("Error while saving on the local file system: {:?}", e);
        }
    }

    pub fn enqueue(&self, _queue: &Box<dyn Queue>) {}

    //adds the name and value to breadcrums
    pub fn add_value(&mut self, name: &str, value: serde_json::Value) {
        let mut current_breadcrumbs = self.clone().breadcrumbs.unwrap_or(HashMap::new());
        current_breadcrumbs.insert(name.to_string(), value);
        self.breadcrumbs = Some(current_breadcrumbs);
    }
}
