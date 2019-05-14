extern crate observe_usage;

use observer::{context::Context, queue::QueueEnum};
use std::string::ToString;
use observe_usage::policy::Policy;

fn main() {
    let ctx = Context::new("test_context".to_string(), QueueEnum::DummyQueue);
    let result = Policy::create_policy(&ctx, "activa_policy");
    Policy::update_policy(&ctx, "activa", "activa_policy");
    print!("result: {:?}",result);
    &ctx.finalise();
}

