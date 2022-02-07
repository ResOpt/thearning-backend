use chrono::{Date, NaiveDate, NaiveTime};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::{Serialize, Deserialize};

use crate::classes::models::Classroom;
use crate::schema::assignments;
use crate::assignments::utils::{generate_random_id, get_ids};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "assignments"]
pub struct Assignments {
    pub assignment_id: String,
    pub assignment_name: String,
    pub class_id: String,
    pub due_date: Option<NaiveDate>,
    pub due_time: Option<NaiveTime>,
    pub posted_date: NaiveDate,
    pub instructions: Option<String>,
    pub total_marks: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "assignments"]
pub struct FillableAssignments {
    pub assignment_name: String,
    pub class_id: String,
    pub due_date: Option<NaiveDate>,
    pub due_time: Option<NaiveTime>,
    pub instructions: Option<String>,
}

impl Assignments {
    pub fn create(assignment_data: FillableAssignments, connection: &PgConnection) -> QueryResult<Self> {
        let assignments = Self {
            assignment_id: generate_random_id(&get_ids(connection).unwrap()),
            assignment_name: assignment_data.assignment_name,
            class_id: assignment_data.class_id,
            due_date: match assignment_data.due_date {
                Some(v) => {
                    Some(NaiveDate::from(v))
                }
                None => None
            },
            due_time: match assignment_data.due_time {
                Some(v) => {
                    Some(NaiveTime::from(v))
                }
                None => None
            },
            posted_date: NaiveDate::from(chrono::offset::Local::now().date().naive_local()),
            instructions: assignment_data.instructions,
            total_marks: Some(100),
        };

        diesel::insert_into(assignments::table)
            .values(&assignments)
            .execute(connection)?;

        assignments::table.order(assignments::assignment_id.desc()).first(connection)

    }
}