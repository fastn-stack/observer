extern crate observe_usage;
use observe_usage::policy::Policy;

fn main() {
    let logger = observer::backends::logger::Logger::builder()
        .with_path("/tmp/observer.log")
        .with_stdout()
        .build();

    observer::builder(Box::new(logger)).init();
    observer::create_context("main");
    let _result = Policy::create_policy("activa_policy");
    observer::end_context();
}
