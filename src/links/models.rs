use crate::errors::ThearningResult;
use crate::schema::links;
use crate::traits::{Embedable, Manipulable};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Clone)]
#[table_name = "links"]
pub struct Link {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Link {
    pub fn receive(id: &String, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(links::table.find(id).get_result::<Self>(conn)?)
    }
}

impl Manipulable<Self> for Link {
    fn create(new_data: Self, conn: &PgConnection) -> ThearningResult<Self> {
        diesel::insert_into(links::table)
            .values(&new_data)
            .execute(conn)?;

        Ok(links::table.find(new_data.id).get_result::<Self>(conn)?)
    }

    fn update(&self, update: Self, conn: &PgConnection) -> ThearningResult<Self> {
        todo!()
    }

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        todo!()
    }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        todo!()
    }
}

impl Embedable for Link {}
