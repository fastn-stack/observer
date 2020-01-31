extern crate observe_usage;
use observe_usage::policy::Policy;

fn main() {

    // Create Logger(implementation of Observer
    let logger = observer::backends::logger::Logger::builder()
        .with_path("/tmp/observer.log")
        .with_stdout()
        .build();
    // Build Observer Object
    observer::builder(logger).init();

    for x in 0..10 {
        // Testing with multi thread
        std::thread::spawn(move ||{
            observer::create_context(&("main_".to_string() + &x.to_string()));
            let _result = Policy::create_policy("activa_policy");
            observer::end_context();
        });
    }
    std::thread::sleep(std::time::Duration::from_secs(2));

}
