use crate::frame::Frame;
use crate::queue::{Queue, QueueEnum};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

pub static LOCAL_FILE_SYSTEM_DIRECTORY: &str = "/Users/venkatesh/observer_files/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    key: String,
    context_id: String,
    queue: QueueEnum,
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
    pub fn new(context_id: String, queue: QueueEnum) -> Context {
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
//        self.update_end_ts(Utc::now());
//        let destination_folder = LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + "context/";
//        if !Path::new(destination_folder.as_str()).exists() {
//            create_dir(destination_folder.clone()).unwrap(); // TODO
//        }
//        let mut file = File::create(destination_folder + "/" + self.key.as_str()).unwrap();
//        file.write(self.get_data().as_bytes()).unwrap(); // TODO
    }

    pub fn start_frame(&self, frame_id: String) -> Frame {
        let temp = self.clone().frame;
        self.frame.replace(Frame::new(frame_id));
        temp.into_inner()
    }

    pub fn end_frame(&self, frame: Frame, critical: bool, result: String, success: bool) {
        self.frame.borrow_mut().end_ts = Some(Utc::now());
        self.frame.borrow_mut().result = Some(result);
        self.frame.borrow_mut().success = Some(success);
        let temp = self.clone().frame;
        temp.clone().into_inner().save(critical);
        self.modify_context(frame);
        self.modify_add(temp.into_inner());
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

    pub fn update_end_ts(&self, end_ts: DateTime<Utc>) {
        self.frame.borrow_mut().end_ts = Some(end_ts);
    }

    pub fn get_data(&self) -> String {
        serde_json::to_value(self.clone()).unwrap().to_string()
    }

}
pub fn observe_field(ctx: &Context, name: &str, value: &str){
    unimplemented!()
}

pub fn observe_string(ctx: &Context, name: &str, value: String){
    ctx.frame.borrow_mut().add_value(name, serde_json::to_value(value).unwrap());
}

pub fn observe_i32(ctx: &Context, name: &str, value: i32){
    ctx.frame.borrow_mut().add_value(name, serde_json::to_value(value).unwrap());
}
