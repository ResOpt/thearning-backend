use chrono::{Local, NaiveDateTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assignments::models::Assignment;
use crate::errors::ThearningResult;
use crate::files::models::UploadedFile;
use crate::schema::assignments;
use crate::schema::attachments;
use crate::schema::attachments::attachment_id;
use crate::users::models::User;
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations)]
#[belongs_to(User, foreign_key = "uploader")]
#[belongs_to(Assignment, foreign_key = "assignment_id")]
#[belongs_to(UploadedFile, foreign_key = "file_id")]
#[table_name = "attachments"]
pub struct Attachment {
    pub attachment_id: String,
    pub file_id: String,
    pub assignment_id: Option<String>,
    pub announcement_id: Option<String>,
    pub uploader: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct FillableAttachment<'a> {
    pub file_id: &'a str,
    pub assignment_id: Option<&'a str>,
    pub announcement_id: Option<&'a str>,
    pub uploader: &'a str,
}

impl Attachment {
    pub fn create(
        new_data: FillableAttachment,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let new_attachment = Self {
            attachment_id: generate_random_id().to_string(),
            file_id: new_data.file_id.to_string(),
            assignment_id: match new_data.assignment_id {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            announcement_id: match new_data.announcement_id {
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

    pub fn load_by_assignment_id(assignment_id: &String, conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        Ok(attachments::table.filter(attachments::attachment_id.eq(assignment_id)).load::<Self>(conn)?)
    }

    pub fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(diesel::delete(attachments::table.filter(attachment_id.eq(&self.attachment_id))).get_result(conn)?)
    }
}
