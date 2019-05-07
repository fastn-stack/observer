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

    pub fn change_name(ctx: &Context, pid: &str, name: &str) -> Result<Policy> {
        observe(ctx, || {
            let mut p = Policy::get_by_id(pid)?;
            let old_name = p.name.clone();
            p.name = name.into();

            Ok((
                p,
                NameChanged {
                    pid: pid.into(),
                    name: name.into(),
                    old_name,
                },
            ))
        })
    }
}

#[derive(Serialize)]
struct NameChanged {
    pid: String,
    name: String,
    old_name: String,
}
