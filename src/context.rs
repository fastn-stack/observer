use crate::{frame::Frame, queue::Queue, utils, Result};
use serde_derive::{Deserialize, Serialize};
use std::{cell::RefCell, cell::RefMut, env, io::Write};

pub static mut DIR_EXISTS: bool = false;
pub static mut CON_DIR_EXISTS: bool = false;
lazy_static! {
    pub static ref LOG_DIR: String = {
        let log_dir = format!(
            "{}/{}",
            env::var("LOG_DIR").unwrap_or("/var/log".to_owned()),
            "observer/"
        );
        match utils::create_dir_all_if_not_exists(&log_dir) {
            Ok(_) => {
                println!("Observer LOG_DIR :: {}", log_dir);
                unsafe { DIR_EXISTS = true }
            }
            Err(err) => {
                println!("Not able to create/find dir LOG_DIR :: {}", log_dir);
                println!("Make sure it will not be able to store logs at local");
                println!("Err is {:?}", err);
            }
        }
        log_dir
    };
    pub static ref CONTEXT_DIR: String = {
        let context_dir = format!("{}{}/", LOG_DIR.to_string(), "context");
        match utils::create_dir_all_if_not_exists(&context_dir) {
            Ok(_) => {
                println!("Context LOG_DIR :: {}", context_dir);
                unsafe { CON_DIR_EXISTS = true }
            }
            Err(err) => {
                println!("Not able to create/find dir CONTEXT_DIR :: {}", context_dir);
                println!("Make sure it will not be able to store logs at local");
                println!("Err is {:?}", err);
            }
        }
        context_dir
    };
}

pub fn is_log_dir_exists() -> bool {
    let _ = LOG_DIR.to_string();
    unsafe { DIR_EXISTS }
}

pub fn is_ctx_dir_exists() -> bool {
    let _ = CONTEXT_DIR.to_string();
    unsafe { CON_DIR_EXISTS }
}

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
        queue: &Box<dyn Queue>,
    ) {
        self.mut_frame()
            .end()
            .set_success(success)
            .set_result(result);
        let ctx_current_frame = self.replace_frame(frame);
        ctx_current_frame.save(is_critical, queue);
        self.frame.borrow_mut().add_sub_frame(ctx_current_frame);
    }

    pub fn replace_frame(&self, frame: Frame) -> Frame {
        self.frame.replace(frame)
    }

    pub fn mut_frame(&self) -> RefMut<Frame> {
        self.frame.borrow_mut()
    }

    pub fn finalise(&self) -> Result<()> {
        if is_ctx_dir_exists() {
            match utils::create_file(&CONTEXT_DIR, self.key.as_str()) {
                Ok(mut file) => {
                    if let Err(err) = file.write(json!(self).to_string().as_bytes()) {
                        println!("Context file write error :: {:#?}", err);
                    };
                }
                Err(err) => {
                    println!("Context file create error {:#?}", err);
                }
            };
        }
        Ok(())
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}

fn observe_it(ctx: &Context, name: &str, value: serde_json::Value) {
    ctx.frame.borrow_mut().add_breadcrumbs(name, json!(value));
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
