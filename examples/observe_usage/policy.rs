extern crate observer;
use chrono::{DateTime, Utc};
use observer::context::Context;
use observer::event::Event;
use observer::AResult;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Policy {
    pub policy_id: String,
    pub policy_start_date: DateTime<Utc>,
    pub policy_end_date: DateTime<Utc>,
    pub policy_holder_id: i32,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SPolicy {
    pub policy_id: String,
    pub policy_holder_id: i32,
}

impl Event<Policy, SPolicy> for Policy {
    fn map(&self, ctx: &Context, _data: &AResult<Policy>) -> AResult<SPolicy> {
        let data = _data.clone().unwrap();
        let sp = SPolicy {
            policy_id: data.policy_id,
            policy_holder_id: data.policy_holder_id,
        };
        Ok(sp)
    }

    fn name(&self) -> String {
        "Policy".to_string()
    }

    fn destination(&self) -> String {
        "policy".to_string()
    }

    fn is_critical(&self) -> bool {
        false
    }
}
