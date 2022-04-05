use std::{env, fs};
use std::path::Path;
use diesel::{EqAll, QueryDsl, RunQueryDsl};
use jsonwebtoken::{Algorithm, Header};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;
use serde::{Deserialize, Serialize};

use crate::auth::{ApiKey, generate_token};
use crate::db;
use crate::files::models::UploadedFile;
use crate::files::routes;
use crate::schema::files::dsl::files;
use crate::schema::files::{file_path, file_url};
use crate::schema::users::dsl::users;
use crate::schema::users::{email, profile_photo, user_id};
use crate::users::models::{InsertableUser, Role, User};
use crate::users::utils::is_email;
use crate::traits::Manipulable;

#[post("/", data = "<user>")]
async fn create<'a>(user: Form<InsertableUser<'a>>, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let mut user = user.into_inner();
    match Role::from_str(&user.status) {
        Ok(_) => {}
        Err(_) => return Err(Status::Conflict),
    }

    let new_user = User {
        user_id: user.user_id.to_string(),
        fullname: user.fullname.to_string(),
        profile_photo: "".to_string(),
        email: user.email.to_string(),
        password: user.password.to_string(),
        bio: user.bio.to_string(),
        status: user.status.to_string(),
    };

    let cloned_user = new_user.clone();

    match User::create(new_user, &connection) {
        Ok(query) => {}
        Err(_) => {
            return Err(Status::Conflict)
        },
    }

    let image_file = match &user.image.name() {
        Some(_) => {
            match routes::process_image(user.image, &user.file_name).await {
                Ok(v) => v,
                Err(_) => return Err(Status::BadRequest)
            }
        }
        None => {
            let url = env::var("SITE_URL").unwrap();
            format!("{}/api/media/img/placeholder.png", url)
        }
    };

    diesel::update(users.filter(user_id.eq_all(cloned_user.user_id)))
        .set(profile_photo.eq_all(image_file))
        .execute(&*connection).unwrap();

    Ok(Json(json!({"status": 200})))
}

#[delete("/", data = "<uid>")]
fn delete_user(key: ApiKey, uid: String, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {

    match User::get_role(&key.0, &*conn).unwrap() {
        Role::Admin => {
            match diesel::delete(users.filter(user_id.eq_all(&uid)))
                .execute(&*conn) {
                Ok(_) => {

                }
                Err(_) => {
                    match diesel::delete(users.filter(email.eq_all(&uid)))
                        .execute(&*conn) {
                        Ok(_) => {

                        }
                        Err(_) => {
                            return Err(Status::BadRequest)
                        }
                    }
                }
            }
        }
        _ => return Err(Status::Unauthorized)
    }

    Ok(Json(json!({"status": 200})))
}


#[derive(Serialize, Deserialize)]
struct Credentials {
    key: String,
    password: String,
}

#[post("/", format = "application/json", data = "<credentials>")]
fn login(
    credentials: Json<Credentials>,
    connection: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    let header: Header = Header::new(Algorithm::HS512);
    let key = credentials.key.to_string();
    let password = credentials.password.to_string();

    match User::get_by_key(&key, password, &connection) {
        None => Err(Status::NotFound),
        Some(user) => match User::get_role(&key, &connection) {
            Ok(k) => match generate_token(&key, &k) {
                Ok(v) => Ok(Json(json!({ "status": 200, "token": v }))),
                Err(_) => Err(Status::InternalServerError),
            },
            Err(e) => Err(Status::InternalServerError),
        },
    }
}

#[get("/", format = "application/json")]
fn info(key: ApiKey, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    match User::find_user(&key.0, &connection) {
        Ok(user) => Ok(Json(json!({
            "status": 200,
            "data": {
            "user_id": user.user_id,
            "fullname": user.fullname,
            "profile_photo": user.profile_photo,
            "email": user.email,
            "bio": user.bio,
            "status": user.status,
                }
        }))),
        Err(_) => Err(Status::NotFound),
    }
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
        .mount("/api/user", routes![create, info, delete_user])
        .mount("/api/auth", routes![login])
}
