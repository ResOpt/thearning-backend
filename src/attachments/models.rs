use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::attachments;
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations)]
#[table_name = "attachments"]
pub struct Attachment {
    attachment_id: String,
    file_id: String,
    assignment_id: Option<String>,
    uploader: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations)]
#[table_name = "attachments"]
pub struct AttachmentResult {
    attachment_id: String,
    file_id: String,
    assignment_id: String,
    uploader: String,
}

impl Attachment {
    pub fn create(
        file_id: &String,
        assignment_id: &Option<&String>,
        uploader: &String,
        conn: &PgConnection,
    ) -> QueryResult<AttachmentResult> {
        let new_attachment = Self {
            attachment_id: generate_random_id().to_string(),
            file_id: file_id.to_string(),
            assignment_id: match assignment_id {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            uploader: uploader.to_string(),
        };

        diesel::insert_into(attachments::table)
            .values(&new_attachment)
            .execute(conn)?;

        attachments::table
            .find(new_attachment.attachment_id)
            .get_result::<AttachmentResult>(conn)
    }
}
