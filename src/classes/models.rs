use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};

use crate::classes::utils::*;
use crate::schema::{classes, topics};
use crate::traits::Manipulable;
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

#[derive(
Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable, Debug,
)]
#[belongs_to(Classroom)]
#[table_name = "topics"]
pub struct Topic {
    pub id: String,
    pub topic_name: String,
    pub classroom_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewTopic {
    pub topic_name: String,
    pub classroom_id: String,
}

impl Manipulable<Self> for Classroom {
    fn create(class: Self, conn: &PgConnection) -> QueryResult<Self> {

        let new_class = Self {
            class_id: class.class_id,
            class_name: class.class_name,
            section: class.section,
            class_creator: class.class_creator,
            class_description: class.class_description,
            class_image: class.class_image,
        };
        diesel::insert_into(classes::table)
            .values(&new_class)
            .execute(conn)?;

        classes::table
            .find(new_class.class_id)
            .get_result::<Self>(conn)
    }

    fn update(&self, update: Self, conn: &PgConnection) -> QueryResult<Self> {

        diesel::update(classes::table.filter(classes::class_id.eq(&self.class_id)))
            .set((classes::class_name.eq(&update.class_name),
                        classes::class_creator.eq(&update.class_creator),
                        classes::class_image.eq(&update.class_image),
                        classes::class_description.eq(&update.class_description))).execute(conn)?;

        classes::dsl::classes.find(&self.class_id).get_result::<Self>(conn)
    }

    fn delete(&self, conn: &PgConnection) -> QueryResult<Self> {
        todo!()
    }

    fn get_all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        todo!()
    }
}

impl Topic {
    pub fn create(topic: NewTopic, connection: &PgConnection) -> QueryResult<Self> {
        let generate_code = generate_random_id().to_string();

        let new_topic = Self {
            id: generate_code,
            topic_name: topic.topic_name,
            classroom_id: topic.classroom_id,
        };

        diesel::insert_into(topics::table)
            .values(&new_topic)
            .execute(connection)?;

        topics::table
            .find(new_topic.id)
            .get_result::<Self>(connection)
    }
}
