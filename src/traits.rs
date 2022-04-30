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

pub trait ClassUser
    where
        Self: Sized,
{
    fn create(uid: &String, class_id: &String, conn: &PgConnection) -> ThearningResult<Self>;

    fn load_in_class(class_id: &String, conn: &PgConnection) -> ThearningResult<Vec<Self>>;

    fn find(uid: &String, conn: &PgConnection) -> ThearningResult<Vec<Self>>;
}

pub trait Embedable {}
