use jsonwebtoken::{Algorithm, Header};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

use crate::auth::{ApiKey, Claims, generate_token};
use crate::db;
use crate::users::models::{Role, User};
use crate::assignments::models::{FillableAssignments, Assignments};

#[post("/create", data = "<data>")]
fn create(key: ApiKey, data: Json<FillableAssignments>, conn: db::DbConn) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let d = data.into_inner();

    match Assignments::create(d, &conn) {
        Ok(v) => Ok(Json(json!({"success":true, "assignment_id":v.assignment_id}))),
        Err(_) => Err(Json(json!({"success":false})))
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/assignments", routes![create])
}