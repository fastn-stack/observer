use observer::prelude::*;
use observer::Result;


#[observed(namespace="fn_test")]
pub fn b() -> i32 {
    observe_field("name", "b");
    observe_field("age", 30);
    std::thread::sleep(std::time::Duration::from_secs(1));
    1
}

#[observed(namespace="fn_test")]
pub fn a() -> i32{
    b();
    observe_field("name", "a");
    observe_field("age", 28);
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Hello World");
    b()
}