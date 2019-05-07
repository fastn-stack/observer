use crate::context::Context;
use crate::AResult;
use std::time::Instant;

pub trait Event<T> {
    fn map(&self, ctx: &Context, _data: &T) -> AResult<serde_json::Value>;

    fn with<F>(&self, ctx: &Context, cb: F) -> AResult<T>
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let r = cb();
        let start2 = Instant::now();
        let result = self.map(ctx, &r)?;
        println!(
            "result: {:?}\ncb_time: {:?}\nmap_time: {:?}",
            result,
            start2.duration_since(start),
            Instant::now().duration_since(start2)
        );
        Ok(r)
    }
}

#[derive(Debug)]
pub struct OID {
    oid: String,
    okind: String,
}

pub trait OEvent<T> {
    fn map(&self, in_: &Context, data: &T) -> AResult<serde_json::Value>;
    fn oid(&self, data: &T) -> OID;

    fn with<F>(&self, ctx: &Context, cb: F) -> AResult<T>
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

#[cfg(test)]
mod tests {
    use crate::{AResult, Context, Event, OEvent, OID};

    #[derive(Debug)]
    pub struct CreateUser {
        phone: String,
    }

    impl Event<AResult<i32>> for CreateUser {
        fn map(&self, _ctx: &Context, _data: &AResult<i32>) -> AResult<serde_json::Value> {
            Ok(serde_json::Value::Null)
        }
    }
    fn create_user(ctx: &Context, phone: &str) -> AResult<i32> {
        CreateUser {
            phone: phone.to_string(),
        }.with(ctx, || Ok(phone.len() as i32))?
    }

    #[derive(Debug)]
    pub struct CreatePolicy {
        user_id: i32,
    }

    impl OEvent<AResult<i32>> for CreatePolicy {
        fn map(&self, _ctx: &Context, _data: &AResult<i32>) -> AResult<serde_json::Value> {
            Ok(serde_json::Value::Null)
        }

        fn oid(&self, _data: &AResult<i32>) -> OID {
            OID {
                oid: "policy_oid".to_string(),
                okind: "policy_oid".to_string(),
            }
        }
    }

    fn create_policy(ctx: &Context, user_id: i32) -> AResult<i32> {
        CreatePolicy { user_id: 12345 }.with(ctx, || Ok(user_id * 2))?
    }

    #[test]
    fn create_user_test() {
        let ctx = Context {};
        let user = create_user(&ctx, "hello").unwrap();
        assert_eq!(user, 5);
    }

    #[test]
    fn create_policy_test() {
        let ctx = Context {};
        let user = create_policy(&ctx, 2).unwrap();
        assert_eq!(user, 4);
    }
}
