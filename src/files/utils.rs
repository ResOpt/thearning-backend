use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;
use diesel::Connection;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::schema::files;

pub fn get_file_ids(connection: &PgConnection) -> Result<Vec<String>, Error> {
    files::table
        .select(files::file_id)
        .load::<String>(connection)
}
