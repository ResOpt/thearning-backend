use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use rocket::{self, routes};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;

use crate::auth::ApiKey;
use crate::classes::models::{Classroom, NewClassroom};
use crate::classes::utils::get_class_codes;
use crate::db;
use crate::db::DbConn;
use crate::schema::admins::dsl::admins;
use crate::schema::admins::user_id as admin_id;
use crate::schema::classes::dsl::classes as class_q;
use crate::schema::students::dsl::students;
use crate::schema::students::user_id as student_id;
use crate::schema::teachers::dsl::teachers;
use crate::schema::teachers::user_id as teacher_id;
use crate::schema::users;
use crate::users::models::{Admin, ClassUser, Role, Student, Teacher, User};

#[post("/", data = "<new_class>", rank = 1)]
pub fn create_classroom(
    key: ApiKey,
    new_class: Json<NewClassroom>,
    connection: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    if let Ok(r) = User::get_role(&key.0, &connection) {
        match r {
            Role::Student => Err(Status::Forbidden),
            Role::Teacher => match Classroom::create(new_class.into_inner(), &connection) {
                Ok(id) => {
                    Teacher::create(&key.0, &id.class_id, &connection).unwrap();

                    Ok(Json(json!({ "success": true, "class_id":  &id.class_id})))
                }
                Err(e) => Err(Status::InternalServerError),
            },
            Role::Admin => match Classroom::create(new_class.into_inner(), &connection) {
                Ok(id) => Ok(Json(json!({ "success": true, "class_id":  id.class_id}))),
                Err(e) => Err(Status::InternalServerError),
            },
        }
    } else {
        Err(Status::Forbidden)
    }
}

//#[derive(Serialize, Deserialize)]
//pub struct ClassCode(pub String);

fn create_classuser<T: ClassUser>(
    key: &String,
    class_id: &String,
    conn: &DbConn,
) -> Result<Json<JsonValue>, Status> {
    T::create(key, class_id, conn)
        .map(|_| Json(json!({"status":200 as i32})))
        .map_err(|_| Status::BadRequest)
}

#[post("/<class_id>", rank = 2)]
pub fn join(
    key: ApiKey,
    class_id: String,
    connection: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    let codes = get_class_codes(&connection).unwrap();

    if !codes.contains(&class_id) {
        return Err(Status::NotFound);
    }

    if let Ok(r) = User::get_role(&key.0, &connection) {
        match r {
            Role::Student => create_classuser::<Student>(&key.0, &class_id, &connection),
            Role::Teacher => create_classuser::<Teacher>(&key.0, &class_id, &connection),
            Role::Admin => create_classuser::<Admin>(&key.0, &class_id, &connection),
        }
    } else {
        Err(Status::BadRequest)
    }
}

#[get("/", rank = 1)]
fn classrooms(key: ApiKey, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let user = users::table.find(&key.0).get_result::<User>(&*connection);
    match Role::from_str(user.as_ref().unwrap().status.as_str()).unwrap() {
        Role::Student => {
            let student = students
                .filter(student_id.eq(user.unwrap().user_id))
                .load::<Student>(&*connection)
                .unwrap();

            let mut c: Vec<Classroom> = Vec::new();
            for i in student {
                let class = class_q
                    .find(i.class_id)
                    .get_result::<Classroom>(&*connection)
                    .unwrap();
                c.push(class);
            }

            Ok(Json(json!({ "class_id": c })))
        }
        Role::Teacher => {
            let teacher = teachers
                .filter(teacher_id.eq(user.unwrap().user_id))
                .load::<Teacher>(&*connection)
                .unwrap();

            let mut c: Vec<Classroom> = Vec::new();
            for i in teacher {
                let class = class_q
                    .find(i.class_id)
                    .get_result::<Classroom>(&*connection)
                    .unwrap();
                c.push(class);
            }

            Ok(Json(json!({ "class_id": c })))
        }
        Role::Admin => {
            let admin = admins
                .filter(admin_id.eq(user.unwrap().user_id))
                .load::<Admin>(&*connection)
                .unwrap();

            let mut c: Vec<Classroom> = Vec::new();
            for i in admin {
                let class = class_q
                    .find(i.class_id)
                    .get_result::<Classroom>(&*connection)
                    .unwrap();
                c.push(class);
            }

            Ok(Json(json!({ "class_id": c })))
        }
    }
}

#[get("/<class_id>", rank = 2)]
fn class(key: ApiKey, class_id: String, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    unimplemented!()
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount(
        "/api/classroom",
        routes![create_classroom, join, classrooms],
    )
}
