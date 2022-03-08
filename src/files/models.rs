use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Clone)]
#[table_name = "files"]
pub struct UploadedFile {
    pub file_id: String,
    pub filename: String,
    pub filetype: String,
}

impl UploadedFile {
    pub fn new(filename: &String, filetype: &String, conn: &PgConnection) -> QueryResult<Self> {
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
