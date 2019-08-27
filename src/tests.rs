// Mainly load testing based
use crate::{context, queue};
use ackorelic::newrelic_fn::{nr_end_custom_segment, nr_start_custom_segment};
use std::thread;
use std::time::Duration;
use threadpool;

fn create_segment() {
    let mut count = 0;
    let pool = threadpool::ThreadPool::new(5);
    for _ in 0..1000 {
        count += 1;
        pool.execute(move || {
            let segment = nr_start_custom_segment("segment_id");
            thread::sleep(Duration::from_secs(2));
            nr_end_custom_segment(segment);
        });
    }
    thread::sleep(Duration::from_secs(2));
    println!("Count :: {}", count);
}

#[test]
fn load_test_create_context() {
    for _ in 0..10 {
        context::create_context(
            "test_context".to_string(),
            Box::new(queue::DemoQueue {
                name: "api_testing".to_string(),
            }),
        );
        create_segment();
        context::end_context();
    }
}
