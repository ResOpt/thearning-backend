use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::assignments::models::Assignment;
use crate::errors::ThearningResult;
use crate::schema::{marks, submissions};
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

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Clone, Queryable)]
#[table_name = "marks"]
pub struct Mark {
    pub id: String,
    pub submission_id: Option<String>,
    pub marker_id: Option<String>,
    pub student_id: Option<String>,
    pub value: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct FillableMark {
    pub submission_id: Option<String>,
    pub marker_id: Option<String>,
    pub student_id: Option<String>,
    pub value: i32,
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

    pub fn mark(&self, value: &i32, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(diesel::update(submissions::table.find(&self.submission_id))
            .set(submissions::marks_allotted.eq(value))
            .get_result::<Self>(conn)?)
    }

    pub fn find_submission(submission_id: &String, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(submissions::table
            .find(submission_id)
            .get_result::<Self>(conn)?)
    }

    pub fn load_unsubmitted(assignment_id: &String, conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        Ok(submissions::table
            .filter(submissions::assignment_id.eq(assignment_id))
            .filter(submissions::submitted.eq(false))
            .load::<Self>(conn)?)
    }

    pub fn get_by_assignment(assignment_id: &String, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(submissions::table
            .filter(submissions::assignment_id.eq(assignment_id))
            .get_result::<Self>(conn)?)
    }

    pub fn load_by_assignment(assignment_id: &String, conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        Ok(submissions::table
            .filter(submissions::assignment_id.eq(assignment_id))
            .load::<Self>(conn)?)
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

    pub fn update_on_time(&self, assignment: &Assignment, conn: &PgConnection) -> ThearningResult<Self> {

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
        .set(submissions::on_time.eq(on_time))
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

impl Mark {
    pub fn get_by_id(id: &str, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(marks::table
            .find(id)
            .get_result::<Self>(conn)?)
    }

    pub fn get_by_submission_id(
        submission_id: &String,
        conn: &PgConnection,
    ) -> ThearningResult<Self> {
        Ok(marks::table
            .filter(marks::submission_id.eq(Some(submission_id)))
            .get_result::<Self>(conn)?)
    }
}

impl Manipulable<FillableMark> for Mark {
    fn create(new_data: FillableMark, conn: &PgConnection) -> ThearningResult<Self> {
        let submission = Submissions::find_submission(&new_data.submission_id.as_ref().unwrap(), conn)?;

        let mark = Mark {
            id: format!("{}{}", generate_random_id(), generate_random_id()),
            submission_id: new_data.submission_id,
            marker_id: new_data.marker_id,
            student_id: Option::from(submission.user_id),
            value: new_data.value,
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(marks::table)
            .values(&mark)
            .execute(conn)?;

        let res = marks::table
            .find(mark.id)
            .get_result::<Self>(conn)?;

        Ok(res)
    }

    fn update(&self, update: FillableMark, conn: &PgConnection) -> ThearningResult<Self> {

        let mark = Mark {
            id: self.id.to_string(),
            submission_id: self.submission_id.clone(),
            marker_id: update.marker_id.clone(),
            student_id: self.student_id.clone(),
            value: update.value,
            created_at: self.created_at,
        };

        diesel::update(marks::table.find(&mark.id))
            .set(&mark)
            .execute(conn)?;

        Ok(marks::table.find(&mark.id).get_result::<Self>(conn)?)
    }

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        todo!()
    }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        todo!()
    }
}
