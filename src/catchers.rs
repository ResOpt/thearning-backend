use std::fmt;

use rocket::serde::json::{Json, Value as JsonValue};
use rocket::serde::json::serde_json::json;

#[derive(Debug)]
pub enum Errors {
    FailedToCreateJWT,
    TokenExpired,
    TokenInvalid,
    UserAlreadyExist,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::FailedToCreateJWT => write!(f, "Failed to create JWT!"),
            Errors::TokenExpired => write!(f, "Token is already expired!"),
            Errors::TokenInvalid => write!(f, "Token is invalid!"),
            Errors::UserAlreadyExist => write!(f, "User already exist!"),
        }
    }
}

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
