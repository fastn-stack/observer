use crate::context::{Context, Frame, SFrame};
use crate::AResult;
use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};
use serde;
use serde_json;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

pub static LOCAL_FILE_SYSTEM_DIRECTORY: &str = "/Users/venkatesh/observer_files/";

pub static KAFKA_TOPIC: &str = "my-topic";

pub static KAFKA_BROKER: &str = "localhost:9092";

pub trait Event<T, K> {
    fn map(&self, ctx: &Context, _data: &AResult<T>) -> AResult<K>;

    fn name(&self) -> String;

    fn destination(&self) -> String;

    fn is_critical(&self) -> bool {
        false
    }

    fn save(&self, frame: Frame) -> AResult<()> {
        if self.is_critical() {
            //write it to the queue directly
            let data = frame.get_data();
            produce_message(
                data.to_string().as_bytes(),
                KAFKA_TOPIC,
                vec![KAFKA_BROKER.to_string()],
            );
        } else {
            //write it to local file system
            let destination_folder =
                LOCAL_FILE_SYSTEM_DIRECTORY.to_string() + self.destination().as_str();
            if Path::new(destination_folder.as_str()).exists() {
                let mut file =
                    File::create(destination_folder + "/" + frame.get_key().as_str()).unwrap();
                let data = frame.get_data();
                file.write(data.to_string().as_bytes());
            } else {
                create_dir(destination_folder.clone());
                let mut file =
                    File::create(destination_folder + "/" + frame.get_key().as_str()).unwrap();
                let data = frame.get_data();
                file.write(data.to_string().as_bytes());
            }
        }
        Ok(())
    }
}

pub fn produce_message(data: &[u8], topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    producer.send(&Record::from_value(topic, data));

    Ok(())
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
