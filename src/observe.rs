use crate::{context::Context, event::Event, frame::Frame, resulty::Resulty, Result};
use chrono::Utc;

pub fn observe<F, T>(ctx: &Context, table_name: &str, is_critical: bool, run: F) -> Result<T>
where
    F: FnOnce() -> std::result::Result<T, failure::Error>,
    T: std::fmt::Debug + serde::Serialize + Resulty,
{
    let temp_frame = ctx.replace_frame(Frame::new(table_name.to_string()));
    let result = match run() {
        Ok(response) => {
            ctx.end_frame(temp_frame, json!(response), true, is_critical, &ctx.queue);
            Ok(response)
        }
        Err(err) => {
            ctx.end_frame(temp_frame, json!(format!("{:?}", err)), false, is_critical, &ctx.queue);
            Err(err)
        }
    };
    ctx.mut_frame().end();
    result
}
