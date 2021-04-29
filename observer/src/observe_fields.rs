use crate::observe_result;
use crate::{field, transient_field};
use std::collections::HashMap;

// TODO: let it accept either str or String
pub fn observe_string(name: &'static str, value: &str) {
    field(name, json!(value));
}

pub fn observe_json(name: &'static str, value: serde_json::Value) {
    field(name, value)
}

pub fn observe_bool(name: &'static str, value: bool) {
    field(name, json!(value))
}

pub fn observe_optional_bool(name: &'static str, value: Option<bool>) {
    if let Some(v) = value {
        field(name, json!(v))
    }
}

pub fn observe_char(name: &'static str, value: char) {
    field(name, json!(value))
}

pub fn observe_i8(name: &'static str, value: i8) {
    field(name, json!(value))
}

pub fn observe_i16(name: &'static str, value: i16) {
    field(name, json!(value))
}

pub fn observe_i32(name: &'static str, value: i32) {
    field(name, json!(value));
}

pub fn observe_i64(name: &'static str, value: i64) {
    field(name, json!(value))
}

pub fn observe_isize(name: &'static str, value: isize) {
    field(name, json!(value))
}

pub fn observe_u8(name: &'static str, value: u8) {
    field(name, json!(value))
}

pub fn observe_u16(name: &'static str, value: u16) {
    field(name, json!(value))
}

pub fn observe_u32(name: &'static str, value: u32) {
    field(name, json!(value));
}

pub fn observe_u64(name: &'static str, value: u64) {
    field(name, json!(value))
}

pub fn observe_usize(name: &'static str, value: usize) {
    field(name, json!(value))
}

pub fn observe_f64(name: &'static str, value: f64) {
    field(name, json!(value));
}

pub fn observe_f32(name: &'static str, value: f32) {
    field(name, json!(value));
}

pub fn transient_string(name: &'static str, value: &str) {
    transient_field(name, json!(value));
}

pub fn transient_json(name: &'static str, value: serde_json::Value) {
    transient_field(name, value)
}

pub fn transient_bool(name: &'static str, value: bool) {
    transient_field(name, json!(value))
}

pub fn transient_char(name: &'static str, value: char) {
    transient_field(name, json!(value))
}

pub fn transient_i8(name: &'static str, value: i8) {
    transient_field(name, json!(value))
}

pub fn transient_i16(name: &'static str, value: i16) {
    transient_field(name, json!(value))
}

pub fn transient_i32(name: &'static str, value: i32) {
    transient_field(name, json!(value));
}

pub fn transient_i64(name: &'static str, value: i64) {
    transient_field(name, json!(value))
}

pub fn transient_isize(name: &'static str, value: isize) {
    transient_field(name, json!(value))
}

pub fn transient_u8(name: &'static str, value: u8) {
    transient_field(name, json!(value))
}

pub fn transient_u16(name: &'static str, value: u16) {
    transient_field(name, json!(value))
}

pub fn transient_u32(name: &'static str, value: u32) {
    transient_field(name, json!(value));
}

pub fn transient_u64(name: &'static str, value: u64) {
    transient_field(name, json!(value))
}

pub fn transient_usize(name: &'static str, value: usize) {
    transient_field(name, json!(value))
}

pub fn transient_f64(name: &'static str, value: f64) {
    transient_field(name, json!(value));
}

pub fn transient_f32(name: &'static str, value: f32) {
    transient_field(name, json!(value));
}

pub fn observe_result_string(value: &str) {
    observe_result(json!(value))
}

pub fn observe_result_bool(value: bool) {
    observe_result(json!(value))
}

pub fn observe_result_char(value: char) {
    observe_result(json!(value))
}

pub fn observe_result_i8(value: i8) {
    observe_result(json!(value))
}

pub fn observe_result_i16(value: i16) {
    observe_result(json!(value))
}

pub fn observe_result_i32(value: i32) {
    observe_result(json!(value))
}

pub fn observe_result_i64(value: i64) {
    observe_result(json!(value))
}

pub fn observe_result_isize(value: isize) {
    observe_result(json!(value))
}

pub fn observe_result_u8(value: u8) {
    observe_result(json!(value))
}

pub fn observe_result_u16(value: u16) {
    observe_result(json!(value))
}

pub fn observe_result_u32(value: u32) {
    observe_result(json!(value));
}

pub fn observe_result_u64(value: u64) {
    observe_result(json!(value))
}

pub fn observe_result_usize(value: usize) {
    observe_result(json!(value))
}

pub fn observe_result_f64(value: f64) {
    observe_result(json!(value));
}

pub fn observe_result_f32(value: f32) {
    observe_result(json!(value));
}

pub fn observe_result_object(value: serde_json::Value) {
    observe_result(value);
}

pub fn observe_result_list<T: serde::Serialize>(value: &[T]) {
    observe_result(json!(value));
}

pub fn observe_result_map<K, V, S: ::std::hash::BuildHasher>(value: &HashMap<K, V, S>)
where
    K: std::hash::Hash + std::cmp::Eq + serde::Serialize,
    V: serde::Serialize,
{
    observe_result(json!(value));
}
