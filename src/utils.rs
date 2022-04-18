use std::ops::Deref;

use chrono::NaiveDate;
use diesel::{PgConnection, QueryResult};
use rand::Rng;
use rocket::form;
use rocket::form::{DataField, FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use crate::errors::ThearningResult;

use crate::traits::Manipulable;

pub fn generate_random_id() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>().abs()
}

pub fn update<T, U>(table: T, new_data: U, conn: &PgConnection)
                    -> ThearningResult<T>
    where T: Manipulable<U> {
    table.update(new_data, conn)
}

#[derive(Serialize, Deserialize)]
pub struct NaiveDateForm(NaiveDate);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for NaiveDateForm {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match NaiveDate::parse_from_str(field.value, "%Y-%m-%d") {
            Ok(res) => Ok(NaiveDateForm(res)),
            Err(_) => Err(form::Error::validation(""))?
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        unimplemented!()
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;
    fn deref(&self) -> &NaiveDate {
        &self.0
    }
}
