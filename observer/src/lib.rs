#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate failure;
#[macro_use]
extern crate observer_attribute;

pub mod backends;
pub mod context;
#[cfg(feature = "mysql")]
pub mod mysql;
pub mod observe;
pub mod observe_fields;
pub mod observer_newrelic;
#[cfg(feature = "postgres")]
pub mod pg;
pub mod prelude;
mod span;
mod sql_parse;
pub use crate::context::Context;
#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;
pub type Result<T> = std::result::Result<T, failure::Error>;

pub trait Backend {
    fn app_started(&self);
    fn app_ended(&self);
    fn context_created(&self, id: &str);
    fn context_ended(&self, ctx: &crate::Context);
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

pub fn builder(backend: Box<dyn Backend>) -> Observer {
    Observer::builder(backend)
}

//pub fn create_observer(backends: Vec<Box<dyn Backend>>) {
//    OBSERVER.with(|observer| {
//        let mut observer = observer.borrow_mut();
//        observer.replace(Observer::new(backends))
//    });
//}

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

    pub fn builder(backend: Box<dyn Backend>) -> Self {
        Observer {
            backends: vec![backend],
            context: std::cell::RefCell::new(Box::new(None)),
        }
    }

    pub fn add_backend(mut self, backend: Box<dyn Backend>) -> Self {
        self.backends.push(backend);
        self
    }

    pub fn init(self) {
        for backend in self.backends.iter() {
            backend.app_started()
        }

        OBSERVER.with(|observer| {
            let mut observer = observer.borrow_mut();
            observer.replace(self)
        });
    }

    //    pub fn new(backends: Vec<Box<dyn Backend>>) -> Self {
    //        for backend in backends.iter() {
    //            backend.app_started()
    //        }
    //        Observer {
    //            backends,
    //            context: std::cell::RefCell::new(Box::new(None)),
    //            log_path: None,
    //            stdout: false,
    //        }
    //    }

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
            let _ = ctx.finalise(true, true);
            for backend in self.backends.iter() {
                backend.context_ended(&ctx);
            }
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
