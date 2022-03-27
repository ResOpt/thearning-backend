use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::classes::utils::*;
use crate::schema::{classes, topics};
use crate::utils::generate_random_id;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations)]
#[table_name = "classes"]
pub struct Classroom {
    pub class_id: String,
    pub class_name: String,
    pub class_creator: String,
    pub class_description: Option<String>,
    pub class_image: Option<String>,
    pub section: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "classes"]
pub struct NewClassroom {
    pub class_name: String,
    pub section: String,
    pub class_creator: String,
    pub class_description: Option<String>,
    pub class_image: Option<String>,
}

#[derive(
Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable, Debug,
)]
#[belongs_to(Classroom)]
#[table_name = "topics"]
pub struct Topic {
    id: String,
    topic_name: String,
    classroom_id: String,
}

impl Classroom {
    pub fn create(class: NewClassroom, connection: &PgConnection) -> QueryResult<Self> {
        let codes = get_class_codes(connection)?;
        let generate_code = generate_class_code(&codes);
        let new_class = Self {
            class_id: generate_code,
            class_name: class.class_name,
            section: class.section,
            class_creator: class.class_creator,
            class_description: class.class_description,
            class_image: class.class_image,
        };
        diesel::insert_into(classes::table)
            .values(&new_class)
            .execute(connection)?;

        classes::table
            .find(new_class.class_id)
            .get_result::<Self>(connection)
    }
}

impl Topic {
    pub fn create(topic_name: &String, class_id: &String, connection: &PgConnection) -> QueryResult<Self> {
        let generate_code = generate_random_id().to_string();

        let new_topic = Self {
            id: generate_code,
            topic_name: topic_name.to_string(),
            classroom_id: class_id.to_string(),
        };

        diesel::insert_into(topics::table)
            .values(&new_topic)
            .execute(connection)?;

        topics::table
            .find(new_topic.id)
            .get_result::<Self>(connection)
    }
}
