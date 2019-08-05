use crate::{frame::Span, queue::Queue, utils, Result};
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
    pub span_stack: RefCell<Vec<Span>>,
    pub queue: Box<dyn Queue>,
}

thread_local! {
    static CONTEXT: RefCell<Option<Context>> = RefCell::new(None);
}

pub fn create_context(id: String, queue: Box<Queue>) {
    CONTEXT.with(|obj| {
        RefCell::borrow_mut(obj);
        let mut context = obj.borrow_mut();
        if context.is_none() {
            context.replace(Context::new(id, queue));
        }
    });
}

pub fn end_context() {
    CONTEXT.with(|obj| {
        if let Some(ref ctx) = obj.borrow().as_ref() {
            let _ = ctx.finalise();
        }
    });
}

pub fn start_frame(id: &str) {
    CONTEXT.with(|obj| {
        if let Some(ref ctx) = obj.borrow().as_ref() {
            let _ = ctx.start_frame(id);
        }
    });
}

pub fn end_frame(is_critical: bool, err: Option<String>) {
    CONTEXT.with(|obj| {
        if let Some(ref ctx) = obj.borrow().as_ref() {
            let _ = ctx.end_frame(is_critical, err);
        }
    });
}

pub fn end_ctx_frame() {
    CONTEXT.with(|obj| {
        if let Some(ref ctx) = obj.borrow().as_ref() {
            let _ = ctx.end_ctx_frame();
        }
    });
}

pub(crate) fn observe_field(name: &str, value: serde_json::Value) {
    CONTEXT.with(|obj| {
        if let Some(ref ctx) = obj.borrow().as_ref() {
            let frame = ctx.span_stack.borrow_mut().pop();
            if let Some(mut frame) = frame {
                frame.add_breadcrumbs(name, json!(value));
                ctx.span_stack.borrow_mut().push(frame);
            }
        }
    });
}

pub fn observe_result(result: serde_json::Value) {
    CONTEXT.with(|obj| {
        if let Some(ref ctx) = obj.borrow().as_ref() {
            let frame = ctx.span_stack.borrow_mut().pop();
            if let Some(mut frame) = frame {
                frame.set_result(result);
                ctx.span_stack.borrow_mut().push(frame);
            }
        }
    });
}

impl Context {
    pub fn new(id: String, queue: Box<Queue>) -> Context {
        // TODO: For new_relic purpose, Later need to remove this dependency
        nr_start_web_transaction(&id);
        Context {
            id,
            key: uuid::Uuid::new_v4().to_string(),
            span_stack: RefCell::new(vec![Span::new("main")]),
            queue,
        }
    }

    pub fn start_frame(&self, id: &str) {
        self.span_stack.borrow_mut().push(Span::new(id));
    }

    pub fn end_frame(&self, is_critical: bool, err: Option<String>) {
        let child = self.span_stack.borrow_mut().pop();
        let parent = self.span_stack.borrow_mut().pop();
        if let Some(mut child_frame) = child {
            child_frame.set_success(err.is_none()).set_err(err).end();
            child_frame.save(is_critical, &self.queue);
            if let Some(mut parent_frame) = parent {
                parent_frame.sub_frames.push(child_frame);
                self.span_stack.borrow_mut().push(parent_frame);
            } else {
                self.span_stack.borrow_mut().push(child_frame);
            }
        }
    }

    fn end_ctx_frame(&self) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.end();
            self.span_stack.borrow_mut().push(frame);
        }
    }

    pub fn finalise(&self) -> Result<()> {
        // TODO: For new_relic purpose, Later need to remove this dependency
        self.end_ctx_frame();
        nr_end_transaction();
        if true {
            self.queue.enqueue(json!({ "Context": self }))
        } else {
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
        }
        Ok(())
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}
