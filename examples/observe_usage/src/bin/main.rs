extern crate observe_usage;

use observe_usage::policy::Policy;
use observer::{context::Context, queue::DemoQueue};
use std::string::ToString;

fn main() {
    for x in 0..5 {
        let ctx = Context::new(
            "db_call_test".to_string(),
            Box::new(DemoQueue {
                name: "DemoQueue".to_string(),
            }),
        );
        let result = Policy::create_policy(&ctx, "activa_policy");
        println!("Context: {:#?}", ctx);
        &ctx.finalise();
    }
}
