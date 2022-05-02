use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::assignments::models::Assignment;
use crate::schema::submissions;

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations)]
#[belongs_to(Assignment)]
#[table_name = "submissions"]
pub struct Submissions {
    pub submission_id: String,
    pub assignment_id: String,
    pub user_id: String,
    pub submitted_date: Option<NaiveDate>,
    pub submitted_time: Option<NaiveTime>,
    pub on_time: bool,
    pub marks_allotted: Option<i32>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct FillableSubmissions {
    pub assignment_id: String,
    pub user_id: String,
}
