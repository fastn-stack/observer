use crate::tables::{users_skill::dsl, users_skill::dsl::users_skill};
use diesel::{connection::Connection, prelude::*};

use observer::pg::OConnection;
use observer::prelude::*;

#[derive(Queryable, Debug)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub allocation_logic: String,
}

#[observed]
pub fn db_call() {
    let database_url = "postgres://root@127.0.0.1/acko";
    let nr_conn = OConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let _nr_result: Vec<Skill> = users_skill
        .filter(dsl::id.gt(20))
        .load::<Skill>(&nr_conn)
        .expect("Error loading skills");
    // println!("Result {:?}", _nr_result);
}
