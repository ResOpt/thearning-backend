use std::{env, fs};
use std::path::Path;
use chrono::Local;
use diesel::{EqAll, QueryDsl, RunQueryDsl};
use dotenv::var;
use jsonwebtoken::{Algorithm, Header};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;
use serde::{Deserialize, Serialize};

use crate::auth::{ApiKey, generate_token};
use crate::db;
use crate::file_routes::process_image;
use crate::files::models::{UploadedFile, UploadType};
use crate::files::routes;
use crate::schema::files::dsl::files;
use crate::schema::files::{file_path, file_url};
use crate::schema::users::dsl::users;
use crate::schema::users::{email, profile_photo, user_id};
use crate::users::models::{InsertableUser, PasswordChange, Role, UpdatableUser, User};
use crate::users::utils::is_email;
use crate::traits::Manipulable;
use crate::utils::update;

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
        birth_place: user.birth_place,
        birth_date: *user.birth_date,
        bio: user.bio.to_string(),
        status: user.status.to_string(),
        created_at: Local::now().naive_local()
    };

    let cloned_user = new_user.clone();

    match User::create(new_user, &connection) {
        Ok(query) => {}
        Err(_) => {
            return Err(Status::Conflict)
        },
    }

    let image_file = match user.image {
        Some(img) => {
            match routes::process_image(img, UploadType::ProfilePhoto, &user.file_name.unwrap_or("file.jpg".to_string())).await {
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

#[post("/update", data = "<data>")]
async fn update_user<'a>(key: ApiKey, data: Form<UpdatableUser<'a>>, conn: db::DbConn) -> Result<Status, Status> {
    let data = data.into_inner();

    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    let cloned_user = user.clone();

    let image: Option<String> = match data.image {
        Some(i) => {
            match process_image(i, UploadType::ProfilePhoto,&data.file_name.unwrap_or("filename.jpg".to_string())).await {
                Ok(res) => {
                    let old_file = UploadedFile::get_from_url(&cloned_user.profile_photo, &conn).unwrap();
                    fs::remove_file(old_file.file_path).unwrap();

                    Some(res)
                },
                Err(_) => return Err(Status::BadRequest)
            }
        }
        None => None,
    };

    let updated_user = User {
        user_id: key.0,

        fullname: data.fullname,
        profile_photo: match image {
            Some(i) => i,
            None => cloned_user.profile_photo
        },
        email: data.email,
        password: cloned_user.password,
        birth_place: data.birth_place,
        birth_date: *data.birth_date,
        bio: data.bio,
        status: cloned_user.status,
        created_at: Local::now().naive_local()
    };

    match update(user, updated_user, &conn) {
        Ok(_) => {},
        Err(_) => return Err(Status::UnprocessableEntity)
    };

    Ok(Status::Ok)
}

#[post("/password_change", format = "application/json", data = "<data>")]
fn password_change(key: ApiKey, data: Json<PasswordChange>, conn: db::DbConn) -> Result<Status, Status> {

    let data = data.into_inner();

    let user = match User::find_user(&key.0, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::NotFound)
    };

    match user.update_password(data, &conn) {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::Unauthorized)
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
            "created_at": user.created_at,
                }
        }))),
        Err(_) => Err(Status::NotFound),
    }
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
        .mount("/api/user", routes![create, info, delete_user, password_change, update_user])
        .mount("/api/auth", routes![login])
}
