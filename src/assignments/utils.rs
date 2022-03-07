use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;

use crate::schema::assignments;

pub fn get_ids(connection: &PgConnection) -> Result<Vec<String>, Error> {
    assignments::table
        .select(assignments::assignment_id)
        .load::<String>(connection)
}
