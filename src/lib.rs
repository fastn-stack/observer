#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod context;
mod event;
mod frame;
mod observe;
mod queue;

pub use crate::context::Context;
pub use crate::event::{Event, OEvent, OID};
pub use crate::observe::observe;

pub type Result<T> = std::result::Result<T, failure::Error>;
