use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;

use crate::assignments::models::{Assignments, FillableAssignments};
use crate::attachments::models::Attachment;
use crate::auth::ApiKey;
use crate::db;
use crate::db::DbConn;
use crate::files::models::UploadedFile;

#[derive(Serialize, Deserialize)]
struct AssignmentData {
    assignment: FillableAssignments,
    files: Option<Vec<UploadedFile>>,
}

#[post("/create", data = "<data>")]
fn create(
    key: ApiKey,
    data: Json<AssignmentData>,
    conn: db::DbConn,
) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let d = data.into_inner();
    let assignment = d.assignment.clone();
    let files = d.files.clone();

    let new_assignment = match Assignments::create(assignment, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Json(json!({"success":false}))),
    };
    match &files {
        Some(f) => {
            for file in f {
                let new_file = match UploadedFile::new(&file.file_id, &file.filename, &file.file_path, &file.file_url,&file.filetype, &*conn) {
                    Ok(nf) => nf,
                    Err(_) => return Err(Json(json!({"success":false}))),
                };
                let new_attachment = match Attachment::create(
                    &new_file.file_id,
                    &Some(&new_assignment.assignment_id),
                    &key.0,
                    &*conn,
                ) {
                    Ok(na) => na,
                    Err(_) => return Err(Json(json!({"success":false}))),
                };
            }
        }
        None => {}
    };

    Ok(Json(
        json!({"success": true, "assignment_id": new_assignment.assignment_id}),
    ))
}

#[get("/<assignment_id>", rank = 2)]
fn assignment(key: ApiKey, assignment_id: String, conn: DbConn) -> Result<Json<JsonValue>, Status> {
    unimplemented!()
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount("/api/classroom/assignments", routes![create])
}
