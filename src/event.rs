use crate::context::{Context, Span};
use crate::AResult;
use chrono::prelude::*;
use chrono::DateTime;
use serde;
use serde_json;
use std::time::Instant;

pub trait Event<T> {
    fn map(&self, ctx: &Context, _data: &T) -> AResult<serde_json::Value>;
    fn ekind(&self) -> String;

    fn with<F>(&self, ctx: &mut Context, cb: F) -> AResult<T>
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let start_time: DateTime<Utc> = Utc::now();
        let r = cb();
        let start2 = Instant::now();
        let start_time2: DateTime<Utc> = Utc::now();
        let result = self.map(ctx, &r)?;
        println!(
            "result: {:?}\ncb_time: {:?}\nmap_time: {:?}",
            result,
            start2.duration_since(start),
            Instant::now().duration_since(start2)
        );
        ctx.add_span(Span {
            start_time,
            end_time: start_time2,
            ekind: self.ekind(),
            delta: start_time2.signed_duration_since(start_time),
        });
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
    fn ekind(&self) -> String;

    fn with<F>(&mut self, ctx: &mut Context, cb: F) -> AResult<T>
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let start_time: DateTime<Utc> = Utc::now();
        let r = cb();
        let start2 = Instant::now();
        let start_time2: DateTime<Utc> = Utc::now();
        let result = self.map(ctx, &r)?;
        println!(
            "result: {:?}\ncb_time: {:?}\nmap_time: {:?}\n oid: {:?}",
            result,
            start2.duration_since(start),
            Instant::now().duration_since(start2),
            self.oid(&r),
        );
        ctx.add_span(Span {
            start_time,
            end_time: start_time2,
            ekind: self.ekind(),
            delta: start_time2.signed_duration_since(start_time),
        });
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

        fn ekind(&self) -> String {
            "create_user".to_string()
        }
    }

    fn create_user(ctx: &mut Context, phone: &str) -> AResult<i32> {
        CreateUser {
            phone: phone.to_string(),
        }
        .with(ctx, || Ok(phone.len() as i32))?
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

        fn ekind(&self) -> String {
            "create_policy".to_string()
        }
    }

    fn create_policy(ctx: &mut Context, user_id: i32) -> AResult<i32> {
        CreatePolicy { user_id: 12345 }.with(ctx, || Ok(user_id * 2))?
    }

    #[test]
    fn create_user_test() {
        let mut ctx = Context::default();
        let user = create_user(&mut ctx, "hello").unwrap();
        assert_eq!(user, 5);
        println!("User {:?}", ctx);
    }

    #[test]
    fn create_policy_test() {
        let mut ctx = Context::default();
        let user = create_policy(&mut ctx, 2).unwrap();
        assert_eq!(user, 4);
        println!("Policy {:?}", ctx);
    }
}
