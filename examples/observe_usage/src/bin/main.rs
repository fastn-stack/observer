extern crate observe_usage;

use observe_usage::policy::Policy;
use observer::{context::Context, queue::DemoQueue};
use std::string::ToString;

fn main() {
    let ctx = Context::new("test_context".to_string(), Box::new(DemoQueue{name: "Abrar".to_string()}));
    let result = Policy::create_policy(&ctx, "activa_policy");
    print!("result: {:?}", result);
    &ctx.finalise();
}
