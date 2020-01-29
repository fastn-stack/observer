use crate::span::Span;
use serde_derive::{Deserialize, Serialize};

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

    pub fn id(&self) -> String {
        self.id.to_string()
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

    pub(crate) fn span_log(&self, value: &str) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.add_logs(value);
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

    pub fn finalise(&self) {
        self.end_ctx_frame();
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}
