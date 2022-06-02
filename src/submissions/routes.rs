use chrono::Local;
use diesel::dsl::any;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::{self, routes};
use rocket_dyn_templates::handlebars::JsonValue;

use crate::auth::{ApiKey, ClassGuard};
use crate::db;
use crate::schema::submissions::dsl::submissions;
use crate::submissions::models::{FillableMark, FillableSubmissions, Mark, Submissions};
use crate::traits::Manipulable;
use crate::users::models::User;
use crate::utils::update;

#[post("/<class_id>/submissions/<submission_id>/submit")]
pub fn submit_submission(
    key: ClassGuard,
    class_id: &str,
    submission_id: &str,
    conn: db::DbConn,
) -> Result<Status, Status> {
    let user = match User::find_user(&key.0, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Status::NotFound),
    };

    let submission = match Submissions::find_submission(&submission_id.to_string(), &conn) {
        Ok(sub) => sub,
        Err(_) => return Err(Status::NotFound),
    };

    match Submissions::get_by_id(&submission.assignment_id, &user.user_id, &conn) {
        Ok(_) => (),
        Err(_) => return Err(Status::Unauthorized),
    };

    if submission.submitted {
        return Err(Status::BadRequest);
    }

    match submission.submit(&conn) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/<class_id>/submissions/<submission_id>/unsubmit")]
pub fn unsubmit_submission(
    key: ClassGuard,
    class_id: &str,
    submission_id: &str,
    conn: db::DbConn,
) -> Result<Status, Status> {
    let user = match User::find_user(&key.0, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Status::NotFound),
    };

    let submission = match Submissions::find_submission(&submission_id.to_string(), &conn) {
        Ok(sub) => sub,
        Err(_) => return Err(Status::NotFound),
    };

    match Submissions::get_by_id(&submission.assignment_id, &user.user_id, &conn) {
        Ok(_) => (),
        Err(_) => return Err(Status::Unauthorized),
    };

    if !submission.submitted {
        return Err(Status::BadRequest);
    }

    match submission.submit(&conn) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/<class_id>/submissions/<submission_id>/mark", data = "<mark>")]
pub fn mark_submission(
    key: ClassGuard,
    class_id: &str,
    mark: Json<FillableMark>,
    submission_id: &str,
    conn: db::DbConn,
) -> Result<Json<JsonValue>, Status> {

    let mark = mark.into_inner();

    let user = match User::find_user(&key.0, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Status::NotFound),
    };

    if user.is_student() {
        return Err(Status::Unauthorized);
    }

    let submission = match Submissions::find_submission(&submission_id.to_string(), &conn) {
        Ok(sub) => sub,
        Err(_) => return Err(Status::NotFound),
    };

    if submission.marks_allotted.is_some() {
        return Err(Status::Conflict);
    }

    let new_mark = FillableMark {
        marker_id: Option::from(user.user_id),
        ..mark
    };

    let mark = Mark::create(new_mark, &conn).unwrap();

    submission.mark(&mark.value, &conn).unwrap();

    Ok(Json(json!({
        "mark": mark,
    })))

}

#[patch("/<class_id>/submissions/<submission_id>/mark", data = "<mark>")]
pub fn update_mark(
    key: ClassGuard,
    class_id: &str,
    mark: Json<FillableMark>,
    submission_id: &str,
    conn: db::DbConn,
) -> Result<Status, Status> {

    let mark = mark.into_inner();

    let user = match User::find_user(&key.0, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Status::NotFound),
    };

    if user.is_student() {
        return Err(Status::Unauthorized);
    }

    let this_mark = Mark::get_by_submission_id(&submission_id.to_string(), &conn).unwrap();

    let submission = Submissions::find_submission(&submission_id.to_string(), &conn).unwrap();

    submission.mark(&this_mark.value, &conn).unwrap();

    update(this_mark, mark, &conn).unwrap();

    Ok(Status::Ok)
}
