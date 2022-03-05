use std::path::Path;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::classes::utils::*;
use crate::schema::classes;

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
    pub fn create(class: NewClassroom, connection: &PgConnection) -> QueryResult<Self> {
        let codes = get_class_codes(connection)?;
        let generate_code = generate_class_code(&codes);
        let new_class = Self {
            class_id: generate_code,
            class_name: class.class_name,
            section: class.section,
        };
        diesel::insert_into(classes::table)
            .values(&new_class)
            .execute(connection)?;

        classes::table
            .find(new_class.class_id)
            .get_result::<Self>(connection)
    }
}
