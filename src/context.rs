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

fn observe_it(ctx: &Context, name: &str, value: serde_json::Value) {
    ctx.frame
        .borrow_mut()
        .add_breadcrumbs(name, json!(value));
}

pub fn observe_string(ctx: &Context, name: &str, value: String) {
    observe_it(ctx, name, json!(value));
}

pub fn observe_bool(ctx: &Context, name: &str, value: bool) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_char(ctx: &Context, name: &str, value: char) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_i8(ctx: &Context, name: &str, value: i8) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_i16(ctx: &Context, name: &str, value: i16) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_i32(ctx: &Context, name: &str, value: i32) {
    observe_it(ctx, name, json!(value));
}

pub fn observe_i64(ctx: &Context, name: &str, value: i64) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_isize(ctx: &Context, name: &str, value: isize) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_u8(ctx: &Context, name: &str, value: u8) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_u16(ctx: &Context, name: &str, value: u16) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_u32(ctx: &Context, name: &str, value: u32) {
    observe_it(ctx, name, json!(value));
}

pub fn observe_u64(ctx: &Context, name: &str, value: u64) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_usize(ctx: &Context, name: &str, value: usize) {
    observe_it(ctx, name, json!(value))
}

pub fn observe_f64(ctx: &Context, name: &str, value: f64) {
    observe_it(ctx, name, json!(value));
}

pub fn observe_f32(ctx: &Context, name: &str, value: f32) {
    observe_it(ctx, name, json!(value));
}

pub fn observe_field(_ctx: &Context, _name: &str, _value: &str) {
    unimplemented!()
}