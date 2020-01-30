
# Data Structures and their functionality of Context and frame
-

```rust
    struct Context {
        key: String,
        context_id: String,
        queue: QueueEnum,
        frame: RefCell<Frame>,
    }

    // In Context struct need to change QueueEnum to Box<Queue>


    impl Context {
        fn new(){
            // creates new object of context
        }

        fn finalise(&self) {
            // finalise object of context
        }

        fn start_frame(&self, frame_id: String) -> Frame {
            // start frame
        }

        fn end_frame(&self, frame: Frame, critical: bool, result: String, success: bool) {
            // ending frame
        }

        fn modify_context(&self, new_frame: Frame) {
            // replace frame with new frame
        }

        fn modify_add(&self, new_frame: Frame) {
            // adding new sub_frames
        }

        fn get_key(&self) -> String {
            // return key of context
        }

        fn update_end_ts(&self, end_ts: DateTime<Utc) {
            // update frame end time
        }

        fn get_data(&self) -> String {
            // return serialized context
        }
    }

    struct Frame {
        key: String, // uuid
        frame_id: String, // function name
        breadcrumbs: Option<HashMap<String, serde_json::Value>>,
        start_ts: DateTime<Utc>,
        pub success: Option<bool>,
        pub result: Option<String>, // serde::Value
        pub end_ts: Option<DateTime<Utc>>,
        pub sub_frames: Vec<Frame>,
    }

    impl Frame {
        fn new(id: String) -> Frame {
            //return new frame
        }

        fn get_data(&self) -> serde_json::Value {
            // return serialized frame
        }

        fn get_key(&self) -> String {
            // get id of frame
        }

        fn save(&self, critical: bool, queue: QueueEnum) {
            // save frame into local
        }

        fn save_on_local(&self) {
            // save frame on local
            // this function will create directories according to function
            // observed function names(where observed is used) and save all the frames
            // related to that function
        }
    }
```
