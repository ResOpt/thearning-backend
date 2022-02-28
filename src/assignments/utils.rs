use diesel::Connection;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::schema::{assignments, classes};

pub fn get_ids(connection: &PgConnection) -> Result<Vec<String>, Error> {
    assignments::table.select(assignments::assignment_id).load::<String>(connection)
}

pub fn generate_random_id(existing_codes: &Vec<String>) -> String {
    let code = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect::<String>();

    if existing_codes.contains(&code) {
        generate_random_id(existing_codes)
    } else {
        code
    }
}