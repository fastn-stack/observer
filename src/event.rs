use serde;
use serde_json;
use std::time::Instant;

pub type AResult<T> = Result<T, ()>;
pub struct Context {}

pub trait Event<T> {
    fn map(&self, context_: &Context, _data: &T) -> AResult<serde_json::Value>;

    fn with<F>(&self, context_: &Context, cb: F) -> AResult<T>
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let r = cb();
        let start2 = Instant::now();
        let result = self.map(context_, &r)?;
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
pub struct CreateUser {
    phone: String,
}

impl<T> Event<T> for CreateUser {
    fn map(&self, _context_: &Context, _data: &T) -> AResult<serde_json::Value> {
        Ok(serde_json::Value::Null)
    }
}

fn create_user(context_: &Context, phone: &str) -> AResult<i32> {
    CreateUser {
        phone: phone.to_string(),
    }
    .with(context_, || Ok(phone.len() as i32))?
}

#[cfg(test)]
mod tests {
    use super::{create_user, Context};

    #[test]
    fn create_user_test() {
        let context_ = Context {};
        let user = create_user(&context_, "hello").unwrap();
        assert_eq!(user, 5);
    }
}
