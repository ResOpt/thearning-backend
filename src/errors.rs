use std::fmt;
use rocket_contrib::json::{JsonValue, Json};
use rocket::Request;

#[derive(Debug)]
pub enum Errors {
    FailedToCreateJWT,
    TokenExpired,
    TokenInvalid
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::FailedToCreateJWT => write!(f, "Failed to create JWT!"),
            Errors::TokenExpired => write!(f, "Token is already expired!"),
            Errors::TokenInvalid => write!(f, "Token is invalid!"),
        }
    }
}

#[catch(401)]
fn unauthorized() -> Json<JsonValue> {
    Json(json!({"success": false, "code": 401}))
}

#[catch(404)]
fn not_found() -> Json<JsonValue> {
    Jason(jason!({"success":false, "code": 404}))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.register(catchers![unauthorized, not_found])
}
