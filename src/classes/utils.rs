use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;
use diesel::Connection;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::schema::classes;

pub fn get_class_codes(connection: &PgConnection) -> Result<Vec<String>, Error> {
    classes::table
        .select(classes::class_id)
        .load::<String>(connection)
}

pub fn generate_class_code(existing_codes: &Vec<String>) -> String {
    let code = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>();

    if existing_codes.contains(&code) {
        generate_class_code(existing_codes)
    } else {
        code
    }
}
