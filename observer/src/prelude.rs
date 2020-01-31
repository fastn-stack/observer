pub use crate::observe::Observe;
pub use crate::observe_fields::*;
pub use crate::Result as ObserverResult;

pub fn observe_field<T>(_key: &str, _v: T) {}
