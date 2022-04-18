use std::fmt;
use diesel::ConnectionError;
use rocket::http::Status;

use rocket::serde::json::{Json, Value as JsonValue};
use rocket::serde::json::serde_json::json;

#[derive(Debug)]
pub enum ErrorKind {
    QueryError(diesel::result::Error),
    IOError(std::io::Error),
    DBError(ConnectionError),
    JWTError(jsonwebtoken::errors::Error),
    InvalidValue,
}

impl From<diesel::result::Error> for ErrorKind {
    fn from(error: diesel::result::Error) -> Self {
        ErrorKind::QueryError(error)
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(error: std::io::Error) -> Self {
        ErrorKind::IOError(error)
    }
}

impl From<ConnectionError> for ErrorKind {
    fn from(error: ConnectionError) -> Self {
        ErrorKind::DBError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for ErrorKind {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        ErrorKind::JWTError(error)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ErrorKind::QueryError(err) => err.to_string(),
            ErrorKind::IOError(err) => err.to_string(),
            ErrorKind::DBError(err) => err.to_string(),
            ErrorKind::JWTError(err) => err.to_string(),
            ErrorKind::InvalidValue => "Invalid".to_string(),
        };

        write!(f, "{}", msg)
    }
}

pub type ThearningResult<T> = Result<T, ErrorKind>;

#[catch(401)]
fn unauthorized() -> Json<JsonValue> {
    Json(json!({"success": false, "code": 401}))
}

#[catch(404)]
fn not_found() -> Json<JsonValue> {
    Json(json!({"success":false, "code": 404}))
}

#[catch(400)]
fn bad_request() -> Json<JsonValue> {
    Json(json!({"success":false, "status":400}))
}

#[catch(409)]
fn conflict() -> Json<JsonValue> {
    Json(json!({"success":false, "status": 409}))
}

#[catch(500)]
fn server_error() -> Json<JsonValue> {
    Json(json!({"success":false, "status": 500}))
}

#[catch(403)]
fn forbidden() -> Json<JsonValue> {
    Json(json!({"success":false, "status": 403}))
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.register(
        "/",
        catchers![
            unauthorized,
            not_found,
            bad_request,
            conflict,
            server_error,
            forbidden
        ],
    )
}
