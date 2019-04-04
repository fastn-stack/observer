extern crate observer;
use observer::context::{observe,Context};
use observer::event::Event;
use observer::AResult;
use chrono::Utc;
use std::thread;
use serde_derive::{Serialize ,Deserialize};
use serde::ser::Serialize;


#[derive(Debug,Serialize,Deserialize,Clone,Copy)]
pub struct UserId {
    id: i32
}

impl Event<UserId> for UserId {
    fn map(&self, ctx: &Context, _data: UserId) -> AResult<UserId>
    {
        Ok(_data)
    }
}

fn print_user_id(){
    let a = UserId{
        id: 10,
    };
}

fn main() {
    let a = Context{
        id: 20
    };
    let c =
    UserId {
        id: 20
    };

    observe(&a, c, ||{
        Ok(c)
    });
}