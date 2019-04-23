extern crate observer;
use chrono::Utc;
use core::borrow::{Borrow, BorrowMut};
use observer::context::{Context, Frame};
use observer::event::Event;
use observer::observer::{observe, save};
use observer::AResult;
use serde::ser::Serialize;
use serde_derive::{Deserialize, Serialize};
use std::collections::LinkedList;
use std::thread;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserId {
    id: i32,
}

impl Event<UserId> for UserId {
    fn map(&self, ctx: &RefCell<Context>, _data: UserId) -> AResult<UserId> {
        Ok(_data)
    }

    fn name(&self) -> String {
        "UserId".to_string()
    }

    fn destination(&self) -> String {
        "user_id".to_string()
    }
}

fn print_user_id() {
    let mut s = 25;
    let a = UserId { id: 25 };
}

//fn main1() {
//    let a = Context {
//        id: 20,
//        ctx: "initial".to_string()
//    };
//    let c =
//        UserId {
//            id: 25,
//        };
//
//    observe(&a, c.clone(), ||{
//        Ok(c)
//    });
//
////    println!("{:?}", c);
//}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bar {
    val: i32,
}

impl Event<Bar> for Bar {
    fn map(&self, ctx: &RefCell<Context>, _data: Bar) -> AResult<Bar> {
        Ok(_data)
    }

    fn name(&self) -> String {
        "Bar".to_string()
    }

    fn destination(&self) -> String {
        "bar".to_string()
    }

    fn is_critical(&self) -> bool {
        true
    }
}

#[derive(Debug)]
struct Foo<'a> {
    val: i32,
    bar: Bar,
    val_ref: &'a mut i32,
}

use std::cell::RefCell;
use std::fs::read_to_string;
extern crate uuid;
use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::new_v4();
    println!("{}", my_uuid);
    let frame = Frame {
        key: my_uuid.to_string(),
        frame_id: "main".to_string(),
        breadcrumbs: None,
        start_ts: Utc::now(),
        end_ts: None,
        sub_frames: vec![],
    };
    let ctx = RefCell::new(Context {
        context_id: "test_context".to_string(),
        frame,
        key: my_uuid.to_string(),
    });
    observer_test(&ctx);
    print_bar(&ctx);
    ctx.borrow_mut().frame.end_ts = Some(Utc::now());
    println!("{:?}", ctx);
    save(&ctx);
}

fn observer_test(ctx: &RefCell<Context>) {
    let user_id = UserId { id: 1 };
    observe(ctx, user_id.clone(), || {
        println!("hello_world");
        print_bar(ctx);
        Ok(user_id)
    });
}

fn print_bar(ctx: &RefCell<Context>) {
    let bar = Bar { val: 20 };
    observe(ctx, bar.clone(), || {
        print!("this is in bar");
        Ok(bar)
    });
}
