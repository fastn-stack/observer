use crate::{context::Context, event::Event, frame::Frame, Result};
use chrono::Utc;

pub fn observe<F,T>(ctx: &Context, table_name: &str, critical: bool, closure: F) -> Result<T>
where
    F: FnOnce() -> std::result::Result<T, failure::Error>,
    T: std::fmt::Debug + serde::Serialize,
{
    let frame = ctx.start_frame(table_name.to_string());
    let mut result: String = String::from("");
    let success: bool;
    match closure() {
        Ok(res) => {
            result = format!("{:?}",res);
            success = true;
            ctx.end_frame(frame,critical,result,success);
            Ok(res)
        },
        Err(E) => {
            result = format!("{:?}",E);
            success = false;
            ctx.end_frame(frame,critical,result,success);
            Err(E)
        }
    }
}