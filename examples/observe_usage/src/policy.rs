use chrono::prelude::*;
use observer::{observe, Context, Result};

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

    pub fn create_policy(ctx: &Context, name: &str) -> Result<Policy> {
        observe(ctx, || {
            let policy = Policy {
                id: "".into(),
                name: name.into(),
                updated_on: Utc::now(),
            };
            Ok((policy.clone(), policy))
        })
    }

    #[observed("policy_name_changed:critical")]
    pub fn change_name(ctx: &Context, pid: &str, name: &str) -> Result<Policy> {
        let mut p = Policy::get_by_id(pid)?;

        observe!(ctx, "pid", pid);
        observe!(ctx, "old_name", &p.name);

        p.name = name.into();

        observe_success!(ctx, p.id); // or observe_failure!(ctx, error_message)
        Ok(p)
    }

    pub fn change_name(ctx: &Context, pid: &str, name: &str) -> Result<Policy> {
        observe(ctx, "policy_name_changed::critical", || {
            let mut p = Policy::get_by_id(pid)?;

            ctx.observe_i32("pid", pid);
            ctx.observe_str("old_name", &p.name);

            p.name = name.into();

            Ok(p)
        }
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
