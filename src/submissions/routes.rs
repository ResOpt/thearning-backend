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
use crate::submissions::models::{FillableSubmissions, Submissions};
use crate::traits::Manipulable;
use crate::users::models::User;

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

    match submission.unsubmit(&conn) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}
