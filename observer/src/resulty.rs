// TODO: Rough work

//pub trait Resulty
//where
//    Self: std::fmt::Debug,
//{
//    fn to_string(&self) -> String {
//        format!("{:?}", &self)
//    }
//}

//pub trait Observable {
//    fn observe(&self) -> serde_json::Value;
//}
//
////impl<T> Observable for T where T: serde::Serialize {
////    fn observe(&self)->  serde_json::Value {
////        json!(self)
////    }
////}
//
//impl<T, E> Observable for Result<T, E>
//    where T: Observable, E: Observable
//{
//    fn observe(&self) -> serde_json::Value {
//        match &self {
//            Ok(v) => json!({"ok": v.observe()}),
//            Err(e) => json!({"err": e.observe()}),
//        }
//    }
//}
//
//#[derive(Serialize)]
//pub struct A {
//    id: i32
//}
//
//impl Observable for A {
//    fn observe(&self) -> serde_json::Value {
//        json!("")
//    }
//}



//#[observed]
//pub fn nth_prime(in_: In, n: i32) -> i32 {
//    observe_field!(in_.ctx, "n", n);
//    let p = {
//        0
//    };
//    observe_result!(in_.ctx, p)
//    // observe_i32_without_result()
//}

//#[observed_with_result]
//pub fn create_user(in_: In, username: &str, age: i32) -> Result<User, String> {
//    observe_field!(in_.ctx, "username", username);
//    // events.json contains:
//    // username: "string"
//    // observe_string_field(in_.ctx, "username", username);
//    observe_field!(in_.ctx, "age", age);
//    // age: "i32"
//    // observe_int_field(in_.ctx, "age", age);
//
//    observe_result!(in_.ctx, insert_user(username, age), |u| u.kind.to_string())
//    // observed_result: "i32"
//
//    store_age(age)?;
//
//    let user = observe_i32_result(in_.ctx, insert_user(username, age), |u| (u.id, u.foo)?;
//    //
//    user
//}
//
//fn observe_i32_result<T, E, F>(ctx: Context, r: Result<T, E>, f: F) -> Result<T, E>
//where F: FnOnce(&T) -> i32
//{
//
//}
//
//fn observe_i32_without_result<T, E, F>(ctx: Context, r: i32) -> i32
//{
//
//}

//pub fn atoi(ctx: Context, a: &str) -> i32 {
//    let x = || {
//        Ok(54)
//    }();
//    ctx.update(json!(x.observe()));
//    x
//}

//pub trait Resulty<T, E> {
//    fn o_success_i32<F>(self, ctx: &crate::Context, f: F) -> Result<T, E>
//    where F: FnOnce(T) -> i32;
//}
//
//
//impl<T, E> Resulty<T, E> for Result<T, E> {
//
//    fn o_success_i32<F>(self, ctx: &crate::Context, f: F) -> Result<T, E>
//    where
//        F: FnOnce(&T) -> i32
//    {
//        match self {
//            Ok(t) => {
//                let t1 = f(&t);
//                Ok(t)
//            },
//            Err(err) => {
//                Err(err)
//            }
//        }
//    }
//}

//pub trait ResultyI32 {
//    fn o_success_i32(&self, ctx: &crate::Context, f: F) -> Result<E, T>
//        where F: FnOnce(T) -> i32;
//}
//


//impl Resulty for Result<E, T>
//where E: std::fmt::Debug,
//{
//    fn o_success_i32(&self, ctx: &crate::Context, f: F) -> Result<E, T>
//        where F: FnOnce(T) -> i32
//    {
//        match self {
//            Ok(t) => {
//                ctx.observe_success(f(t))
//            },
//            Err(e) => {
//                ctx.observe_failure(e)
//            }
//        }
//    }
//}
