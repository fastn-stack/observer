#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate failure;

pub mod context;
mod event;
mod frame;
pub mod observe;
pub mod queue;
pub mod resulty;
mod utils;

pub use crate::context::Context;
pub use crate::event::{Event, OEvent, OID};
pub use crate::observe::observe;
use std::env;
pub type Result<T> = std::result::Result<T, failure::Error>;

lazy_static!{
    static ref LOG_DIR: String = env::var("OBSERVER_LOGS")
    .unwrap_or("/var/log/".to_string());
}

pub fn check_path() -> String {
    format!("OBSERVER LOGDIR {:?}", LOG_DIR.to_string())
}


#[cfg(test)]
pub mod tests {
    #[test]
    fn test_log_path(){
        println!("LOGDIR {:?}", super::check_path());
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
