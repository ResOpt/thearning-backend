use diesel::{EqAll, QueryDsl, RunQueryDsl};
use rocket::{self, routes};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};

use diesel::prelude::*;

use crate::auth::ApiKey;
use crate::classes::models::{Classroom, NewClassroom};
use crate::db;
use crate::schema::students::dsl::students;
use crate::schema::teachers;
use crate::users::models::{Role, Teacher, User, Student};
use crate::users::utils::is_email;
use crate::classes::utils::get_class_codes;
use crate::db::DbConn;
use crate::schema::classes;
use crate::schema::students::user_id;
use crate::schema::users;

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
pub fn join(key: ApiKey, class_id: String, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let mut _key = key.0.clone();

    if is_email(&key.0) {
        match User::get_id_from_email(&key.0, &connection) {
            Ok(id) => { _key = id; }
            Err(_) => return Err(Status::NotFound)
        }
    }

    let codes = get_class_codes(&connection).unwrap();

    if !codes.contains(&class_id) {
        return Err(Status::NotFound)
    }

    if let Ok(r) = User::get_role(&_key, &connection) {
        match r {
            Role::Student => {
                Student::create(&_key, &class_id, &connection)
                    .map(|_| Json(json!({"success":true, "role":"student"})))
                    .map_err(|_| Status::BadRequest)
            }
            Role::Teacher => {
                Teacher::create(&_key, &class_id, &connection)
                    .map(|_| Json(json!({"success":true, "role":"student"})))
                    .map_err(|_| Status::BadRequest)
            }
            Role::Admin => {
                //TODO
                unimplemented!()
            }
        }
    }
    else {
        Err(Status::BadRequest)
    }
}

#[get("/")]
fn classrooms(key: ApiKey, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let mut key_ = key.0.clone();
    if is_email(&key.0) {
        key_ = match User::get_id_from_email(&key.0, &connection) {
            Ok(v) => v,
            Err(e) => return Err(Status::BadRequest)
        }
    }

    let user = users::table.find(key_).get_result::<User>(&*connection);
    match Role::from_str(user.as_ref().unwrap().status.as_str()).unwrap() {
        Role::Student => {
            let student = students
                .filter(user_id
                    .eq(user
                        .unwrap()
                        .user_id))
                .load::<Student>(&*connection)
                .unwrap();

            Ok(Json(json!({"class_id":student
                .into_iter()
                .map(|x| x.class_id)
                .collect::<Vec<_>>()})))
        },
        _ => Ok(Json(json!({"status":"guru/admin"})))
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/classroom", routes![create_classroom, join, classrooms])
}