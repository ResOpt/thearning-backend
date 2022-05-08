use chrono::Local;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, PgConnection};
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
use crate::links::models::Link;
use crate::schema::attachments;
use crate::traits::Manipulable;
use crate::users::models::User;
use crate::utils::{update, generate_random_id};
use crate::assignments::models::AssignmentData;
use crate::traits::Embedable;
use crate::submissions::models::{Submissions, FillableSubmissions};


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

#[derive(Serialize)]
struct AssignmentResponse {
    attachment: Attachment,
    file: Option<UploadedFile>,
    link: Option<Link>,
}

fn get_attachments(vec: Vec<Attachment>, conn: &PgConnection) -> Vec<AssignmentResponse> {

    let mut res = Vec::<AssignmentResponse>::new();

    for thing in vec {
        let resp = AssignmentResponse {
            attachment: thing.clone(),
            file: match &thing.file_id {
                Some(id) => {
                    Some(UploadedFile::receive(id, conn).unwrap())
                }
                None => None,
            },
            link: match &thing.link_id {
                Some(id) => {
                    Some(Link::receive(id, conn).unwrap())
                }
                None => None
            },
        };
        res.push(resp)
    }

    res
}

#[get("/students/<assignment_id>")]
fn students_assignment(key: ApiKey, assignment_id: &str, conn: DbConn) -> Result<Json<JsonValue>, Status> {

    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    if !user.is_student() {
        return Err(Status::Forbidden)
    }

    let assignment = match Assignment::get_by_id(&assignment_id.to_string(), &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound)
    };

    let assignment_attachments = attachments::table.filter(attachments::assignment_id.eq(&assignment.assignment_id)).load::<Attachment>(&*conn).unwrap();

    let submission = match Submissions::get_by_id(&assignment_id.to_string(), &user.user_id, &conn) {
        Ok(sub) => {
            sub
        }
        Err(_) => {
            let new_submission = FillableSubmissions {
                assignment_id: assignment.assignment_id.clone(),
                user_id: user.user_id,
            };

           match Submissions::create(new_submission, &conn) {
               Ok(s) => s,
               Err(_) => return Err(Status::InternalServerError)
           }
        }
    };

    let submission_attachments = attachments::table.filter(attachments::submission_id.eq(&submission.submission_id)).load::<Attachment>(&*conn).unwrap();

    let assignment_resp = get_attachments(assignment_attachments, &conn);

    let submission_resp = get_attachments(submission_attachments, &conn);

    Ok(Json(json!({"assignment_attachments": assignment_resp, "assignment": assignment, "submission": submission, "submission_attachments": submission_resp})))
}

#[get("/teachers/<assignment_id>")]
fn teachers_assignment(key: ApiKey, assignment_id: &str, conn: DbConn) -> Result<Json<JsonValue>, Status> {

    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    if user.is_student() {
        return Err(Status::Forbidden)
    }

    let assignment = match Assignment::get_by_id(&assignment_id.to_string(), &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound)
    };

    let submission = match Submissions::get_by_assignment(&assignment.assignment_id, &conn) {
        Ok(s) => Some(s),
        Err(_) => None,
    };

    let assignment_attachments = attachments::table.filter(attachments::assignment_id.eq(&assignment.assignment_id)).load::<Attachment>(&*conn).unwrap();

    let assignment_resp = get_attachments(assignment_attachments, &conn);

    Ok(Json(json!({"assignment_attachments": assignment_resp, "assignment": assignment, "submissions": submission})))
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount("/api/assignments", routes![draft, update_assignment, delete_assignment, students_assignment, teachers_assignment])
}
