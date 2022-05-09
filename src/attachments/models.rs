use std::fs;

use std::ops::Deref;
use chrono::{Local, NaiveDateTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assignments::models::Assignment;
use crate::errors::ThearningResult;
use crate::files::models::UploadedFile;
use crate::schema::assignments;
use crate::schema::{attachments, files, links};
use crate::schema::attachments::attachment_id;
use crate::users::models::User;
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations, Clone)]
#[belongs_to(User, foreign_key = "uploader")]
#[belongs_to(Assignment, foreign_key = "assignment_id")]
#[belongs_to(UploadedFile, foreign_key = "file_id")]
#[table_name = "attachments"]
pub struct Attachment {
    pub attachment_id: String,
    pub file_id: Option<String>,
    pub link_id: Option<String>,
    pub assignment_id: Option<String>,
    pub announcement_id: Option<String>,
    pub submission_id: Option<String>,
    pub uploader: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct FillableAttachment<'a> {
    pub file_id: Option<String>,
    pub link_id: Option<String>,
    pub assignment_id: Option<&'a str>,
    pub announcement_id: Option<&'a str>,
    pub submission_id: Option<&'a str>,
    pub uploader: &'a str,
}

impl Attachment {
    pub fn create(
        new_data: FillableAttachment,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let new_attachment = Self {
            attachment_id: generate_random_id().to_string(),
            file_id: new_data.file_id,
            link_id: new_data.link_id,
            assignment_id: match new_data.assignment_id {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            announcement_id: match new_data.announcement_id {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            submission_id: match new_data.submission_id {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            uploader: new_data.uploader.to_string(),
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(attachments::table)
            .values(&new_attachment)
            .execute(conn)?;

        attachments::table
            .find(new_attachment.attachment_id)
            .get_result::<Self>(conn)
    }

    pub fn find(id: &str, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(attachments::table.find(id).get_result::<Self>(conn)?)
    }

    pub fn load_by_assignment_id(assignment_id: &String, conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        Ok(attachments::table.filter(attachments::assignment_id.eq(assignment_id)).load::<Self>(conn)?)
    }

    pub fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        match &self.file_id {
          Some(id) => {
            let file = UploadedFile::receive(id, conn)?;
            fs::remove_file(file.file_path)?;
            diesel::delete(files::table.filter(files::file_id.eq(id))).execute(conn);
        }  
          None => {
              ()
          }
        }
        match &self.link_id {
            Some(id) => {
                diesel::delete(links::table.filter(links::id.eq(id.trim()))).execute(conn);
            }
            None => {
                ()
            }
        }       
        
        Ok(diesel::delete(attachments::table.filter(attachment_id.eq(&self.attachment_id))).get_result(conn)?)
    }
}
