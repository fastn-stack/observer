#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate failure;
#[allow(unused_imports)]
#[macro_use]
extern crate observer_attribute;

pub mod backends;
pub mod context;
#[cfg(feature = "mysql")]
pub mod mysql;
pub mod observe;
pub mod observe_fields;
#[cfg(feature = "postgres")]
pub mod pg;
pub mod prelude;
pub mod span;
mod sql_parse;

pub use crate::context::Context;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub trait Backend: Send + Sync {
    fn app_started(&self);
    fn app_ended(&self);
    fn context_created(&self, id: &str);
    fn context_ended(&self, ctx: &crate::Context);
    fn span_created(&self, id: &str);
    fn span_data(&self, key: &str, value: &str);
    fn span_ended(&self, span: Option<&crate::span::Span>);
}

pub struct Observer {
    backends: Vec<Box<dyn Backend>>,
}

lazy_static! {
    static ref OBSERVER: std::sync::Arc<std::sync::RwLock<Option<Observer>>> =
        std::sync::Arc::new(std::sync::RwLock::new(None));
}

thread_local! {
    static CONTEXT: std::cell::RefCell<Option<Context>> = std::cell::RefCell::new(None);
}

pub fn builder(backend: Box<dyn Backend>) -> Observer {
    Observer::builder(backend)
}

pub fn create_context(context_id: &str) {
    match OBSERVER.as_ref().read() {
        Ok(obj) => {
            if let Some(obj) = obj.as_ref() {
                obj.create_context(context_id);
            }
        }
        Err(_err) => {}
    };
}

pub fn end_context() {
    match OBSERVER.as_ref().read() {
        Ok(obj) => {
            if let Some(obj) = obj.as_ref() {
                obj.end_context();
            }
        }
        Err(_err) => {}
    };
}

pub fn observe_span_log(value: &str) {
    match OBSERVER.as_ref().read() {
        Ok(obj) => {
            if let Some(obj) = obj.as_ref() {
                obj.span_log(value);
            }
        }
        Err(_err) => {}
    };
}

pub(crate) fn start_span(id: &str) {
    match OBSERVER.as_ref().read() {
        Ok(obj) => {
            if let Some(obj) = obj.as_ref() {
                obj.create_span(id);
            }
        }
        Err(_err) => {}
    };
}

pub(crate) fn end_span(is_critical: bool, err: Option<String>) {
    match OBSERVER.as_ref().read() {
        Ok(obj) => {
            if let Some(obj) = obj.as_ref() {
                obj.end_span(is_critical, err);
            }
        }
        Err(_err) => {}
    };
}

pub(crate) fn observe_field(key: &str, value: serde_json::Value) {
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            ctx.observe_span_field(key, value);
        }
    });
}

pub(crate) fn observe_result(result: impl serde::Serialize) {
    CONTEXT.with(|ctx| {
        if let Some(ctx) = ctx.borrow().as_ref() {
            ctx.observe_span_result(result);
        }
    });
}

#[allow(dead_code)]
pub fn observe_span_id(id: &str) {
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            ctx.observe_span_id(id);
        }
    });
}

impl Observer {
    /// Initialized Observer with different backends(NewRelic, StatsD, Sentry, Jaeger, etc...)
    /// and call their app started method

    pub fn builder(backend: Box<dyn Backend>) -> Self {
        Observer {
            backends: vec![backend],
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

        match OBSERVER.write() {
            Ok(mut obj) => {
                obj.replace(self);
            }
            Err(_e) => {}
        };
    }

    /// It will iterate through all backends and call their context_created method.
    pub(crate) fn create_context(&self, context_id: &str) {
        CONTEXT.with(|obj| {
            let mut context = obj.borrow_mut();
            if context.is_none() {
                context.replace(Context::new(context_id.to_string()));
            }
            for backend in self.backends.iter() {
                backend.context_created(context_id);
            }
        });
    }

    /// It will end context object and drop things if needed.
    pub(crate) fn end_context(&self) {
        CONTEXT.with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            if let Some(ctx) = ctx.as_ref() {
                ctx.finalise();
                for backend in self.backends.iter() {
                    backend.context_ended(&ctx);
                }
            }
            ctx.take();
        });
    }

    pub(crate) fn create_span(&self, id: &str) {
        CONTEXT.with(|ctx| {
            if let Some(ctx) = ctx.borrow().as_ref() {
                ctx.start_span(id);
                for backend in self.backends.iter() {
                    backend.span_created(id);
                }
            }
        });
    }

    pub(crate) fn end_span(&self, is_critical: bool, err: Option<String>) {
        CONTEXT.with(|ctx| {
            if let Some(ctx) = ctx.borrow().as_ref() {
                ctx.end_span(is_critical, err);
                for backend in self.backends.iter() {
                    backend.span_ended(ctx.span_stack.borrow().last());
                }
            }
        });
    }

    pub(crate) fn span_log(&self, value: &str) {
        CONTEXT.with(|ctx| {
            if let Some(ctx) = ctx.borrow().as_ref() {
                ctx.span_log(value);
            }
        });
    }
}
