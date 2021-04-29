#[macro_use]
extern crate serde_json;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate failure;
#[allow(unused_imports)]
#[macro_use]
extern crate observer_attribute;
#[macro_use]
extern crate serde_derive;

pub mod backends;
pub mod context;
#[cfg(feature = "mysql")]
pub mod mysql;
pub mod observe;
pub mod observe_fields;
#[cfg(feature = "postgres")]
pub mod pg;
pub mod span;
mod sql_parse;

pub use crate::context::Context;
pub use crate::observe::Observe;
pub use crate::observe_fields::*;
pub use crate::span::{Span, SpanItem};

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub trait Backend: Send + Sync {
    fn app_started(&self) {}
    fn app_ended(&self) {}
    fn context_created(&self, _id: &str) {}
    fn context_ended(&self, _ctx: &crate::Context) {}
    fn span_created(&self, _id: &str) {}
    fn span_data(&self, _key: &str, _value: &str) {}
    fn span_ended(&self, _span: Option<&crate::span::Span>) {}
}

pub struct Observer {
    backends: Vec<Box<dyn Backend>>,
}

lazy_static! {
    static ref OBSERVER: std::sync::Arc<antidote::RwLock<Option<Observer>>> =
        std::sync::Arc::new(antidote::RwLock::new(None));
}

thread_local! {
    static CONTEXT: std::cell::RefCell<Option<Context>> = std::cell::RefCell::new(None);
}

pub fn builder(backend: Box<dyn Backend>) -> Observer {
    Observer::builder(backend)
}

pub fn create_context(context_id: &str) {
    let obj = OBSERVER.as_ref().read();
    if let Some(obj) = obj.as_ref() {
        obj.create_context(context_id);
    }
}

pub fn end_context() -> Option<impl serde::Serialize> {
    let obj = OBSERVER.as_ref().read();
    if let Some(obj) = obj.as_ref() {
        Some(obj.end_context())
    } else {
        None
    }
}

pub fn printed_context() -> Option<String> {
    use backends::logger::print_context;
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            Some(print_context(ctx))
        } else {
            None
        }
    })
}

pub fn shape_hash() -> String {
    use sha2::Digest;

    let trace_without_data = shape_trace().unwrap_or_else(|| "".to_string());
    format!("{:x}", sha2::Sha256::digest(trace_without_data.as_bytes()))
}

pub fn shape_trace() -> Option<String> {
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            Some(ctx.trace_without_data(false))
        } else {
            None
        }
    })
}

pub fn test_trace() -> Option<String> {
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            Some(ctx.trace_without_data(true))
        } else {
            None
        }
    })
}

pub fn trace() -> Option<String> {
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            Some(ctx.trace_without_data(true))
        } else {
            None
        }
    })
}

pub fn log(value: &'static str) {
    let obj = OBSERVER.as_ref().read();
    if let Some(obj) = obj.as_ref() {
        obj.span_log(value);
    }
}

pub(crate) fn start_span(id: &str) {
    let obj = OBSERVER.as_ref().read();
    if let Some(obj) = obj.as_ref() {
        obj.create_span(id);
    }
}

pub(crate) fn end_span(is_critical: bool, err: Option<String>) {
    let obj = OBSERVER.as_ref().read();
    if let Some(obj) = obj.as_ref() {
        obj.end_span(is_critical, err);
    }
}

pub(crate) fn field(key: &'static str, value: serde_json::Value) {
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            ctx.observe_span_field(key, value);
        }
    });
}

pub(crate) fn transient_field(key: &'static str, value: serde_json::Value) {
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            ctx.observe_span_transient_field(key, value);
        }
    });
}

#[allow(dead_code)]
pub(crate) fn observe_query(
    query: String,
    bind: Option<String>,
    result: std::result::Result<usize, String>,
) {
    CONTEXT.with(|context| {
        if let Some(ctx) = context.borrow().as_ref() {
            ctx.observe_query(query, bind, result);
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

        let mut obj = OBSERVER.as_ref().write();
        obj.replace(self);
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
    pub(crate) fn end_context(&self) -> impl serde::Serialize {
        CONTEXT.with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            match ctx.as_ref() {
                Some(ctx) => {
                    ctx.finalise();
                    for backend in self.backends.iter() {
                        backend.context_ended(&ctx);
                    }
                }
                None => {
                    unreachable!("this is bug");
                }
            };
            ctx.take()
        })
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

    pub(crate) fn span_log(&self, value: &'static str) {
        CONTEXT.with(|ctx| {
            if let Some(ctx) = ctx.borrow().as_ref() {
                ctx.span_log(value);
            }
        });
    }
}
