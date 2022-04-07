use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};

use crate::schema::files;

pub enum FileType {
    Video,
    Image,
    Document,
    PDF,
}

pub enum UploadType {
    ProfilePhoto,
    ClassPicture,
    AssignmentFile,
}

impl FileType {
    pub fn from_str(filetype: &str) -> Self {
        match filetype {
            "video" => Self::Video,
            "image" => Self::Image,
            "document" => Self::Document,
            "pdf" => Self::PDF,
            _ => unimplemented!()
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Clone)]
#[table_name = "files"]
pub struct UploadedFile {
    pub file_id: String,
    pub filename: String,
    pub file_path: String,
    pub file_url: String,
    pub filetype: String,
}

impl UploadedFile {
    pub fn new(file_id: &String, filename: &String, file_path: &String, file_url: &String, filetype: &String, conn: &PgConnection) -> QueryResult<Self> {
        let new_file = Self {
            file_id: file_id.to_string(),
            filename: filename.to_string(),
            file_path: file_path.to_string(),
            file_url: file_url.to_string(),
            filetype: filetype.to_string(),
        };

        diesel::insert_into(files::table)
            .values(&new_file)
            .execute(conn)?;

        files::table.find(new_file.file_id).get_result::<Self>(conn)
    }

    pub fn receive(file_id: &String, conn: &PgConnection) -> QueryResult<Self> {
        files::table.find(file_id).get_result::<Self>(conn)
    }
}
