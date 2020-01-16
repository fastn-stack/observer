extern crate observe_usage;

use observe_usage::policy::Policy;
use observer::Observer;
use std::string::ToString;

fn main() {

    let back: Vec<Box<dyn observer::Backend>> = vec![Box::new(observer::observer_newrelic::ObserverNewRelic::new())];
    observer::create_observer(back);

    observer::create_context("main");
    let _result = Policy::create_policy("activa_policy");
    observer::end_context();

    //    for _x in 0..1 {
    //        let ctx = Context::new(
    //            "db_call_test".to_string(),
    //            Box::new(DemoQueue {
    //                name: "DemoQueue".to_string(),
    //            }),
    //        );
    //
    //        create_context(
    //            "db_call_test".to_string(),
    //            Box::new(DemoQueue {
    //                name: "DemoQueue".to_string(),
    //            }),
    //        );
    //
    //        let _result = Policy::create_policy("activa_policy");
    //        end_context();
    //        &ctx.finalise();
    //        println!("Context: {:#?}", ctx);
    //    }
}
