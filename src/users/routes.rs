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
use crate::users::models::{InsertableUser, Role, User};

#[post("/", data = "<user>")]
async fn create<'a>(user: Form<InsertableUser<'a>>, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let mut _user = user.into_inner();
    match Role::from_str(&_user.status) {
        Ok(_) => {}
        Err(_) => return Err(Status::Conflict),
    }

    // let path = match _user.image.name() {
    //     Some(v) => v,
    //     None => return Err(Status::BadRequest)
    // };

    // let extract_filename = Path::new(&path).file_name();
    //
    // let extracted: String = match extract_filename.unwrap().to_str().map(|s| s.to_string()) {
    //     Some(v) => v,
    //     None => return Err(Status::UnprocessableEntity),
    // };

    let image_file = match &_user.image.name() {
        Some(_) => {
            match routes::process_image(_user.image, &_user.file_name).await {
                Ok(v) => v,
                Err(_) => return Err(Status::BadRequest)
            }
        }
        None => {
            let url = env::var("SITE_URL").unwrap();
            format!("{}/api/media/img/placeholder.png", url)
        }
    };

    let new_user = User {
        user_id: _user.user_id.to_string(),
        fullname: _user.fullname.to_string(),
        profile_photo: image_file.clone(),
        email: _user.email.to_string(),
        password: _user.password.to_string(),
        bio: _user.bio.to_string(),
        status: _user.status.to_string(),
    };

    match User::create(new_user, &connection) {
        Ok(query) => Ok(Json(json!({
            "status": 200,
        }))),
        Err(_) => {
            let file_obj = files
                .filter(file_url.eq_all(image_file))
                .get_result::<UploadedFile>(&*connection).unwrap();
            fs::remove_file(&file_obj.file_path).unwrap();
            diesel::delete(files.filter(file_path.eq_all(&file_obj.file_path)))
                .execute(&*connection)
                .unwrap();
            Err(Status::Conflict)
        },
    }
}


#[delete("/", data = "<user_id>")]
fn delete_user(key: ApiKey, user_id: String, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {
    todo!()
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
        .mount("/api/user", routes![create, info])
        .mount("/api/auth", routes![login])
}
