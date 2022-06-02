use crate::errors::ThearningResult;
use chrono::{Local, NaiveDateTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};

use crate::schema::{classes, topics};
use crate::traits::{ClassUser, Manipulable};
use crate::users::models::{Admin, Student, Teacher};
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Clone)]
#[table_name = "classes"]
pub struct Classroom {
    pub class_id: String,
    pub class_name: String,
    pub class_creator: Option<String>,
    pub class_description: Option<String>,
    pub class_image: Option<String>,
    pub section: String,
    pub created_at: NaiveDateTime,
}

#[derive(FromForm)]
pub struct NewClassroom<'a> {
    pub class_name: String,
    pub section: String,
    pub class_creator: Option<String>,
    pub class_description: Option<String>,
    pub image: Option<TempFile<'a>>,
    pub file_name: Option<String>,
}

#[derive(FromForm)]
pub struct UpdatableClassroom<'a> {
    pub class_name: Option<String>,
    pub section: Option<String>,
    pub class_description: Option<String>,
    pub image: Option<TempFile<'a>>,
    pub file_name: Option<String>,
}

#[derive(
    Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable, Debug,
)]
#[belongs_to(Classroom)]
#[table_name = "topics"]
pub struct Topic {
    pub id: String,
    pub topic_name: String,
    pub classroom_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewTopic {
    pub topic_name: String,
    pub classroom_id: String,
}

impl Classroom {
    pub fn find(id: &String, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(classes::table.find(id).get_result::<Self>(conn)?)
    }

    pub fn user_in_class(class_id: &String, uid: &String, conn: &PgConnection) -> bool {
        let students = Student::load_in_class(class_id, conn).unwrap();
        let teachers = Teacher::load_in_class(class_id, conn).unwrap();
        let admins = Admin::load_in_class(class_id, conn).unwrap();
        match (
            students.iter().find(|x| &x.user_id == uid),
            teachers.iter().find(|x| &x.user_id == uid),
            admins.iter().find(|x| &x.user_id == uid),
        ) {
            (None, None, None) => false,
            _ => true,
        }
    }
}

impl Manipulable<Self> for Classroom {
    fn create(class: Self, conn: &PgConnection) -> ThearningResult<Self> {
        let new_class = Self {
            class_id: class.class_id,
            class_name: class.class_name,
            section: class.section,
            class_creator: class.class_creator,
            class_description: class.class_description,
            class_image: class.class_image,
            created_at: Local::now().naive_local(),
        };
        diesel::insert_into(classes::table)
            .values(&new_class)
            .execute(conn)?;

        let res = classes::table
            .find(new_class.class_id)
            .get_result::<Self>(conn)?;

        Ok(res)
    }

    fn update(&self, update: Self, conn: &PgConnection) -> ThearningResult<Self> {
        diesel::update(classes::table.filter(classes::class_id.eq(&self.class_id)))
            .set((
                classes::class_name.eq(&update.class_name),
                classes::class_creator.eq(&update.class_creator),
                classes::section.eq(&update.section),
                classes::class_image.eq(&update.class_image),
                classes::class_description.eq(&update.class_description),
            ))
            .execute(conn)?;

        let res = classes::dsl::classes
            .find(&self.class_id)
            .get_result::<Self>(conn)?;

        Ok(res)
    }

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        diesel::delete(classes::table.filter(classes::class_id.eq(&self.class_id)))
            .execute(conn)?;

        let res = classes::dsl::classes
            .find(&self.class_id)
            .get_result::<Self>(conn)?;

        Ok(res)
    }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        Ok(classes::table.load::<Self>(conn)?)
    }
}

impl Topic {
    pub fn create(topic: NewTopic, connection: &PgConnection) -> QueryResult<Self> {
        let generate_code = generate_random_id().to_string();

        let new_topic = Self {
            id: generate_code,
            topic_name: topic.topic_name,
            classroom_id: topic.classroom_id,
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(topics::table)
            .values(&new_topic)
            .execute(connection)?;

        topics::table
            .find(new_topic.id)
            .get_result::<Self>(connection)
    }
}
