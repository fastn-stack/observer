extern crate failure;
extern crate serde;
extern crate serde_json;

mod context;
mod event;

pub type AResult<T> = Result<T, failure::Error>;
pub use crate::context::Context;
pub use crate::event::{Event, OEvent, OID};
