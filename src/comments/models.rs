use crate::errors::ThearningResult;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{comments, private_comments};
use crate::traits::Manipulable;
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "comments"]
pub struct Comment {
    pub id: String,
    pub user_id: String,
    pub assignment_id: Option<String>,
    pub announcement_id: Option<String>,
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "private_comments"]
pub struct PrivateComment {
    pub id: String,
    pub user_id: String,
    pub submission_id: Option<String>,
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct FillableComment {
    pub user_id: Option<String>,
    pub assignment_id: Option<String>,
    pub announcement_id: Option<String>,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct FillablePrivateComment {
    pub user_id: Option<String>,
    pub submission_id: Option<String>,
    pub body: String,
}

impl Comment {
    pub fn find_comment(id: &String, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(comments::table.find(id).get_result::<Self>(conn)?)
    }

    pub fn load_by_assignment(
        assignment_id: &String,
        conn: &PgConnection,
    ) -> ThearningResult<Vec<Self>> {
        Ok(comments::table
            .filter(comments::assignment_id.eq(assignment_id))
            .load::<Comment>(conn)?)
    }
}

impl PrivateComment {
    pub fn find_comment(id: &String, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(private_comments::table.find(id).get_result::<Self>(conn)?)
    }

    pub fn load_by_submission(
        submission_id: &String,
        conn: &PgConnection,
    ) -> ThearningResult<Vec<Self>> {
        Ok(private_comments::table
            .filter(private_comments::submission_id.eq(submission_id))
            .load::<PrivateComment>(conn)?)
    }
}

impl Manipulable<FillableComment> for Comment {
    fn create(new_data: FillableComment, conn: &PgConnection) -> ThearningResult<Self> {
        let new_comment = Comment {
            id: format!("{}{}", generate_random_id(), generate_random_id()),
            user_id: new_data.user_id.unwrap(),
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
        Ok(diesel::delete(comments::table.find(&self.id)).get_result::<Self>(conn)?)
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
        Ok(diesel::delete(private_comments::table.find(&self.id)).get_result::<Self>(conn)?)
    }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        todo!()
    }
}

pub trait Commenter {
    type Output;
    fn get_user_id(&self) -> &Self::Output;
}

impl Commenter for Comment {
    type Output = String;

    fn get_user_id(&self) -> &Self::Output {
        &self.user_id
    }
}
impl Commenter for PrivateComment {
    type Output = String;

    fn get_user_id(&self) -> &Self::Output {
        &self.user_id
    }
}