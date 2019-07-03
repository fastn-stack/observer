use crate::{frame::Frame, queue::Queue, utils, Result};
use serde_derive::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    cell::RefMut,
    fs::File,
    io::Write,
};

pub static LOCAL_FILE_SYSTEM_DIRECTORY: &str = "/Users/abrar/observer_files/";

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    id: String,
    key: String,
    pub queue: Box<dyn Queue>,
    pub frame: RefCell<Frame>,
}


impl Context {
    pub fn new(id: String, queue: Box<Queue>) -> Context {
        Context {
            id,
            frame: RefCell::new(Frame::new("main".to_string())),
            key: uuid::Uuid::new_v4().to_string(),
            queue,
        }
    }

    pub fn start_frame(&self) {
        self.frame.borrow_mut().start();
    }

    pub fn end_frame(
        &self,
        frame: Frame,
        result: serde_json::Value,
        success: bool,
        is_critical: bool,
        queue: &Box<dyn Queue>
    ) {
        self.mut_frame()
            .end()
            .set_success(success)
            .set_result(result);

        let ctx_current_frame = self.replace_frame(frame);
        ctx_current_frame.save(is_critical,  queue);
        self.frame.borrow_mut().add_sub_frame(ctx_current_frame);
    }

    pub fn replace_frame(&self, frame: Frame) -> Frame {
        self.frame.replace(frame)
    }

    pub fn mut_frame(&self) -> RefMut<Frame> {
        self.frame.borrow_mut()
    }

    pub fn finalise(&self)-> Result<()> {
        let path = LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + "context/";
        let _ = utils::create_dir_if_not_exists(&path);
        let mut file = File::create(path + "/" + self.key.as_str()).unwrap();
        file.write(json!(self).to_string().as_bytes()).unwrap(); // TODO
        println!("{:#?}", self);
        Ok(())
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}

pub fn observe_field(_ctx: &Context, _name: &str, _value: &str) {
    unimplemented!()
}

pub fn observe_string(ctx: &Context, name: &str, value: String) {
    ctx.frame
        .borrow_mut()
        .add_value(name, serde_json::to_value(value).unwrap());
}

pub fn observe_i32(ctx: &Context, name: &str, value: i32) {
    ctx.frame
        .borrow_mut()
        .add_value(name, serde_json::to_value(value).unwrap());
}
