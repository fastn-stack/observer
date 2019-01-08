use chrono::prelude::*;
use chrono::{DateTime, Duration};

#[derive(Debug)]
pub struct Span {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub ekind: String,
    pub delta: Duration,
}

#[derive(Debug)]
pub struct Context {
    spans: Vec<Span>,
}

impl Context {
    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span)
    }
}

impl Default for Context {
    fn default() -> Context {
        Context { spans: Vec::new() }
    }
}
