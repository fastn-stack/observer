use crate::event::Event;
use crate::AResult;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::thread;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Context {
    pub key: String,
    pub context_id: String,
    pub frame: Frame,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub key: String,
    pub frame_id: String,
    pub breadcrumbs: Option<serde_json::Value>,
    pub start_ts: DateTime<Utc>,
    pub end_ts: Option<DateTime<Utc>>,
    pub sub_frames: Vec<Frame>,
}

impl Context {
    pub fn modify_context(ctx: &RefCell<Context>, new_frame: Frame) {
        ctx.borrow_mut().frame = new_frame;
    }

    pub fn modify_add(ctx: &RefCell<Context>, new_frame: Frame) {
        ctx.borrow_mut().frame.sub_frames.push(new_frame)
    }
}
