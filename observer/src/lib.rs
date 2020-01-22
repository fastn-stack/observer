#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate failure;
#[macro_use]
extern crate observer_attribute;

pub mod context;
mod event;
mod span;
//#[cfg(feature = "mysql")]
//pub mod mysql;
pub mod observe;
pub mod observe_fields;
pub mod observer_newrelic;
#[cfg(feature = "postgres")]
pub mod pg;
pub mod prelude;
mod sql_parse;
mod utils;
pub use crate::context::Context;
pub use crate::event::{Event, OEvent, OID};
use std::env;

#[cfg(test)]
mod tests;
pub type Result<T> = std::result::Result<T, failure::Error>;

pub trait Backend {
    fn app_started(&self);
    fn app_ended(&self);
    fn context_created(&self, id: &str);
    fn context_ended(&self);
    fn span_created(&self, id: &str);
    fn span_data(&self, key: &str, value: &str);
    fn span_ended(&self);
}

pub struct Observer {
    backends: Vec<Box<dyn Backend>>,
    context: std::cell::RefCell<Box<Option<crate::Context>>>,
}

thread_local! {
    static OBSERVER: std::cell::RefCell<Option<Observer>> = std::cell::RefCell::new(None);
}

pub fn create_observer(backends: Vec<Box<dyn Backend>>) {
    OBSERVER.with(|observer| {
        let mut observer = observer.borrow_mut();
        observer.replace(Observer::new(backends))
    });
}

pub fn create_context(context_id: &str) {
    OBSERVER.with(|observer| {
        if let Some(obj) = observer.borrow().as_ref() {
            obj.create_context(context_id);
        }
    });
}

pub fn end_context() {
    OBSERVER.with(|observer| {
        if let Some(obj) = observer.borrow().as_ref() {
            obj.end_context();
        }
        let mut observer = observer.borrow_mut();
        observer.take();
    });
}

pub fn observe_span_log(value: &str) {
    OBSERVER.with(|observer| {
        if let Some(obj) = observer.borrow().as_ref() {
            obj.span_log(value);
        }
    });
}

pub(crate) fn start_span(id: &str) {
    OBSERVER.with(|observer| {
        if let Some(obj) = observer.borrow().as_ref() {
            obj.create_span(id);
        }
    });
}

pub(crate) fn end_span(is_critical: bool, err: Option<String>) {
    OBSERVER.with(|observer| {
        if let Some(obj) = observer.borrow().as_ref() {
            obj.end_span(is_critical, err);
        }
    });
}

//pub(crate) fn end_ctx_frame() {
//    OBSERVER.with(|observer| {
//        if let Some(obj) = observer.borrow().as_ref() {
//            if let Some(ctx) = obj.context.borrow().as_ref() {
//                ctx.end_ctx_frame();
//            }
//        }
//    });
//}

pub(crate) fn observe_field(key: &str, value: serde_json::Value) {
    OBSERVER.with(|observer| {
        if let Some(obj) = observer.borrow().as_ref() {
            if let Some(ctx) = obj.context.borrow().as_ref() {
                ctx.observe_span_field(key, value)
            }
        }
    });
}

pub(crate) fn observe_result(result: impl serde::Serialize) {
    OBSERVER.with(|observer| {
        if let Some(obj) = observer.borrow().as_ref() {
            if let Some(ctx) = obj.context.borrow().as_ref() {
                ctx.observe_span_result(result);
            }
        }
    });
}

pub(crate) fn observe_span_id(id: &str) {
    OBSERVER.with(|observer| {
        if let Some(obj) = observer.borrow().as_ref() {
            if let Some(ctx) = obj.context.borrow().as_ref() {
                ctx.observe_span_id(id);
            }
        }
    });
}

impl Observer {
    /// Initialized Observer with different backends(NewRelic, StatsD, Sentry, Jaeger, etc...)
    /// and call their app started method
    pub fn new(backends: Vec<Box<dyn Backend>>) -> Self {
        for backend in backends.iter() {
            backend.app_started()
        }
        Observer {
            backends,
            context: std::cell::RefCell::new(Box::new(None)),
        }
    }
    /// It will iterate through all backends and call their context_created method.
    pub(crate) fn create_context(&self, context_id: &str) {
        let mut context = self.context.borrow_mut();
        if context.is_none() {
            context.replace(crate::context::Context::new(context_id.to_string()));
            for backend in self.backends.iter() {
                backend.context_created(context_id);
            }
        }
    }

    /// It will end context object and drop things if needed.
    pub(crate) fn end_context(&self) {
        if let Some(ctx) = self.context.borrow().as_ref() {
            let _ = ctx.finalise();
        }
        for backend in self.backends.iter() {
            backend.context_ended();
        }
    }

    pub(crate) fn create_span(&self, id: &str) {
        if let Some(ctx) = self.context.borrow().as_ref() {
            ctx.start_span(id);
        }
        for backend in self.backends.iter() {
            backend.span_created(id);
        }
    }
    pub(crate) fn end_span(&self, is_critical: bool, err: Option<String>) {
        if let Some(ctx) = self.context.borrow().as_ref() {
            ctx.end_span(is_critical, err);
        }
        for backend in self.backends.iter() {
            backend.span_ended();
        }
    }

    pub(crate) fn span_log(&self, value: &str) {
        if let Some(ctx) = self.context.borrow().as_ref() {
            ctx.span_log(value);
        }
    }
}

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
