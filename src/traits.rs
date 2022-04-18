use diesel::{PgConnection, QueryResult};
use crate::errors::ThearningResult;

pub trait Manipulable<T>
    where
        Self: Sized,
{
    fn create(new_data: T, conn: &PgConnection) -> ThearningResult<Self>;

    fn update(&self, update: T, conn: &PgConnection) -> ThearningResult<Self>;

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self>;

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>>;
}

pub trait ClassUser {
    fn create(uid: &String, class_id: &String, connection: &PgConnection) -> ThearningResult<Self>
        where
            Self: Sized;
}
