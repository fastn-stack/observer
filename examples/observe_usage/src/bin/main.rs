extern crate observe_usage;

use observe_usage::policy::Policy;
use observer::{context::Context, queue::QueueEnum};
use std::string::ToString;

fn main() {
    let ctx = Context::new("test_context".to_string(), QueueEnum::DummyQueue);
    let result = Policy::create_policy(&ctx, "activa_policy");
    Policy::update_policy(&ctx, "activa", "activa_policy");
    print!("result: {:?}", result);
    &ctx.finalise();
}
