use jsonwebtoken::{Algorithm, Header};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

use crate::auth::{ApiKey, Claims, generate_token};
use crate::db;
use crate::users::models::{Role, User};
use crate::users::utils::is_email;

#[post("/create", data = "<user>")]
fn create(user: Json<User>, connection: db::DbConn) -> Result<Json<User>, Status> {
    User::create(user.into_inner(), &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    key: String,
    password: String,
}

#[post("/login", format = "application/json", data = "<credentials>")]
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

#[get("/info", format = "application/json")]
fn info(key: ApiKey, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let mut key_ = key.0.clone();
    if is_email(&key.0) {
        match User::get_id_from_email(&key.0, &connection) {
            Ok(id) => key_ = id,
            Err(_) => return Err(Status::NotFound)
        }
    }
    match User::find_user(&key_, &connection) {
        Ok(user) => {
            Ok(Json(
                json!({
                    "user_id": user.user_id,
                    "fullname": user.fullname,
                    "profile_photo": user.profile_photo,
                    "email": user.email,
                    "bio": user.bio,
                    "status": user.status,
                })
            ))
        }
        Err(_) => Err(Status::NotFound)
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![create,info])
        .mount("/auth", routes![login])
}


