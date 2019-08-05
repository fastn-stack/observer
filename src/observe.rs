use crate::context::*;

pub fn observe_with_result<F, T, E>(
    // ctx: &Context,
    table_name: &str,
    is_critical: bool,
    run: F,
) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
    E: std::fmt::Debug,
{
    start_frame(table_name);
    match run() {
        Ok(r) => {
            end_frame(is_critical, None);
            Ok(r)
        }
        Err(e) => {
            end_frame(is_critical, Some(format!("{:?}", e)));
            Err(e)
        }
    }
}

pub fn observe_all<F, T>(table_name: &str, is_critical: bool, run: F) -> T
where
    F: FnOnce() -> T,
{
    start_frame(table_name);
    let result = run();
    end_frame(is_critical, None);
    result
}
