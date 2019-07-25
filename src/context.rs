use crate::{frame::Frame, queue::Queue, utils, Result};
use ackorelic::newrelic_fn::{nr_end_transaction, nr_start_web_transaction};
use serde_derive::{Deserialize, Serialize};
use std::{cell::RefCell, env, io::Write};

pub static mut DIR_EXISTS: bool = false;
pub static mut CON_DIR_EXISTS: bool = false;

lazy_static! {
    pub static ref LOG_DIR: String = {
        let log_dir = format!(
            "{}/{}",
            env::var("LOG_DIR").unwrap_or_else(|_| "/var/log".to_owned()),
            "observer/"
        );
        match utils::create_dir_all_if_not_exists(&log_dir) {
            Ok(_) => {
                // println!("Observer LOG_DIR :: {}", log_dir);
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
                // println!("Context LOG_DIR :: {}", context_dir);
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
    pub frame_stack: RefCell<Vec<Frame>>,
    pub queue: Box<dyn Queue>,
}

impl Context {
    pub fn new(id: String, queue: Box<Queue>) -> Context {
        // TODO: For new_relic purpose, Later need to remove this dependency
        nr_start_web_transaction(&id);
        Context {
            id,
            key: uuid::Uuid::new_v4().to_string(),
            frame_stack: RefCell::new(vec![Frame::new("main")]),
            queue,
        }
    }

    pub fn start_frame(&self, id: &str) {
        self.frame_stack.borrow_mut().push(Frame::new(id));
    }

    pub fn end_frame(&self, is_critical: bool, err: Option<String>) {
        let child = self.frame_stack.borrow_mut().pop();
        let parent = self.frame_stack.borrow_mut().pop();
        if let Some(mut child_frame) = child {
            child_frame.set_success(err.is_none()).set_err(err).end();
            child_frame.save(is_critical, &self.queue);
            if let Some(mut parent_frame) = parent {
                parent_frame.sub_frames.push(child_frame);
                self.frame_stack.borrow_mut().push(parent_frame);
            } else {
                self.frame_stack.borrow_mut().push(child_frame);
            }
        }
    }

    fn end_ctx_frame(&self) {
        let frame = self.frame_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.end();
            self.frame_stack.borrow_mut().push(frame);
        }
    }

    pub fn finalise(&self) -> Result<()> {
        // TODO: For new_relic purpose, Later need to remove this dependency
        nr_end_transaction();
        self.end_ctx_frame();
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
