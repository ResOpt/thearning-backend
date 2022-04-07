use diesel::{PgConnection, QueryResult};
use rand::Rng;
use crate::traits::Manipulable;

pub fn generate_random_id() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>().abs()
}

pub fn update<T: Manipulable<U>, U>(table: T, new_data: U, conn: &PgConnection)
    -> QueryResult<T> {
    table.update(new_data, conn)
}
