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

use crate::auth::ClassGuard;
use crate::comments::models::{Comment, FillableComment, FillablePrivateComment, PrivateComment};
use crate::db;
use crate::errors::ThearningResult;
use crate::traits::Manipulable;
use crate::users::models::User;

#[post("/<class_id>/comments", data = "<data>")]
pub fn post_comment(
    key: ClassGuard,
    class_id: &str,
    data: Json<FillableComment>,
    conn: db::DbConn,
) -> Result<Status, Status> {
    let mut data = data.into_inner();

    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    data.user_id = Some(user.user_id);

    let new_comment = match Comment::create(data, &conn) {
        Ok(c) => c,
        Err(_) => return Err(Status::BadRequest),
    };

    Ok(Status::Ok)
}

#[post("/<class_id>/privatecomments", data = "<data>")]
pub fn post_private_comment(
    key: ClassGuard,
    class_id: &str,
    data: Json<FillablePrivateComment>,
    conn: db::DbConn,
) -> Result<Status, Status> {
    let mut data = data.into_inner();

    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    data.user_id = Some(user.user_id);

    let new_comment = match PrivateComment::create(data, &conn) {
        Ok(c) => c,
        Err(_) => return Err(Status::BadRequest),
    };

    Ok(Status::Ok)
}

#[delete("/<class_id>/comments", data = "<data>")]
pub fn delete_comment(
    key: ClassGuard,
    class_id: &str,
    data: String,
    conn: db::DbConn,
) -> Result<Status, Status> {
    let comment = match Comment::find_comment(&data, &conn) {
        Ok(c) => c,
        Err(_) => return Err(Status::NotFound),
    };

    if comment.user_id != key.0 {
        return Err(Status::Unauthorized);
    }

    comment.delete(&conn);

    Ok(Status::Ok)
}

#[delete("/<class_id>/privatecomments", data = "<data>")]
pub fn delete_private_comment(
    key: ClassGuard,
    class_id: &str,
    data: String,
    conn: db::DbConn,
) -> Result<Status, Status> {
    let comment = match PrivateComment::find_comment(&data, &conn) {
        Ok(c) => c,
        Err(_) => return Err(Status::NotFound),
    };

    if comment.user_id != key.0 {
        return Err(Status::Unauthorized);
    }

    comment.delete(&conn);

    Ok(Status::Ok)
}
