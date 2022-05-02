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

// #[post("/", data = "<data>")]
// fn post_submission