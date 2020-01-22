use crate::{span::Span, utils, Result};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;

pub static mut DIR_EXISTS: bool = false;
pub static mut CON_DIR_EXISTS: bool = false;
static SPACE: usize = 4;

lazy_static! {
    pub static ref LOG_DIR: String = {
        let log_dir = format!(
            "{}/{}",
            std::env::var("LOG_DIR").unwrap_or_else(|_| "/var/log".to_owned()),
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
    pub span_stack: std::cell::RefCell<Vec<Span>>,
}

thread_local! {
    static CONTEXT: std::cell::RefCell<Option<Context>> = std::cell::RefCell::new(None);
}

impl Context {
    pub fn new(id: String) -> Context {
        Context {
            id,
            key: uuid::Uuid::new_v4().to_string(),
            span_stack: std::cell::RefCell::new(vec![Span::new("main")]),
        }
    }

    pub fn start_span(&self, id: &str) {
        self.span_stack.borrow_mut().push(Span::new(id));
    }

    pub(crate) fn observe_span_id(&self, id: &str) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.set_id(id);
            self.span_stack.borrow_mut().push(frame);
        }
    }

    pub(crate) fn observe_span_field(&self, key: &str, value: serde_json::Value) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.add_breadcrumbs(key, value);
            self.span_stack.borrow_mut().push(frame);
        }
    }

    pub(crate) fn observe_span_result(&self, value: impl serde::Serialize) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.set_result(value);
            self.span_stack.borrow_mut().push(frame);
        }
    }

    pub fn end_span(&self, _is_critical: bool, err: Option<String>) {
        let child = self.span_stack.borrow_mut().pop();
        let parent = self.span_stack.borrow_mut().pop();
        if let Some(mut child_frame) = child {
            child_frame.set_success(err.is_none()).set_err(err).end();
            if let Some(mut parent_frame) = parent {
                parent_frame.sub_frames.push(child_frame);
                self.span_stack.borrow_mut().push(parent_frame);
            } else {
                self.span_stack.borrow_mut().push(child_frame);
            }
        }
    }

    pub(crate) fn end_ctx_frame(&self) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.end();
            self.span_stack.borrow_mut().push(frame);
        }
    }

    pub fn finalise(&self) -> Result<()> {
        self.end_ctx_frame();
        if true {
            // println!("{:#?}", self.span_stack);
            print_context(&self);
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

pub(crate) fn print_context(ctx: &Context) {
    let mut writer = "".to_string();
    let frame = ctx.span_stack.borrow();
    if let Some(frame) = frame.first() {
        let dur = frame
            .end_time
            .as_ref()
            .unwrap_or(&chrono::Utc::now())
            .signed_duration_since(frame.start_time);
        writer.push_str(&format!(
            "context: {} [{}ms, {}]\n",
            ctx.id,
            dur.num_milliseconds(),
            frame.start_time
        ));
        print_span(&mut writer, &frame.sub_frames, SPACE);
    }
    println!("{}", writer);
}

pub(crate) fn print_span(writer: &mut String, spans: &Vec<Span>, space: usize) {
    for span in spans.iter() {
        let dur = span
            .end_time
            .as_ref()
            .unwrap_or(&chrono::Utc::now())
            .signed_duration_since(span.start_time);
        writer.push_str(&format!(
            "{:>space$}{}: {}ms\n",
            "",
            span.id,
            dur.num_milliseconds(),
            space = space
        ));
        for (key, value) in span.breadcrumbs.iter() {
            writer.push_str(&format!(
                "{:>space$}@{}: {}\n",
                "",
                key,
                value,
                space = space + SPACE
            ));
        }
        if let Some(success) = span.success {
            writer.push_str(&format!(
                "{:>space$}@@success: {}\n",
                "",
                success,
                space = space + SPACE
            ));
        }
        print_span(writer, &span.sub_frames, space + SPACE);
    }
}
