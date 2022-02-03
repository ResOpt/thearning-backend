use diesel::RunQueryDsl;
use rocket::{self, routes};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};

use crate::auth::ApiKey;
use crate::classes::models::{Classroom, NewClassroom};
use crate::db;
use crate::schema::students;
use crate::schema::teachers;
use crate::schema::users;
use crate::users::models::{Role, Teacher, User, Student};
use crate::users::utils::is_email;
use crate::classes::utils::get_class_codes;

#[post("/create", data = "<new_class>")]
pub fn create_classroom(key: ApiKey, new_class: Json<NewClassroom>, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    if let Ok(r) = User::get_role(&key.0, &connection) {
        match r {
            Role::Student => {
                Err(Status::Forbidden)
            }
            Role::Teacher => {
                match Classroom::create(new_class.into_inner(), &connection) {
                    Ok(id) => {
                        if is_email(&key.0) {
                            if let Ok(v) = User::get_id_from_email(&key.0, &connection) {
                                Teacher::create(&v, &id.class_id, &connection).unwrap();
                            }
                        } else {
                            Teacher::create(&key.0, &id.class_id, &connection).unwrap();
                        }

                        Ok(Json(json!({ "success": true, "class_id":  &id.class_id})))
                    }
                    Err(e) => Err(Status::InternalServerError)
                }
            }
            Role::Admin => {
                match Classroom::create(new_class.into_inner(), &connection) {
                    Ok(id) => Ok(Json(json!({ "success": true, "class_id":  id.class_id}))),
                    Err(e) => Err(Status::InternalServerError)
                }
            }
        }
    } else {
        Err(Status::Forbidden)
    }
}

//#[derive(Serialize, Deserialize)]
//pub struct ClassCode(pub String);

#[post("/join/<class_id>")]
pub fn join(key: ApiKey, class_id: String, connection: db::DbConn) -> Result<Json<JsonValue>, Json<JsonValue>> {
    let mut _key = key.0.clone();

    if is_email(&key.0) {
        if let Ok(id) = User::get_id_from_email(&key.0, &connection) {
            _key = id;
        }
    }

    let codes = get_class_codes(&connection).unwrap();

    if !codes.contains(&class_id) {
        return Err(Json(json!({"success":false, "message":"Invalid Class!!"})))
    }

    if let Ok(r) = User::get_role(&_key, &connection) {
        match r {
            Role::Student => {
                Student::create(&_key, &class_id, &connection)
                    .map(|_| Json(json!({"success":true})))
                    .map_err(|_| Json(json!({"success":false, "message":"user already exist"})))
            }
            Role::Teacher => {
                Teacher::create(&_key, &class_id, &connection)
                    .map(|_| Json(json!({"success":true})))
                    .map_err(|_| Json(json!({"success":false, "message":"user already exist"})))
            }
            Role::Admin => {
                //TODO
                unimplemented!()
            }
        }
    }
    else {
        Err(Json(json!({"success":false})))
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/classroom", routes![create_classroom, join])
}