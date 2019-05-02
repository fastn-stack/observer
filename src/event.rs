use crate::{
    context::{Context, Frame, SFrame},
    AResult,
};
use kafka::{
    error::Error as KafkaError,
    producer::{Producer, Record, RequiredAcks},
};
use serde;
use serde_json;
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
    time::Duration,
    time::Instant,
};

pub trait Event<T, K> {
    fn map(&self, ctx: &Context, _data: &AResult<T>) -> AResult<K>;

    fn name(&self) -> String;

    fn destination(&self) -> String;

    fn is_critical(&self) -> bool {
        false
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
    use crate::event::LOCAL_FILE_SYSTEM_DIRECTORY;
    use crate::observer::observe;
    use crate::{AResult, Context, Event};
    use std::fs::File;
    use std::path::Path;

    #[derive(Debug)]
    pub struct CreateUser {
        phone: String,
    }

    impl Event<CreateUser, CreateUser> for CreateUser {
        fn map(&self, _ctx: &Context, _data: &AResult<CreateUser>) -> AResult<CreateUser> {
            Ok(_data.clone().unwrap())
        }

        fn name(&self) -> String {
            "CreateUser".to_string()
        }

        fn destination(&self) -> String {
            "create_user".to_string()
        }
    }
    fn create_user(ctx: &Context, phone: &str) -> AResult<CreateUser> {
        let user = CreateUser {
            phone: phone.to_string(),
        };

        observe(ctx, user.clone(), || Ok(user))
    }

    #[test]
    fn context_data_test() {
        let ctx = Context::new();
        let uuid = ctx.get_key();
        create_user(&ctx, "8888888888");
        ctx.finalise();

        let s = LOCAL_FILE_SYSTEM_DIRECTORY.to_string();
        let mut file = File::open("foo.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
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
