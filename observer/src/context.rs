use chrono::prelude::*;

static SPACE: usize = 4;

#[derive(Debug, Serialize)]
pub struct Context {
    id: String,
    key: String,
    #[serde(serialize_with = "datetime_to_millis")]
    pub created_on: DateTime<Utc>,
    pub span_stack: std::cell::RefCell<Vec<crate::Span>>,
}

fn datetime_to_millis<S>(d: &DateTime<Utc>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.serialize_i64(d.timestamp_millis())
}

thread_local! {
    static CONTEXT: std::cell::RefCell<Option<Context>> = std::cell::RefCell::new(None);
}

impl Context {
    pub fn new(id: String) -> Context {
        Context {
            id,
            key: uuid::Uuid::new_v4().to_string(),
            created_on: Utc::now(),
            span_stack: std::cell::RefCell::new(vec![crate::Span::new("main")]),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn start_span(&self, id: &str) {
        self.span_stack.borrow_mut().push(crate::Span::new(id));
    }

    #[allow(dead_code)]
    pub(crate) fn observe_span_id(&self, id: &str) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.set_id(id);
            self.span_stack.borrow_mut().push(frame);
        }
    }

    pub(crate) fn observe_span_field(&self, key: &'static str, value: serde_json::Value) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.add_breadcrumbs(key, value);
            self.span_stack.borrow_mut().push(frame);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn observe_query(
        &self,
        query: String,
        bind: Option<String>,
        result: Result<usize, String>,
    ) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.add_query(query, bind, result);
            self.span_stack.borrow_mut().push(frame);
        }
    }

    pub(crate) fn observe_span_transient_field(&self, key: &'static str, value: serde_json::Value) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.add_transient_field(key, value);
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

    pub(crate) fn span_log(&self, value: &'static str) {
        let frame = self.span_stack.borrow_mut().pop();
        if let Some(mut frame) = frame {
            frame.add_log(value);
            self.span_stack.borrow_mut().push(frame);
        }
    }

    pub fn end_span(&self, _is_critical: bool, err: Option<String>) {
        let child = self.span_stack.borrow_mut().pop();
        let parent = self.span_stack.borrow_mut().pop();
        if let Some(mut child_frame) = child {
            child_frame.set_success(err.is_none()).set_err(err).end();
            if let Some(mut parent_frame) = parent {
                parent_frame.add_sub_frame(child_frame.created_on, child_frame);
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

    pub fn trace_without_data(&self, print_values: bool) -> String {
        let mut buffer = "".to_string();
        for span in self.span_stack.borrow().iter() {
            print_span_without_data(&mut buffer, span, 0, print_values);
        }
        buffer
    }
}

pub fn print_span_without_data(
    buffer: &mut String,
    span: &crate::Span,
    space: usize,
    print_values: bool,
) {
    if space != 0 {
        buffer.push_str(&format!("{}\n", span.id.as_str()));
    }

    for (_, item) in span.items.iter() {
        match item {
            crate::SpanItem::Log { message } => {
                buffer.push_str(&format!(
                    "{:_>space$}- {message}\n",
                    "",
                    message = message,
                    space = space,
                ));
            }
            crate::SpanItem::Field { name, value } => {
                if print_values {
                    buffer.push_str(&format!(
                        "{:_>space$}- {name}={value}\n",
                        "",
                        name = name,
                        value = if let serde_json::Value::String(s) = value {
                            s.to_string()
                        } else {
                            value.to_string()
                        },
                        space = space
                    ));
                } else {
                    buffer.push_str(&format!(
                        "{:_>space$}- {name}: observed\n",
                        "",
                        name = name,
                        space = space
                    ));
                }
            }
            crate::SpanItem::TransientField { name, value: _ } => {
                buffer.push_str(&format!(
                    "{:_>space$}- {name}: observed\n",
                    "",
                    name = name,
                    space = space
                ));
            }
            crate::SpanItem::Query {
                query,
                bind,
                result,
            } => {
                buffer.push_str(&format!(
                    "{:_>space$}- query: {}\n",
                    "",
                    query,
                    space = space
                ));
                if bind.is_some() {
                    buffer.push_str(&format!(
                        "{:_>space$}   bind: observed\n",
                        "",
                        space = space
                    ));
                }
                match result {
                    Ok(rows) => {
                        if print_values {
                            buffer.push_str(&format!(
                                "{:_>space$}   rows: {}\n",
                                "",
                                rows,
                                space = space
                            ))
                        } else {
                            buffer.push_str(&format!(
                                "{:_>space$}   rows: observed\n",
                                "",
                                space = space
                            ))
                        }
                    }
                    Err(e) => {
                        buffer.push_str(&format!("{:_>space$}  error: {}\n", "", e, space = space))
                    }
                };
            }
            crate::SpanItem::Frame(inner) => {
                buffer.push_str(&format!("{:_>space$}- ", "", space = space));
                print_span_without_data(buffer, inner, space + SPACE, print_values);
            }
        }
    }
}
