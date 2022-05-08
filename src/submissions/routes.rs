use chrono::Local;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::dsl::any;
use diesel::prelude::*;
use rocket::{self, routes};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;

use crate::auth::ApiKey;
use crate::submissions::models::{FillableSubmissions, Submissions};
use crate::db;
use crate::traits::Manipulable;
use crate::users::models::User;

#[post("/submit", data = "<data>")]
fn submit_submission(key: ApiKey, data: String, conn: db::DbConn) -> Result<Status, Status> {

    let user = match User::find_user(&key.0, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Status::NotFound)
    };

    let submission = match Submissions::get_by_id(&data ,&user.user_id, &conn) {
        Ok(sub) => sub,
        Err(_) => return Err(Status::NotFound),
    };

    if submission.submitted {
        return Err(Status::BadRequest)
    }

    match submission.submit(&conn) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/unsubmit", data = "<data>")]
fn unsubmit_submission(key: ApiKey, data: String, conn: db::DbConn) -> Result<Status, Status> {

    let user = match User::find_user(&key.0, &conn) {
        Ok(user) => user,
        Err(_) => return Err(Status::NotFound),
    };

    let submission = match Submissions::get_by_id(&data ,&user.user_id, &conn) {
        Ok(sub) => sub,
        Err(_) => return Err(Status::NotFound),
    };

    if !submission.submitted {
        return Err(Status::BadRequest)
    }

    match submission.unsubmit(&conn) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }

}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
        .mount("/api/submissions", routes![submit_submission, unsubmit_submission])
}