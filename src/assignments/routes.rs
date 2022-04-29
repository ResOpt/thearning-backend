use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::http::{RawStr, Status};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;

use crate::assignments::models::{Assignment, FillableAssignments};
use crate::attachments::models::Attachment;
use crate::auth::ApiKey;
use crate::db;
use crate::db::DbConn;
use crate::files::models::UploadedFile;
use crate::schema::attachments;
use crate::traits::Manipulable;
use crate::utils::update;

#[derive(Serialize, Deserialize)]
struct AssignmentData {
    id: String,
    assignment: FillableAssignments,
    files: Option<Vec<String>>,
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

#[delete("/<assignment_id>")]
fn delete_assignment(key: ApiKey, assignment_id: String, conn: db::DbConn) -> Result<Status, Status> {
    let assignment = match Assignment::get_by_id(&assignment_id, &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound)
    };

    assignment.delete(&conn).unwrap();

    let att = match Attachment::load_by_assignment_id(&assignment.assignment_id, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::NotFound)
    };

    att.into_iter().for_each(|i| {
        i.delete(&conn).unwrap();
    });

    Ok(Status::Ok)
}

#[get("/<assignment_id>")]
fn assignment(key: ApiKey, assignment_id: &str, conn: DbConn) -> Result<Json<JsonValue>, Status> {

    let assignment = match Assignment::get_by_id(&assignment_id.to_string(), &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound)
    };

    let files = attachments::table.filter(attachments::assignment_id.eq(assignment.assignment_id)).load::<Attachment>(&*conn).unwrap();

    Ok(Json(json!({"files": files})))
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount("/api/assignments", routes![draft, update_assignment, assignment, delete_assignment])
}
