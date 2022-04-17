use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::assignments;
use crate::traits::Manipulable;
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "assignments"]
pub struct Assignment {
    pub assignment_id: String,
    pub assignment_name: Option<String>,
    pub class_id: Option<String>,
    pub topic_id: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub due_time: Option<NaiveTime>,
    pub posted_date: NaiveDate,
    pub instructions: Option<String>,
    pub total_marks: Option<i32>,
    pub created_at: NaiveDateTime,
    pub draft: bool,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "assignments"]
pub struct FillableAssignments {
    pub assignment_name: Option<String>,
    pub class_id: Option<String>,
    pub topic_id: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub due_time: Option<NaiveTime>,
    pub instructions: Option<String>,
}

impl Assignment {
    pub fn get_by_id(id: &String, conn: &PgConnection) -> QueryResult<Self> {
        assignments::table.find(id).get_result::<Self>(conn)
    }

    pub fn draft(&self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(assignments::table)
            .values(&*self)
            .execute(conn)?;

        assignments::table.find(&*self.assignment_id).get_result::<Self>(conn)
    }
}

impl Default for Assignment {
    fn default() -> Self {
        Self {
            assignment_id: generate_random_id().to_string(),
            assignment_name: None,
            class_id: None,
            topic_id: None,
            due_date: None,
            due_time: None,
            posted_date: NaiveDate::from(chrono::offset::Local::now().date().naive_local()),
            instructions: None,
            total_marks: None,
            created_at: Local::now().naive_local(),
            draft: true
        }
    }
}

impl Manipulable<FillableAssignments> for Assignment {
    fn create(new_data: FillableAssignments, conn: &PgConnection) -> QueryResult<Self> {
        unimplemented!()
    }

    fn update(&self, update: FillableAssignments, conn: &PgConnection) -> QueryResult<Self> {
        diesel::update(assignments::table.filter(assignments::assignment_id.eq(&self.assignment_id)))
            .set((assignments::assignment_name.eq(&update.assignment_name),
            assignments::due_date.eq(&update.due_date),
            assignments::due_time.eq(&update.due_time),
            assignments::topic_id.eq(&update.topic_id),
            assignments::class_id.eq(&update.class_id),
            assignments::instructions.eq(&update.instructions),
            assignments::draft.eq(false)))
            .execute(conn)?;

        assignments::dsl::assignments.find(&self.assignment_id)
            .get_result::<Self>(conn)
    }

    fn delete(&self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::delete(assignments::table.find(&self.assignment_id))
            .get_result::<Self>(conn)
    }

    fn get_all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        todo!()
    }
}
