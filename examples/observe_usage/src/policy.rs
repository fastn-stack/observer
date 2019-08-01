use chrono::prelude::*;
use observer::{Result};
use std::{time, thread};
use crate::db_test::db_call;
use observer::prelude::*;
use std::collections::HashMap;

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
    pub fn create_policy(name: &str) -> Result<Policy> {
        thread::sleep(time::Duration::from_secs(3));
        db_call();
        let policy = Policy {
            id: "1".into(),
            name: name.into(),
            updated_on: Utc::now(),
        };
        observe_field("pid", "activa_policy_id");
        let mut hm = HashMap::new();
        hm.insert("sds", 1);
        let t: Vec<String> = Vec::new();
        observe_result(&t);
        let _ = Policy::update_policy("policy_id1", "name1");
        let _ = Policy::update_policy1("policy_id2", "name2");
        let _ = Policy::update_policy2("policy_id2", "name2");
        observe_field("pid", "activa_policy_id");
        Ok(policy)
    }

    #[observed(without_result)]
    pub fn update_policy(pid: &str, _name: &str) -> Result<Policy> {
        thread::sleep(time::Duration::from_secs(3));
        let p = Policy::get_by_id(pid)?;
        observe_field("qid", 4839);
        observe_result(1234);
        Ok(p)
    }

    #[observed(without_result)]
    pub fn update_policy1(pid: &str, _name: &str) -> Result<Policy> {
        thread::sleep(time::Duration::from_secs(3));
        let p = Policy::get_by_id(pid)?;
        observe_field("qid", 4839);
        observe_result(2314);
        Ok(p)
    }

    #[observed(without_result)]
    pub fn update_policy2(pid: &str, _name: &str) -> Result<Policy> {
        let p = Policy::get_by_id(pid)?;
        observe_result(2314);
        observe_field("qid", 4839);
        Ok(p)
    }

    #[observed(without_result)]
    pub fn temp(){
        observe_result_i32(1234);
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
