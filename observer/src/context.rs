use crate::span::Span;
use serde_derive::{Deserialize, Serialize};

static SPACE: usize = 4;

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

    pub fn finalise(&self, is_stdout: bool, is_file_log: bool) {
        self.end_ctx_frame();
        let log = if is_stdout || is_file_log {
            print_context(&self)
        } else {
            "".to_string()
        };
        if is_file_log {
            info!("{}", log);
        }
        if is_stdout {
            println!("{}", log);
        }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}

pub(crate) fn print_context(ctx: &Context) -> String {
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
    writer
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
        if span.logs.len() > 0 {
            writer.push_str(&format!("{:>space$}logs:\n", "", space = space + SPACE));
            for log in span.logs.iter() {
                let dur = log
                    .0
                    .signed_duration_since(span.start_time)
                    .num_milliseconds();
                writer.push_str(&format!(
                    "{:>space$} - {}ms: {log}\n",
                    "",
                    dur,
                    log = log.1,
                    space = space + SPACE + 2,
                ));
            }
        }
        print_span(writer, &span.sub_frames, space + SPACE);
    }
}
