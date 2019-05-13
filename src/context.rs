use crate::frame::Frame;
use crate::queue::Queue;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

pub static LOCAL_FILE_SYSTEM_DIRECTORY: &str = "/Users/venkatesh/observer_files/";

#[derive(Debug)]
pub struct Context {
    key: String,
    context_id: String,
    queue: Box<Queue>,
    frame: RefCell<Frame>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SFrame {
    f_key: String,
    frame_id: String,
    breadcrumbs: Option<serde_json::Value>,
    start_ts: DateTime<Utc>,
    end_ts: Option<DateTime<Utc>>,
}

impl Context {
    pub fn new(context_id: String, queue: Box<Queue>) -> Context {
        let frame = Frame::new("main".to_string());

        Context {
            context_id,
            frame: RefCell::new(frame),
            key: uuid::Uuid::new_v4().to_string(),
            queue,
        }
    }

    pub fn finalise(&self) {
        unimplemented!()
        /*
        self.update_end_ts(Utc::now());
        let destination_folder = LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + "context/";
        if !Path::new(destination_folder.as_str()).exists() {
            create_dir(destination_folder.clone()).unwrap(); // TODO
        }
        let mut file = File::create(destination_folder + "/" + self.key.as_str()).unwrap();
        file.write(self.get_data().as_bytes()).unwrap(); // TODO
        */
    }

    pub fn enqueue(&self, frame: Frame) {
        unimplemented!()
    }

    pub fn save_on_local(&self, destination: String, frame: Frame) {
        unimplemented!()

        /*
        let mut result;

        let destination_folder = LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + destination.as_str();

        if !Path::new(destination_folder.as_str()).exists() {
            create_dir(destination_folder.clone()).unwrap(); // TODO
        }
        let mut file = File::create(destination_folder + "/" + frame.get_key().as_str()).unwrap();
        let data = frame.get_data();
        result = file.write(data.to_string().as_bytes());

        if let Err(e) = result {
            println!("Error while saving on the local file system: {:?}", e);
        }
        */
    }

    pub fn modify_context(&self, new_frame: Frame) {
        self.frame.replace(new_frame);
    }

    pub fn modify_add(&self, new_frame: Frame) {
        // self.frame.borrow_mut().sub_frames.push(new_frame)
        unimplemented!()
    }

    pub fn get_key(&self) -> String {
        // self.clone().key
        unimplemented!()
    }

    pub fn get_frame(&self) -> RefCell<Frame> {
        unimplemented!()
        // self.clone().frame
    }

    pub fn update_end_ts(&self, end_ts: DateTime<Utc>) {
        // self.frame.borrow_mut().end_ts = Some(end_ts);
        unimplemented!()
    }

    pub fn update_breadcrumbs(&self, value: serde_json::value::Value) {
        // self.frame.borrow_mut().breadcrumbs = Some(value);
        unimplemented!()
    }

    pub fn get_data(&self) -> String {
        // serde_json::to_value(self.clone()).unwrap().to_string()
        unimplemented!()
    }

}
pub fn observe_field(ctx: &Context, name: &str, value: &str){
    unimplemented!()
}

pub fn observe_string(ctx: &Context, name: &str, value: String){
    unimplemented!()
}

pub fn observe_i32(ctx: &Context, name: &str, value: i32){
    unimplemented!()
}
