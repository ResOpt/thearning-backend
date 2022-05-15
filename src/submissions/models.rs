use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::assignments::models::Assignment;
use crate::errors::ThearningResult;
use crate::schema::submissions;
use crate::traits::Manipulable;
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations)]
#[belongs_to(Assignment)]
#[table_name = "submissions"]
pub struct Submissions {
    pub submission_id: String,
    pub assignment_id: String,
    pub user_id: String,
    pub submitted_date: Option<NaiveDate>,
    pub submitted_time: Option<NaiveTime>,
    pub on_time: Option<bool>,
    pub marks_allotted: Option<i32>,
    pub submitted: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Mark {
    marks: Option<i32>,
}

#[derive(Clone)]
pub struct FillableSubmissions {
    pub assignment_id: String,
    pub user_id: String,
}

impl Submissions {
    pub fn get_by_id(
        assignment_id: &String,
        user_id: &String,
        conn: &PgConnection,
    ) -> ThearningResult<Self> {
        Ok(submissions::table
            .filter(submissions::user_id.eq(user_id))
            .filter(submissions::assignment_id.eq(assignment_id))
            .get_result::<Self>(conn)?)
    }

    pub fn find_submission(submission_id: &String, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(submissions::table
            .find(submission_id)
            .get_result::<Self>(conn)?)
    }

    pub fn get_by_assignment(assignment_id: &String, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(submissions::table
            .filter(submissions::assignment_id.eq(assignment_id))
            .get_result::<Self>(conn)?)
    }

    pub fn unsubmit(&self, conn: &PgConnection) -> ThearningResult<Self> {
        diesel::update(
            submissions::table.filter(submissions::submission_id.eq(&self.submission_id)),
        )
        .set(submissions::submitted.eq(false))
        .execute(conn)?;

        Ok(submissions::dsl::submissions
            .find(&self.submission_id)
            .get_result::<Self>(conn)?)
    }

    pub fn submit(&self, conn: &PgConnection) -> ThearningResult<Self> {
        let assignment = Assignment::get_by_id(&self.assignment_id, conn)?;

        let now = Local::now().naive_local();

        let now_date = Local::today().naive_local();

        let now_time = now.time();

        let submitted = NaiveDateTime::new(now_date, now_time);

        let due = match (assignment.due_date, assignment.due_time) {
            (Some(a), Some(b)) => Some(NaiveDateTime::new(a, b)),
            (Some(a), None) => Some(NaiveDateTime::new(a, NaiveTime::from_hms(23, 59, 59))),
            (None, Some(b)) => Some(NaiveDateTime::new(Local::today().naive_local(), b)),
            (None, None) => None,
        };

        let on_time = match due {
            Some(d) => {
                if submitted < d {
                    Some(true)
                } else {
                    Some(false)
                }
            }
            None => None,
        };

        diesel::update(
            submissions::table.filter(submissions::submission_id.eq(&self.submission_id)),
        )
        .set((
            submissions::submitted.eq(!self.submitted),
            submissions::submitted_date.eq(&now_date),
            submissions::submitted_time.eq(&now_time),
            submissions::on_time.eq(on_time),
        ))
        .execute(conn)?;

        Ok(submissions::dsl::submissions
            .find(&self.submission_id)
            .get_result::<Self>(conn)?)
    }
}

impl Manipulable<FillableSubmissions> for Submissions {
    fn create(new_data: FillableSubmissions, conn: &PgConnection) -> ThearningResult<Self> {

        let assignment = Assignment::get_by_id(&new_data.assignment_id, conn)?;

        let now = Local::now().naive_local();

        let now_date = Local::today().naive_local();

        let now_time = now.time();

        let submitted = NaiveDateTime::new(now_date, now_time);

        let due = match (assignment.due_date, assignment.due_time) {
            (Some(a), Some(b)) => Some(NaiveDateTime::new(a, b)),
            (Some(a), None) => Some(NaiveDateTime::new(a, NaiveTime::from_hms(23, 59, 59))),
            (None, Some(b)) => Some(NaiveDateTime::new(Local::today().naive_local(), b)),
            (None, None) => None,
        };

        let on_time = match due {
            Some(d) => {
                if submitted < d {
                    Some(true)
                } else {
                    Some(false)
                }
            }
            None => None,
        };
        
        let submission = Submissions {
            submission_id: format!("{}{}", generate_random_id(), generate_random_id()),
            assignment_id: new_data.assignment_id,
            user_id: new_data.user_id,
            submitted_date: Some(Local::today().naive_local()),
            submitted_time: Some(Local::now().naive_local().time()),
            on_time,
            marks_allotted: None,
            submitted: false,
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(submissions::table)
            .values(&submission)
            .execute(conn)?;

        let res = submissions::table
            .find(submission.submission_id)
            .get_result::<Self>(conn)?;

        Ok(res)
    }

    fn update(&self, update: FillableSubmissions, conn: &PgConnection) -> ThearningResult<Self> {
        todo!()
    }

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        todo!()
    }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        todo!()
    }
}
