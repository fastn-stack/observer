extern crate failure;
extern crate serde;
extern crate serde_json;

use serde::ser::{Serialize, Serializer};

pub mod context;
pub mod event;

pub type AResult<T> = Result<T, OError>;

#[derive(Debug)]
pub struct OError{
    pub error: failure::Error
}

impl Serialize for OError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(self.into())
    }
}
pub use crate::context::Context;
pub use crate::event::{Event, OEvent, OID};
