use chrono::Utc;
use crate::{context::Context, context::Frame, event::Event, AResult, OError};

pub fn observe<F, T, K>(ctx: &Context, event: T, closure: F) -> AResult<T>
where
    F: FnOnce() -> AResult<T>,
    K: serde::Serialize,
    T: std::fmt::Debug + Event<T, K> + serde::Serialize + std::clone::Clone,
    std::result::Result<T, OError>: std::clone::Clone,
{
    let new_frame = Frame::new(event.name());
    let temp1 = ctx.get_frame();

    ctx.modify_context(new_frame);

    let result = closure();

    ctx.update_end_ts(Utc::now());
    ctx.update_breadcrumbs(serde_json::to_value(event.map(ctx, &result)).unwrap());

    let temp2 = ctx.get_frame().into_inner();
    if event.is_critical() {
        ctx.en_queue(temp2.clone());
    } else {
        ctx.save_on_local(event.destination(), temp2.clone());
    }

    ctx.modify_context(temp1.into_inner());
    ctx.modify_add(temp2);

    result
}
