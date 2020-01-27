use crate::{context::Context, Result};
use std::time::Instant;

pub trait Event<T, K> {
    fn map(&self, ctx: &Context, data: &Result<(T, K)>) -> Result<serde_json::Value>;
    fn is_critical(&self) -> bool {
        false
    }
}

impl<T, K> Event<T, K> for T
where
    K: serde::Serialize,
{
    fn map(&self, _ctx: &Context, data: &Result<(T, K)>) -> Result<serde_json::Value> {
        match data {
            Ok((_data, event)) => Ok(serde_json::to_value(event)?),
            Err(e) => Ok(serde_json::Value::String(format!("{}", e))),
        }
    }
}

#[derive(Debug)]
pub struct OID {
    oid: String,
    okind: String,
}

pub trait OEvent<T> {
    fn map(&self, in_: &Context, data: &T) -> Result<serde_json::Value>;
    fn oid(&self, data: &T) -> OID;

    fn with<F>(&self, ctx: &Context, cb: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let r = cb();
        let start2 = Instant::now();
        let result = self.map(ctx, &r)?;
        println!(
            "result: {:?}\ncb_time: {:?}\nmap_time: {:?}\n oid: {:?}",
            result,
            start2.duration_since(start),
            Instant::now().duration_since(start2),
            self.oid(&r),
        );
        Ok(r)
    }
}

//
//#[cfg(test)]
//mod tests {
//    use crate::{
//        event::OEvent, event::OID, observer::observe, queue::QueueEnum, Context, Event, Result,
//    };
//    use chrono::Utc;
//    use serde_derive::Serialize;
//    use std::fs::File;
//    use std::path::Path;
//
//    #[derive(Debug, Clone, Serialize)]
//    pub struct CreateUser {
//        phone: String,
//    }
//
//
//        fn destination(&self) -> String {
//            "create_user".to_string()
//        }
//    }
//    fn create_user(ctx: &Context, phone: &str) -> Result<CreateUser> {
//        let user = CreateUser {
//            phone: phone.to_string(),
//        };
//
//        observe(ctx, user.clone(), || Ok(user))
//    }
//
//    #[test]
//    fn context_data_test() {
//        let ctx = Context::new(String::from("test_context"), QueueEnum::Kafka);
//        let uuid = ctx.get_key();
//        create_user(&ctx, "8888888888");
//        ctx.update_end_ts(Utc::now());
//
//        let data = ctx.get_data();
//        let context: Context = serde_json::from_str(data.as_str()).unwrap();
//
//        assert_eq!(context.get_key(), uuid);
//        assert_eq!(context.get_queue(), QueueEnum::Kafka);
//        assert_eq!(context, ctx.clone());
//    }
//
//    #[derive(Debug)]
//    pub struct CreatePolicy {
//        user_id: i32,
//    }
//
//    impl OEvent<Result<i32>> for CreatePolicy {
//        fn map(&self, _ctx: &Context, _data: &Result<i32>) -> Result<serde_json::Value> {
//            Ok(serde_json::Value::Null)
//        }
//
//        fn oid(&self, _data: &Result<i32>) -> OID {
//            OID {
//                oid: "policy_oid".to_string(),
//                okind: "policy_oid".to_string(),
//            }
//        }
//    }
//
//    fn create_policy(ctx: &Context, user_id: i32) -> Result<i32> {
//        CreatePolicy { user_id: 12345 }.with(ctx, || Ok(user_id * 2))?
//    }
//}
