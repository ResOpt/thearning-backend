use chrono::format::Item::Error;
use jsonwebtoken::{Algorithm, Header};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;
use serde::{Deserialize, Serialize};

use crate::auth::{ApiKey, Claims, generate_token};
use crate::db;
use crate::errors::Errors;
use crate::users::models::{Role, User};
use crate::users::utils::is_email;

#[post("/", data = "<user>")]
fn create(user: Json<User>, connection: db::DbConn) -> Result<Json<User>, Status> {
    let _user = user.into_inner();
    match Role::from_str(&_user.status) {
        Ok(_) => {},
        Err(_) => return Err(Status::Conflict),
    }
    match User::create(_user, &connection) {
        Ok(query) => {
            Ok(Json(query))
        }
        Err(_) => {
            Err(Status::Conflict)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    key: String,
    password: String,
}

#[post("/", format = "application/json", data = "<credentials>")]
fn login(credentials: Json<Credentials>, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let header: Header = Header::new(Algorithm::HS512);
    let key = credentials.key.to_string();
    let password = credentials.password.to_string();

    match User::get_by_key(&key, password, &connection) {
        None => {
            Err(Status::NotFound)
        }
        Some(user) => {
            match User::get_role(&key, &connection) {
                Ok(k) => {
                    match generate_token(&key, &k) {
                        Ok(v) => Ok(Json(json!({ "success": true, "token": v }))),
                        Err(_) => Err(Status::InternalServerError)
                    }
                }
                Err(e) => {
                    Err(Status::InternalServerError)
                }
            }
        }
    }
}

#[get("/", format = "application/json")]
fn info(key: ApiKey, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {

    match User::find_user(&key.0, &connection) {
        Ok(user) => {
            Ok(Json(
                json!({
                    "code": 200,
                    "data": {
                    "user_id": user.user_id,
                    "fullname": user.fullname,
                    "profile_photo": user.profile_photo,
                    "email": user.email,
                    "bio": user.bio,
                    "status": user.status,
                        }
                })
            ))
        }
        Err(_) => Err(Status::NotFound)
    }
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
        .mount("/api/user", routes![create,info])
        .mount("/api/auth", routes![login])
}


