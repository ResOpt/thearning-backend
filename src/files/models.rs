use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::schema::files;

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
    pub fn new(filename: &str, filetype: &str) -> QueryResult<Self> {
        unimplemented!()
    }
}
