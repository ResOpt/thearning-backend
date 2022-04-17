use chrono::{Local, NaiveDateTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::files;

pub enum FileType {
    MP4,
    MKV,
    JPEG,
    PNG,
    PDF,
    WordDocument,
    ExcelDocument,
}

pub enum UploadType {
    ProfilePhoto,
    ClassPicture,
    AssignmentFile,
}

impl FileType {
    pub fn from_str(filetype: &str) -> Self {
        match filetype {
            "video/mp4" => Self::MP4,
            "video/x-matroska" => Self::MKV,
            "image/jpeg" => Self::JPEG,
            "image/png" => Self::PNG,
            "application/pdf" => Self::PDF,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => Self::WordDocument,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => Self::ExcelDocument,
            _ => unimplemented!()
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Associations, Clone, Queryable)]
#[table_name = "files"]
pub struct UploadedFile {
    pub file_id: String,
    pub filename: String,
    pub file_path: String,
    pub file_url: String,
    pub filetype: String,
    pub created_at: NaiveDateTime,
}

impl UploadedFile {
    pub fn new(file_id: &String, filename: &String, file_path: &String, file_url: &String, filetype: &String, conn: &PgConnection) -> QueryResult<Self> {
        let new_file = Self {
            file_id: file_id.to_string(),
            filename: filename.to_string(),
            file_path: file_path.to_string(),
            file_url: file_url.to_string(),
            filetype: filetype.to_string(),
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(files::table)
            .values(&new_file)
            .execute(conn)?;

        files::table.find(new_file.file_id).get_result::<Self>(conn)
    }

    pub fn receive(file_id: &String, conn: &PgConnection) -> QueryResult<Self> {
        files::table.find(file_id).get_result::<Self>(conn)
    }

    pub fn get_from_url(url: &String, conn: &PgConnection) -> QueryResult<Self> {
        files::table.filter(files::file_url.eq(url)).get_result::<Self>(conn)
    }
}
