use crate::observe_field;
use crate::observe_result;
use std::collections::HashMap;

pub fn observe_string(name: &str, value: &str) {
    observe_field(name, json!(value));
}

pub fn observe_bool(name: &str, value: bool) {
    observe_field(name, json!(value))
}

pub fn observe_char(name: &str, value: char) {
    observe_field(name, json!(value))
}

pub fn observe_i8(name: &str, value: i8) {
    observe_field(name, json!(value))
}

pub fn observe_i16(name: &str, value: i16) {
    observe_field(name, json!(value))
}

pub fn observe_i32(name: &str, value: i32) {
    observe_field(name, json!(value));
}

pub fn observe_i64(name: &str, value: i64) {
    observe_field(name, json!(value))
}

pub fn observe_isize(name: &str, value: isize) {
    observe_field(name, json!(value))
}

pub fn observe_u8(name: &str, value: u8) {
    observe_field(name, json!(value))
}

pub fn observe_u16(name: &str, value: u16) {
    observe_field(name, json!(value))
}

pub fn observe_u32(name: &str, value: u32) {
    observe_field(name, json!(value));
}

pub fn observe_u64(name: &str, value: u64) {
    observe_field(name, json!(value))
}

pub fn observe_usize(name: &str, value: usize) {
    observe_field(name, json!(value))
}

pub fn observe_f64(name: &str, value: f64) {
    observe_field(name, json!(value));
}

pub fn observe_f32(name: &str, value: f32) {
    observe_field(name, json!(value));
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

pub fn observe_result_map<K, V>(value: &HashMap<K, V>)
where
    K: std::hash::Hash + std::cmp::Eq + serde::Serialize,
    V: serde::Serialize,
{
    observe_result(json!(value));
}
