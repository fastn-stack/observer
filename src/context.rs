use crate::kafka_queue::KafkaQueue;
use crate::queue::{
    Queue,
    QueueEnum::{self, Kafka},
};
use crate::{AResult, OError};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_json::json;
use std::{
    cell::RefCell,
    fs::{create_dir, File},
    io::Write,
    path::Path,
};
use uuid::Uuid;

pub static LOCAL_FILE_SYSTEM_DIRECTORY: &str = "/Users/venkatesh/observer_files/";

#[derive(Debug, Clone)]
pub struct Context {
    key: String,
    context_id: String,
    queue: QueueEnum,
    frame: RefCell<Frame>,
}

#[derive(Debug, Clone)]
pub struct Frame {
    key: String,
    frame_id: String,
    breadcrumbs: Option<serde_json::Value>,
    start_ts: DateTime<Utc>,
    end_ts: Option<DateTime<Utc>>,
    sub_frames: Vec<Frame>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SFrame {
    f_key: String,
    frame_id: String,
    breadcrumbs: Option<serde_json::Value>,
    start_ts: DateTime<Utc>,
    end_ts: Option<DateTime<Utc>>,
}

impl Context {
    pub fn new(queue: QueueEnum) -> Context {
        let frame = Frame::new("main".to_string());

        Context {
            context_id: "test_context".to_string(),
            frame: RefCell::new(frame),
            key: Uuid::new_v4().to_string(),
            queue: QueueEnum::Kafka,
        }
    }

    pub fn finalise(&self) {
        self.frame.borrow_mut().end_ts = Some(Utc::now());
        let destination_folder = LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + "context/";
        if Path::new(destination_folder.as_str()).exists() {
            let mut file = File::create(destination_folder + "/" + self.key.as_str()).unwrap();
            file.write(format!("{:?}", self.clone()).as_bytes());
        } else {
            create_dir(destination_folder.clone());
            let mut file = File::create(destination_folder + "/" + self.key.as_str()).unwrap();
            file.write(format!("{:?}", self.clone()).as_bytes());
        }
    }

    pub fn en_queue(&self, frame: Frame) {
        match self.queue {
            Kafka => KafkaQueue::new().en_queue(frame),
        }
    }

    pub fn save_on_local(&self, destination: String, frame: Frame) {
        let mut result;

        let destination_folder =
                    LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + destination.as_str();
        if Path::new(destination_folder.as_str()).exists() {
            let mut file = File::create(destination_folder + "/" + frame.get_key().as_str()).unwrap();
            let data = frame.get_data();
            result = file.write(data.to_string().as_bytes());
        } else {
            create_dir(destination_folder.clone());
            let mut file = File::create(destination_folder + "/" + frame.get_key().as_str()).unwrap();
            let data = frame.get_data();
            result = file.write(data.to_string().as_bytes());
        }
        if let Err(e) = result {
            println!("Error while saving on the local file system: {:?}",e);
        }
    }

    pub fn modify_context(&self, new_frame: Frame) {
        self.frame.replace(new_frame);
    }

    pub fn modify_add(&self, new_frame: Frame) {
        self.frame.borrow_mut().sub_frames.push(new_frame)
    }

    pub fn get_key(&self) -> String {
        self.clone().key
    }

    pub fn get_frame(&self) -> RefCell<Frame> {
        self.clone().frame
    }

    pub fn get_queue(&self) -> QueueEnum {
        self.clone().queue
    }

    pub fn update_end_ts(&self, end_ts: DateTime<Utc>) {
        self.frame.borrow_mut().end_ts = Some(end_ts);
    }

    pub fn update_breadcrumbs(&self, value: serde_json::value::Value) {
        self.frame.borrow_mut().breadcrumbs = Some(value);
    }
}

impl Frame {
    pub fn new(frame_id: String) -> Frame {
        Frame {
            key: Uuid::new_v4().to_string(),
            frame_id,
            breadcrumbs: None,
            start_ts: Utc::now(),
            end_ts: None,
            sub_frames: vec![],
        }
    }

    pub fn get_data(&self) -> serde_json::value::Value {
        json!({
        "f_key" : self.key,
        "frame_id" : self.frame_id,
        "breadcrumbs" : self.breadcrumbs,
        "start_ts" : self.start_ts,
        "end_ts" : self.end_ts
        })
    }

    pub fn get_key(&self) -> String {
        self.clone().key
    }
}
