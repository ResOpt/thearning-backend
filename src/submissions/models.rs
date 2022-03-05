use chrono::{Date, NaiveDate, NaiveTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::classes::models::Classroom;
use crate::schema::submissions;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "submissions"]
pub struct Submissions {
    pub submission_id: String,
    pub assignment_id: String,
    pub user_id: String,
    pub submitted_date: Option<NaiveDate>,
    pub submitted_time: Option<NaiveTime>,
    pub on_time: bool,
    pub marks_allotted: Option<i32>,
    pub submission_file: Option<String>,
}

pub struct FillableSubmissions {}
