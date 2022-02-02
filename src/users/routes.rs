use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

use crate::users::models::{User, Role};
use rocket::http::Status;
use crate::db;
use jsonwebtoken::{Header, Algorithm};
use crate::auth::{Claims, generate_token, ApiKey};

#[post("/create", format="application/json", data = "<user>")]
fn create(user: Json<User>, connection: db::DbConn) -> Result<Json<User>, Status> {
    User::create(user.into_inner(), &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    key: String,
    password: String
}

#[post("/login", data = "<credentials>")]
fn login(credentials: Json<Credentials>, connection: db::DbConn) ->  Result<Json<JsonValue>, Status> {
    let header: Header = Header::new(Algorithm::HS512);
    let key = credentials.key.to_string();
    let password = credentials.password.to_string();

    match User::get_by_key(&key, password, &connection) {
        None => {
            Err(Status::NotFound)
        },
        Some(user) => {
            match User::get_role(&key, &connection) {
                Ok(k) => {
                    match generate_token(&key, &k){
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

// This is just for testing purpose
#[get("/info")]
fn info(key: ApiKey) -> Json<JsonValue> {
    Json(json!(
        {
            "success": true,
            "message": key.0
        }
    ))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![create,info])
        .mount("/auth", routes![login])
}


