# Observer

** Observer is a library to capture observability of rust servers

## Observer 2.0 in action
To use observer 2.0
1. Add dependency into `Cargo.toml`.
```toml
observer = "0.2.0"
observer_attribute = "0.1.6"
```
2. import these dependency into `lib.rs`. 
```rust
extern crate observer;
#[macro_use]
extern crate observer_attribute;
``` 
3. Define event json file and export events path as EVENTS_PATH.
```shell script
export EVENTS_PATH="<Path of events.json file>"
```

event.json file

```json
{
    "foo__create_temp" : {
      "critical" : true,
      "result_type": "list",
      "fields" : {
        "id" : "string"
      }
    },
    "update_temp" : {
      "critical" : false,
      "result_type": "i32",
      "fields" : {
        "id" : "string"
      }
    }
}
```

4. Use observer into your project

```rust
// Here namespace and with_result are optional parameter.
// If define namespace so in event file needs to define `foo__create_policy`
// If with_result defined so it log error also, if function returns an error.
use observer::prelude::*;
use observer::Result;

pub struct Temp; 

#[observed(with_result)]
pub fn update_temp(id: &str) -> observer::Result<Temp> {
    observe_field("id", id); // Need to tell type of id's value in event.json
    observe_result(2314);  // Need to tell type of result in event.json
    observer::observe_span_log("Message from update temp");
    Ok(Temp)
}

#[observed(namespace = "foo")]
pub fn create_temp(id: &str) -> observer::Result<Temp> {
    observe_field("id", "4839");
    observe_result(&vec![1,2,3,4]);
    update_temp(id)
}

fn main(){
    // define logger
    let logger = observer::backends::logger::Logger::builder()
            .with_path("/tmp/observer.log")
            .with_stdout()
            .build();

    // Initialize observer with logger
    observer::builder(Box::new(logger)).init();
    observer::create_context("main");
    
    // Call your functions
    let _result = create_temp("temp");
    
    // End of the observer.
    observer::end_context();

}
```

In `stdout` it should look like
```text
logger_initialized
context: main [0ms, 2020-01-29 11:10:54.728594 UTC]
    foo__create_temp: 0ms
        @id: "4839"
        @@success: true
        #result: [1,2,3,4]
        update_temp: 0ms
            @id: "temp"
            @@success: true
            #result: 2314
            logs:
               - 0ms: Message from update temp
``` 

In log file it should look the same.
```text
logger_initialized
context: main [0ms, 2020-01-29 11:10:54.728594 UTC]
    foo__create_temp: 0ms
        @id: "4839"
        @@success: true
        #result: [1,2,3,4]
        update_temp: 0ms
            @id: "temp"
            @@success: true
            #result: 2314
            logs:
               - 0ms: Message from update temp
```


## Observer `0.1.*` in action
To use Observer
1. Have to define events file(json file and mandatory).
2. Have to define logs dir else it will take default as `/var/log/`.

Firstly We have to define an events to observe functions. Here Events are nothing
but same as function name and in events we have tell which fields has be save in breadcrumbs. 
And critical means whether to save this function locally or queue. If critical 
It will go directly to queue else Observer will save it local.
 
Here we have defined to events `observer_me` and  `observe_me_too` (same as function name).

```json
{
    "observe_me" : {
        "critical" : true,
        "fields" : {
            "foo" : "String",
            "foo1" : "f32"
        }
    },
    "observe_me_too" : {
        "critical" : false,
        "fields" : {
            "foo1" : "i32"
        }
    }
}
```


```rust
// src/bin/main.rs
use observer::{
    context::{observe_string, observe_i32, observe_f32},
    observe::observe,
    queue::Queue,
};

#[observed] // Need to define only this on top of fn which we want to observe
// Context reference is mandatory to pass in observer function.
// fn should be return Result type. 
fn observe_me(ctx: &Context, other_params: i32)-> Result<String> {
    // in "foo" can store only string value else it will give compile error
    // It will this in breadcrumbs in Frame
    observe_field("foo", "value".to_string());
    // in "foo1" can store only f32 value else it will give compile error
    // It will this in breadcrumbs in Frame
    observe_field("foo1", 32.02);
    some_other_fn_call();
    
    // Observing this fn also and it will become a sub-frame of observe_me
    observe_me_too(ctx);
    Ok("observed") 
}

fn some_other_fn_call() {}

#[observed]
fn observe_me_too(ctx: &Context) -> Result<i32> {
    observe_field("foo1", 32);
    Ok(12)
}


#[derive(Serialize, Debug, Deserialize)]
pub struct DemoQueue {
    pub name: String,
}

#[typetag::serde(name = "Abc")]
impl Queue for DemoQueue {
    // TODO: Will give complete definition of in next version surely
    fn enqueue(&mut self, data: serde_json::Value) {
        println!("Data: {}", data)
    }
}


fn main() {
    let ctx = Context::new(
        "test_context".to_string(), 
        Box::new(DemoQueue{name: "Abrar".to_string()})
    );
    let _result = observe_me(&ctx, 12);
    ctx.finalise();
}
```

We are calling observer_me as first function and observer_me_too inside it.
In case of Context Object Observer will create frame observer_me and observer_me_too.
Because observer_me_too is calling calling inside from observer_me so it will become
sub-frame of observer_me.  

It will create log dir by given path or default(/var/log/) and save context into
log_dir_path/context and events in log_dir_path/observe_me and log_dir_path/observe_me_too 
based of criticality of of an event.

## Context log will look like this
```json
{
  "frame": {
    "breadcrumbs": {},
    "key": "17eb437f-a5e2-4243-8dac-fa636429dcf9",
    "result": null,
    "sub_frames": [
      {
        "breadcrumbs": {
          "foo": 32
        },
        "key": "59471fc8-3391-4619-b341-931658a2296e",
        "result": 12,
        "sub_frames": [
          {
            "breadcrumbs": {
              "foo1": 32.02
            },
            "key": "399c8d43-16fb-4cd3-8273-b2666026f2f0",
            "result": "observed",
            "sub_frames": [],
            "success": true,
            "end_time": "2019-07-06T08:27:20.451786Z",
            "id": "observe_me_too",
            "start_time": "2019-07-06T08:27:20.451642Z"
          }
        ],
        "success": true,
        "end_time": "2019-07-06T08:27:20.452680Z",
        "id": "observe_me",
        "start_time": "2019-07-06T08:27:20.451618Z"
      }
    ],
    "success": null,
    "end_time": "2019-07-06T08:27:20.452683Z",
    "id": "main",
    "start_time": "2019-07-06T08:27:20.451590Z"
  },
  "key": "302a5760-107a-4826-8670-2efd57db27c2",
  "queue": {
    "type": "Abc",
    "value": {
      "name": "Abrar"
    }
  },
  "id": "test_context"
}
```

## Frame/Event log will look like this
```json
{
  "key": "399c8d43-16fb-4cd3-8273-b2666026f2f0",
  "id": "observe_me_too",
  "breadcrumbs": {
    "foo1": 32.02
  },
  "end_time": "2019-07-06T08:27:20.451786Z",
  "result": "observed",
  "start_time": "2019-07-06T08:27:20.451642Z",
  "sub_frames": [],
  "success": true
}
```

#### TO run it
It will take to path from env,
EVENTS_PATH(Mandatory) and LOG_DIR (If not exists so it will take /var/log/)
```bash
EVENTS_PATH="" LOG_DIR="" cargo run --bin main
```
