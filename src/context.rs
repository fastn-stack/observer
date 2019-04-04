use crate::event::Event;
use crate::AResult;
use chrono::Utc;
use std::thread;
use serde::Serialize;

pub struct Context {
    pub id: i32
}

pub fn observe<F,T>(ctx: &Context, event: T,closure: F) -> AResult<T>
where F: FnOnce() -> AResult<T>, T: std::fmt::Debug + Event<T> + serde::Serialize + std::clone::Clone{
    let start_ts = Utc::now();
    thread::sleep_ms(1000);
    let result = closure()?;

    println!("{:?}",result);

    let data_to_be_stroed = serde_json::to_value(event.map(ctx,result.clone()));

    println!("{:?}",data_to_be_stroed);

    let end_ts = Utc::now();
    println!("{}",end_ts-start_ts);
    Ok(result)

}


//fn main() {
//    let a = Context{
//        id: 20
//    };
//    observe(&a, &a, ||{
//        let b = Context{
//            id: 40
//        };
//        Ok(b)
//    });
//}