use chrono::{Local, NaiveDateTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use std::fmt;

use crate::{errors::ErrorKind, schema::files, traits::Embedable};

pub enum FileType {
    MP4,
    MKV,
    JPEG,
    PNG,
    PDF,
    Text,
    RAR,
    ZIP,
    WordDocument,
    ExcelDocument,
}

pub enum UploadType {
    ProfilePhoto,
    ClassPicture,
    AssignmentFile,
}

impl FileType {
    pub fn from_str(filetype: &str) -> Result<Self, ErrorKind> {
        match filetype {
            "video/mp4" => Ok(Self::MP4),
            "video/x-matroska" => Ok(Self::MKV),
            "image/jpeg" => Ok(Self::JPEG),
            "image/png" => Ok(Self::PNG),
            "application/pdf" => Ok(Self::PDF),
            "text/plain" => Ok(Self::Text),
            "application/vnd.rar" => Ok(Self::RAR),
            "application/zip" => Ok(Self::ZIP),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                Ok(Self::WordDocument)
            }
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
                Ok(Self::ExcelDocument)
            }
            _ => Err(ErrorKind::InvalidValue),
        }
    }

    pub fn ext(&self) -> &'static str {
        match &self {
            Self::MP4 => "mp4",
            Self::MKV => "mkv",
            Self::JPEG => "jpeg",
            Self::PNG => "png",
            Self::PDF => "pdf",
            Self::Text => "txt",
            Self::RAR => "rar",
            Self::ZIP => "zip",
            Self::WordDocument => "docx",
            Self::ExcelDocument => "xlsx",
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filetype = match &self {
            Self::MP4 => "mp4",
            Self::MKV => "matroshka",
            Self::JPEG => "jpeg",
            Self::PNG => "png",
            Self::PDF => "pdf",
            Self::Text => "text",
            Self::RAR => "rar",
            Self::ZIP => "zip",
            Self::WordDocument => "docx",
            Self::ExcelDocument => "xlsx",
        };

        write!(f, "{}", filetype)
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
    pub fn new(
        file_id: &String,
        filename: &String,
        file_path: &String,
        file_url: &String,
        filetype: &String,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
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
        files::table
            .filter(files::file_url.eq(url))
            .get_result::<Self>(conn)
    }
}

impl Embedable for UploadedFile {}
