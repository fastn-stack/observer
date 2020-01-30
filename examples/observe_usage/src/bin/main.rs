extern crate observe_usage;
use observe_usage::policy::Policy;

fn main() {
    let logger = observer::backends::logger::Logger::builder()
        .with_path("/tmp/observer.log")
        .with_stdout()
        .build();

    observer::builder(Box::new(logger))
        .create_context("main")
        .init();

    let _result = Policy::create_policy("activa_policy");
    observer::end_context();
}
