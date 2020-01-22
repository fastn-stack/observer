extern crate observe_usage;
use observe_usage::policy::Policy;

fn main() {
    let back: Vec<Box<dyn observer::Backend>> = vec![Box::new(
        observer::observer_newrelic::ObserverNewRelic::new(),
    )];
    observer::create_observer(back);
    observer::create_context("main");
    let _result = Policy::create_policy("activa_policy");
    observer::end_context();
}
