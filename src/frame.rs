use chrono::prelude::*;

#[derive(Serialize, Debug)]
pub struct Frame {
    key: String,
    frame_id: String,
    breadcrumbs: Option<serde_json::Value>,
    start_ts: DateTime<Utc>,
    end_ts: Option<DateTime<Utc>>,
    sub_frames: Vec<Frame>,
}

impl Frame {
    pub fn new(frame_id: String) -> Frame {
        Frame {
            key: uuid::Uuid::new_v4().to_string(),
            frame_id,
            breadcrumbs: None,
            start_ts: chrono::Utc::now(),
            end_ts: None,
            sub_frames: vec![],
        }
    }

    pub fn get_data(&self) -> serde_json::value::Value {
        json!({
            "f_key" : self.key,
            "frame_id" : self.frame_id,
            "breadcrumbs" : self.breadcrumbs,
            "start_ts" : self.start_ts,
            "end_ts" : self.end_ts
        })
    }

    pub fn get_key(&self) -> String {
        // self.clone().key
        unimplemented!()
    }
}
