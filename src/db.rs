use std::env;
use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2;
use rocket::{Request, State};
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{self, FromRequest};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

#[cfg(not(test))]
pub fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

#[cfg(test)]
pub fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL_TEST").expect("DATABASE_URL must be set")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.rocket().state::<Pool>().unwrap();
        match pool.get() {
            Ok(conn) => request::Outcome::Success(DbConn(conn)),
            Err(_) => request::Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
