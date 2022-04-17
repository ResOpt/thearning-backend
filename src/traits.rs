use diesel::{PgConnection, QueryResult};

pub trait Manipulable<T>
    where
        Self: Sized,
{
    fn create(new_data: T, conn: &PgConnection) -> QueryResult<Self>;

    fn update(&self, update: T, conn: &PgConnection) -> QueryResult<Self>;

    fn delete(&self, conn: &PgConnection) -> QueryResult<Self>;

    fn get_all(conn: &PgConnection) -> QueryResult<Vec<Self>>;
}

pub trait ClassUser {
    fn create(uid: &String, class_id: &String, connection: &PgConnection) -> QueryResult<Self>
        where
            Self: Sized;
}
