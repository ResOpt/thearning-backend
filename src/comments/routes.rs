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
use crate::comments::models::{Comment, FillableComment};
use crate::db;
use crate::traits::Manipulable;
use crate::users::models::User;

#[post("/", data = "<data>")]
fn post_comment(key: ApiKey, data: Json<FillableComment>, conn: db::DbConn) -> Result<Status, Status> {
    let data = data.into_inner();

    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound)
    };

    let new_comment = match Comment::create(data, &conn) {
        Ok(c) => c,
        Err(_) => return Err(Status::BadRequest)
    };

    Ok(Status::Ok)
}
