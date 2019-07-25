use crate::{context::Context, Result};

pub fn observe_with_result<F, T>(
    ctx: &Context,
    table_name: &str,
    is_critical: bool,
    run: F,
) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    ctx.start_frame(table_name);
    match run() {
        Ok(r) => {
            ctx.end_frame(is_critical, None);
            Ok(r)
        }
        Err(e) => {
            ctx.end_frame(is_critical, Some(e.to_string()));
            Err(e)
        }
    }
}

pub fn observe_all<F, T>(ctx: &Context, table_name: &str, is_critical: bool, run: F) -> T
where
    F: FnOnce() -> T,
{
    ctx.start_frame(table_name);
    let result = run();
    ctx.end_frame(is_critical, None);
    result
}
