use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::errors::ThearningResult;

use crate::schema::{comments, private_comments};
use crate::traits::Manipulable;
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "comments"]
pub struct Comment {
    id: String,
    user_id: String,
    assignment_id: Option<String>,
    announcement_id: Option<String>,
    body: String,
    created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "private_comments"]
pub struct PrivateComment {
    id: String,
    user_id: String,
    submission_id: Option<String>,
    body: String,
    created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct FillableComment {
    user_id: String,
    assignment_id: Option<String>,
    announcement_id: Option<String>,
    body: String,
}

#[derive(Serialize, Deserialize)]
pub struct FillablePrivateComment {
    user_id: Option<String>,
    submission_id: Option<String>,
    body: String,
}

impl Manipulable<FillableComment> for Comment {
    fn create(new_data: FillableComment, conn: &PgConnection) -> ThearningResult<Self> {

        let new_comment = Comment {
            id: format!("{}{}", generate_random_id(), generate_random_id()),
            user_id: new_data.user_id,
            assignment_id: new_data.assignment_id,
            announcement_id: new_data.announcement_id,
            body: new_data.body,
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(comments::table)
        .values(&new_comment)
        .execute(conn)?;

        let res = comments::table
            .find(new_comment.id)
            .get_result::<Self>(conn)?;

        Ok(res)
    }

    fn update(&self, update: FillableComment, conn: &PgConnection) -> ThearningResult<Self> {
        todo!()
    }

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(diesel::delete(comments::table.find(&self.id))
            .get_result::<Self>(conn)?)   
        }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        todo!()
    }
}

impl Manipulable<FillablePrivateComment> for PrivateComment {
    fn create(new_data: FillablePrivateComment, conn: &PgConnection) -> ThearningResult<Self> {

        let new_comment = Self {
            id: format!("{}{}", generate_random_id(), generate_random_id()),
            user_id: new_data.user_id.unwrap(),
            submission_id: new_data.submission_id,
            body: new_data.body,
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(private_comments::table)
        .values(&new_comment)
        .execute(conn)?;

        let res = private_comments::table
            .find(new_comment.id)
            .get_result::<Self>(conn)?;

        Ok(res)
    }

    fn update(&self, update: FillablePrivateComment, conn: &PgConnection) -> ThearningResult<Self> {
        todo!()
    }

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(diesel::delete(private_comments::table.find(&self.id))
            .get_result::<Self>(conn)?)   
    }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        todo!()
    }
}