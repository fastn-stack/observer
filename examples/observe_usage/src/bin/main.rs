extern crate observe_usage;

use observe_usage::policy::Policy;
use observer::{context::*, queue::DemoQueue, };
use std::string::ToString;

fn main() {
    for x in 0..1 {
//        let ctx = Context::new(
//            "db_call_test".to_string(),
//            Box::new(DemoQueue {
//                name: "DemoQueue".to_string(),
//            }),
//        );

        create_context(
            "db_call_test".to_string(),
            Box::new(DemoQueue {
                name: "DemoQueue".to_string(),
            })
        );

        let result = Policy::create_policy("activa_policy");
        end_context();
        //&ctx.finalise();
        //println!("Context: {:#?}", ctx);
    }
}
