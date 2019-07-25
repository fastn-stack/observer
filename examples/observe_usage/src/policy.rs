use chrono::prelude::*;
use observer::{context::observe_i32, Context, Result};
use std::{time, thread};
use crate::db_test::db_call;
use observer::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub updated_on: DateTime<Utc>,
}

impl Policy {
    fn get_by_id(id: &str) -> Result<Policy> {
        Ok(Policy {
            id: id.into(),
            name: "".into(),
            updated_on: Utc::now(),
        })
    }

    #[observed(with_result)]
    pub fn create_policy(ctx: &Context, name: &str) -> Result<Policy> {
        // thread::sleep(time::Duration::from_secs(3));
        db_call();
        let policy = Policy {
            id: "1".into(),
            name: name.into(),
            updated_on: Utc::now(),
        };
        observe_field(ctx, "pid", "activa_policy_id");
        observe_result(ctx, 323232);
        let _ = Policy::update_policy(ctx, "policy_id1", "name1");
        let _ = Policy::update_policy1(ctx, "policy_id2", "name2");
        //        let _ = Policy::update_policy2(ctx, "policy_id2", "name2");
        // observed_field!(ctx, "pid", "activa_policy_id".to_string());
        Ok(policy)
    }

    #[observed(without_result)]
    pub fn update_policy(ctx: &Context, pid: &str, _name: &str) -> Result<Policy> {
        // println!("{}", "Coming here");
        // thread::sleep(time::Duration::from_secs(3));
        let p = Policy::get_by_id(pid)?;
        observe_field(ctx, "qid", 4839);
        observe_result_i32(ctx, 5676);
        Ok(p)
    }

    #[observed(without_result)]
    pub fn update_policy1(ctx: &Context, pid: &str, _name: &str) -> Result<Policy> {
        // thread::sleep(time::Duration::from_secs(3));
        let p = Policy::get_by_id(pid)?;
        observe_field(ctx, "qid", 4839);
        observe_result_i32(ctx, 1234);
        Ok(p)
    }

    #[observed(without_result)]
    pub fn update_policy2(ctx: &Context, pid: &str, _name: &str) -> Result<Policy> {
        let p = Policy::get_by_id(pid)?;
        observe_field(ctx, "qid", 4839);
        Ok(p)
    }

    #[observed(without_result)]
    pub fn temp(ctx: &Context){
        observe_result_i32(ctx, 1234);
    }

    /*
    {
        "policy_name_changed": {
            "pid": Option<Value::String>,
        }
    }

    in attribute macro function, before generating the above function we verify all keys are
    present, and we use observe_str etc depending on value in json.

    also for match, if, we have to ensure all branches call obser!("field"), or none of them.

    check_pr will check json file against prod, and if any key eg "policy_name_changed" is
    different between prod and local branch, it will fail.

    handling of failure in closure: since we wnat ? used, if closure returns error, we will convert
    error to stirng and store in column named result.

    in case of success, what should be stored in result column? one option is Result<T>,
    where T: Resulty
    Resulty is trait that convert T to string. for most types we will implement this using the
    Debug trait, for other types like Policy, end user has to convert Policy to string (mostly just
    the string version of policy id).
    */
}
