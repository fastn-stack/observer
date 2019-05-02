extern crate observer;

use observer::{context::Context, event::Event, AResult};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserId {
    pub id: i32,
}

impl Event<UserId, UserId> for UserId {
    fn map(&self, ctx: &Context, _data: &AResult<UserId>) -> AResult<UserId> {
        Ok(UserId {
            id: _data.clone().unwrap().id,
        })
    }

    fn name(&self) -> String {
        "UserId".to_string()
    }

    fn destination(&self) -> String {
        "user_id".to_string()
    }
}
