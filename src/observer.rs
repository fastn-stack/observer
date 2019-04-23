use crate::context::Frame;
use crate::context::Context;
use crate::event::Event;
use crate::AResult;
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use std::cell::RefCell;
use std::{collections::LinkedList, thread, thread_local};
use std::fs::{create_dir, File};
use std::path::Path;
use uuid::Uuid;
use std::io::Write;

pub static LOCAL_FILE_SYSTEM_DIRECTORY: &str = "/Users/venkatesh/observer_files/";

pub static KAFKA_TOPIC: &str = "my-topic";

pub static KAFKA_BROKER: &str = "localhost:9092";

pub fn observe<F, T>(ctx: &RefCell<Context>, event: T, closure: F) -> AResult<T>
where
    F: FnOnce() -> AResult<T>,
    T: std::fmt::Debug + Event<T> + serde::Serialize + std::clone::Clone,
{
    let start_ts = Utc::now();
    let new_frame = Frame {
        key: Uuid::new_v4().to_string(),
        frame_id: event.name(),
        breadcrumbs: None,
        start_ts,
        end_ts: None,
        sub_frames: vec![],
    };

    let temp1 = ctx.clone().into_inner().frame;
    Context::modify_context(ctx, new_frame);

    let result = closure()?;

    println!("{:?}", result);

    let end_ts = Utc::now();
    ctx.borrow_mut().frame.end_ts = Some(end_ts);

    println!("printing context for the current observe \n{:?}", ctx);
    let data_to_be_stroed = serde_json::to_value(event.map(ctx, result.clone())).unwrap();
    ctx.borrow_mut().frame.breadcrumbs = Some(data_to_be_stroed);

    let temp2 = ctx.clone().into_inner().frame;

    if event.is_critical() {
        //write it to the queue directly
        let data = json!({
            "f_key" : temp2.key,
            "frame_id" : temp2.frame_id,
            "breadcrumbs" : temp2.breadcrumbs,
            "start_ts" : temp2.start_ts,
            "end_ts" : temp2.end_ts
            });
        produce_message(format!("{:?}",data).as_bytes(),KAFKA_TOPIC,vec![KAFKA_BROKER.to_string()]);
    }else{
        //write it to local file system
        let destination_folder = LOCAL_FILE_SYSTEM_DIRECTORY.to_string()+ event.destination().as_str();
        if Path::new(destination_folder.as_str()).exists() {
            let mut file = File::create(destination_folder+"/"+temp2.key.as_str()).unwrap();
            let data = json!({
            "f_key" : temp2.key,
            "frame_id" : temp2.frame_id,
            "breadcrumbs" : temp2.breadcrumbs,
            "start_ts" : temp2.start_ts,
            "end_ts" : temp2.end_ts
            });
            file.write(format!("{:?}",data).as_bytes());
        }else {
            create_dir(destination_folder.clone());
            let mut file = File::create(destination_folder+"/"+temp2.key.as_str()).unwrap();
            let data = json!({
            "f_key" : temp2.key,
            "frame_id" : temp2.frame_id,
            "breadcrumbs" : temp2.breadcrumbs,
            "start_ts" : temp2.start_ts,
            "end_ts" : temp2.end_ts
            });
            file.write(format!("{:?}",data).as_bytes());
        }
    }

    Context::modify_context(ctx, temp1);
    Context::modify_add(ctx, temp2);

    Ok(result)
}


pub fn save(ctx: &RefCell<Context>) {
    let destination_folder = LOCAL_FILE_SYSTEM_DIRECTORY.to_string()+ "context/";
    if Path::new(destination_folder.as_str()).exists() {
            let mut file = File::create(destination_folder+"/"+ctx.borrow_mut().key.as_str()).unwrap();
            file.write(format!("{:?}",ctx.clone().into_inner()).as_bytes());
        }else {
            create_dir(destination_folder.clone());
            let mut file = File::create(destination_folder+"/"+ctx.borrow_mut().key.as_str()).unwrap();
            file.write(format!("{:?}",ctx.clone().into_inner()).as_bytes());
        }
}

use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;

pub fn produce_message(
    data: &[u8],
    topic: &str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    let mut producer = Producer::from_hosts(brokers)
             .with_ack_timeout(Duration::from_secs(1))
             .with_required_acks(RequiredAcks::One)
             .create()?;

//    producer.send(&Record {
//        topic,
//        partition: -1,
//        key: (),
//        value: data,
//    });

    producer.send(&Record::from_value(topic, data));

    Ok(())
}