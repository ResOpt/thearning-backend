use crate::schema::classes;
use crate::classes::utils::*;

use std::path::Path;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde::{Serialize,Deserialize};

use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "classes"]
pub struct Classroom {
    pub class_id: String,
    pub class_name: String,
    pub section: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "classes"]
pub struct NewClassroom {
    pub class_name: String,
    pub section: String,
}

impl Classroom {
    pub fn create(class: Self, connection: &PgConnection) -> QueryResult<Self> {
        let codes = get_class_codes(connection)?;
        let generate_code = generate_class_code(&codes);
        let new_class = Self {
            class_id: generate_code,
            ..class
        };
        diesel::insert_into(classes::table)
            .values(&new_class)
            .execute(connection)?;

        classes::table.order(classes::class_id.desc()).first(connection)
    }
}