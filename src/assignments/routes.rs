use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket_dyn_templates::handlebars::JsonValue;
use serde::{Deserialize, Serialize};

use crate::assignments::models::{Assignments, FillableAssignments};
use crate::auth::ApiKey;
use crate::db;
use crate::db::DbConn;

#[post("/create", data = "<data>")]
fn create(
    key: ApiKey,
    data: Json<FillableAssignments>,
    conn: db::DbConn,
) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let d = data.into_inner();

    match Assignments::create(d, &conn) {
        Ok(v) => Ok(Json(
            json!({"success":true, "assignment_id":v.assignment_id}),
        )),
        Err(_) => Err(Json(json!({"success":false}))),
    }
}

#[get("/<assignment_id>", rank = 2)]
fn assignment(key: ApiKey, assignment_id: String, conn: DbConn) -> Result<Json<JsonValue>, Status> {
    unimplemented!()
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount("/api/classroom/assignments", routes![create])
}
