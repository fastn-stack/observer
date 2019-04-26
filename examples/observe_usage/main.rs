extern crate observer;
extern crate uuid;
mod policy;
mod user_id;

use crate::policy::Policy;
use crate::user_id::UserId;
use chrono::offset::TimeZone;
use chrono::Utc;
use observer::context::Context;
use observer::observer::observe;
use observer::AResult;

fn main() {
    let ctx = Context::new();
    observer_test(&ctx);
    policy_create(&ctx, String::from("2"), 200);
    ctx.finalise();
}

fn observer_test(ctx: &Context) -> AResult<UserId> {
    let user_id = UserId { id: 1 };
    observe(ctx, user_id.clone(), || {
        policy_create(ctx, String::from("1"), 100);
        Ok(user_id)
    })
}

fn policy_create(ctx: &Context, p_id: String, ph_id: i32) -> AResult<Policy> {
    let policy = Policy {
        policy_id: p_id,
        policy_start_date: Utc.ymd(2019, 04, 25).and_hms(0, 0, 0),
        policy_end_date: Utc.ymd(2020, 04, 25).and_hms(0, 0, 0),
        policy_holder_id: ph_id,
        updated_on: Utc::now(),
        created_on: Utc::now(),
    };
    observe(ctx, policy.clone(), || Ok(policy))
}
