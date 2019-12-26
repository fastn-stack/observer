#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate failure;
extern crate observer_attribute;

pub mod context;
mod event;
mod frame;
#[cfg(feature = "mysql")]
pub mod mysql;
pub mod observe;
pub mod observe_fields;
#[cfg(feature = "postgres")]
pub mod pg;
pub mod prelude;
pub mod queue;
mod utils;
pub use crate::context::Context;
pub use crate::event::{Event, OEvent, OID};
use std::env;
#[cfg(test)]
mod tests;
pub type Result<T> = std::result::Result<T, failure::Error>;

lazy_static! {
    static ref LOG_DIR: String =
        env::var("OBSERVER_LOGS").unwrap_or_else(|_| "/var/log/".to_string());
}

pub fn check_path() -> String {
    format!("OBSERVER LOGDIR {:?}", LOG_DIR.to_string())
}

#[cfg(test)]
pub mod test_newrelic {
    use ackorelic::{
        newrelic_fn::{
            nr_end_custom_segment, nr_end_transaction, nr_start_custom_segment,
            nr_start_web_transaction,
        },
        App, NewRelicConfig,
    };
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_log_path() {
        println!("LOGDIR {:?}", super::check_path());
    }

    #[test]
    fn new_relic_test() {
        let mut count = 0;
        nr_start_web_transaction("test_transaction");
        while count < 1000 {
            let seg1 = nr_start_custom_segment("db_pool");
            thread::sleep(Duration::from_millis(10));
            nr_end_custom_segment(seg1);
            count += 1;
        }
        println!("Events Completed");
        nr_end_transaction()
    }

}

/*
enum Value {
    // all big query data types
};

type AttachedData = HashMap<String, Value>;

impl From<i32> for Value {
    fn from(v: i32) -> Value {
        Value::Int(v)
    }
}


pub fn attach(cd: mut AttachedData, key: &str, value: Into<Value>) {
    unimplemented!()
}
*/
