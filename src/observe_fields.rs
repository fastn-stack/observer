use crate::Context;
use std::collections::HashMap;

fn observe_field(ctx: &Context, name: &str, value: serde_json::Value) {
    let frame = ctx.frame_stack.borrow_mut().pop();
    if let Some(mut frame) = frame {
        frame.add_breadcrumbs(name, json!(value));
        ctx.frame_stack.borrow_mut().push(frame);
    }
}

pub fn observe_string(ctx: &Context, name: &str, value: &str) {
    observe_field(ctx, name, json!(value));
}

pub fn observe_bool(ctx: &Context, name: &str, value: bool) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_char(ctx: &Context, name: &str, value: char) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_i8(ctx: &Context, name: &str, value: i8) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_i16(ctx: &Context, name: &str, value: i16) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_i32(ctx: &Context, name: &str, value: i32) {
    observe_field(ctx, name, json!(value));
}

pub fn observe_i64(ctx: &Context, name: &str, value: i64) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_isize(ctx: &Context, name: &str, value: isize) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_u8(ctx: &Context, name: &str, value: u8) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_u16(ctx: &Context, name: &str, value: u16) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_u32(ctx: &Context, name: &str, value: u32) {
    observe_field(ctx, name, json!(value));
}

pub fn observe_u64(ctx: &Context, name: &str, value: u64) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_usize(ctx: &Context, name: &str, value: usize) {
    observe_field(ctx, name, json!(value))
}

pub fn observe_f64(ctx: &Context, name: &str, value: f64) {
    observe_field(ctx, name, json!(value));
}

pub fn observe_f32(ctx: &Context, name: &str, value: f32) {
    observe_field(ctx, name, json!(value));
}

fn observe_result(ctx: &Context, result: serde_json::Value) {
    let frame = ctx.frame_stack.borrow_mut().pop();
    if let Some(mut frame) = frame {
        frame.set_result(result);
        ctx.frame_stack.borrow_mut().push(frame);
    }
}

pub fn observe_result_string(ctx: &Context, value: &str) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_bool(ctx: &Context, value: bool) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_char(ctx: &Context, value: char) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_i8(ctx: &Context, value: i8) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_i16(ctx: &Context, value: i16) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_i32(ctx: &Context, value: i32) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_i64(ctx: &Context, value: i64) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_isize(ctx: &Context, value: isize) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_u8(ctx: &Context, value: u8) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_u16(ctx: &Context, value: u16) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_u32(ctx: &Context, value: u32) {
    observe_result(ctx, json!(value));
}

pub fn observe_result_u64(ctx: &Context, value: u64) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_usize(ctx: &Context, value: usize) {
    observe_result(ctx, json!(value))
}

pub fn observe_result_f64(ctx: &Context, value: f64) {
    observe_result(ctx, json!(value));
}

pub fn observe_result_f32(ctx: &Context, value: f32) {
    observe_result(ctx, json!(value));
}

pub fn observe_result_object(ctx: &Context, value: serde_json::Value) {
    observe_result(ctx, value);
}

pub fn observe_result_list<T: serde::Serialize>(ctx: &Context, value: &[T]) {
    observe_result(ctx, json!(value));
}

pub fn observe_result_map<K, V>(ctx: &Context, value: &HashMap<K, V>)
where
    K: std::hash::Hash + std::cmp::Eq + serde::Serialize,
    V: serde::Serialize,
{
    observe_result(ctx, json!(value));
}
