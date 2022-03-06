use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::db::DbConn;
use crate::files::utils::get_file_ids;
use crate::schema::files;
use crate::utils::generate_random_id;

pub enum FileType {
    Video,
    Image,
    Document,
    PDF,
}

impl FileType {
    pub fn from_str(filetype: &str) -> Self {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations)]
#[table_name = "files"]
pub struct UploadedFile {
    file_id: String,
    filename: String,
    filetype: String,
}

impl UploadedFile {
    pub fn new(filename: &str, filetype: &str, conn: &PgConnection) -> QueryResult<Self> {
        let new_file = Self {
            file_id: generate_random_id().to_string(),
            filename: filename.to_string(),
            filetype: filetype.to_string(),
        };

        diesel::insert_into(files::table)
            .values(&new_file)
            .execute(conn)?;

        files::table.find(new_file.file_id).get_result::<Self>(conn)
    }
}
