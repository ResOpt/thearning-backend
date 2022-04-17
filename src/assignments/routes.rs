use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;

use crate::assignments::models::{Assignment, FillableAssignments};
use crate::auth::ApiKey;
use crate::db;
use crate::db::DbConn;
use crate::files::models::UploadedFile;
use crate::utils::update;

#[derive(Serialize, Deserialize)]
struct AssignmentData {
    id: String,
    assignment: FillableAssignments,
    files: Option<Vec<UploadedFile>>,
}

#[post("/")]
fn draft(key: ApiKey, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let default = Assignment::default();

    default.draft(&conn);

    Ok(Json(json!({"assignment_id": default.assignment_id})))
}

#[patch("/", data = "<data>")]
fn update_assignment(key: ApiKey, data: Json<AssignmentData>, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let data = data.into_inner();

    let assignment = match Assignment::get_by_id(&data.id, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::NotFound),
    };

    let new = update(assignment, data.assignment, &conn).unwrap();

    Ok(Json(json!({"new_assignment": new})))
}

#[get("/<assignment_id>", rank = 2)]
fn assignment(key: ApiKey, assignment_id: String, conn: DbConn) -> Result<Json<JsonValue>, Status> {
    unimplemented!()
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount("/api/classroom/assignments", routes![draft, update_assignment])
}
