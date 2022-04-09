use std::env;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use rocket::{self, routes};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;

use crate::auth::ApiKey;
use crate::classes::models::{Classroom, NewClassroom, NewTopic, Topic};
use crate::classes::utils::{generate_class_code, get_class_codes};
use crate::db;
use crate::db::DbConn;
use crate::files::models::UploadType;
use crate::files::routes;
use crate::schema::admins::dsl::admins;
use crate::schema::admins::user_id as admin_id;
use crate::schema::classes::dsl::classes as class_q;
use crate::schema::students::dsl::students;
use crate::schema::students::user_id as student_id;
use crate::schema::teachers::dsl::teachers;
use crate::schema::teachers::user_id as teacher_id;
use crate::schema::students::class_id as class_id_students;
use crate::schema::teachers::class_id as class_id_teachers;
use crate::schema::users;
use crate::traits::{ClassUser, Manipulable};
use crate::users::models::{Admin, Role, Student, Teacher, User};
use crate::utils::update;

#[post("/", data = "<new_class>", rank = 1)]
async fn create_classroom<'a>(
    key: ApiKey,
    new_class: Form<NewClassroom<'a>>,
    connection: db::DbConn,
) -> Result<Json<JsonValue>, Status> {

    let user = User::find_user(&key.0, &*connection).unwrap();

    match Role::from_str(&user.status).unwrap() {
        Role::Student => return Err(Status::Forbidden),
        _ => {}
    }

    let new_class = new_class.into_inner();

    let codes = get_class_codes(&*connection).unwrap();
    let generate_code = generate_class_code(&codes);

    let class = Classroom {
        class_id: generate_code,
        class_name: new_class.class_name,
        class_creator: new_class.class_creator,
        class_description: new_class.class_description,
        class_image: None,
        section: new_class.section
    };

    match Classroom::create(class.clone(), &connection) {
        Ok(_) => {},
        Err(_) => return Err(Status::BadRequest),
    }

    match Role::from_str(user.status.as_str()).unwrap() {
        Role::Teacher => {
            create_classuser::<Teacher>(&key.0, &class.class_id, &connection);
        }
        Role::Admin => {
            create_classuser::<Admin>(&key.0, &class.class_id, &connection);
        }
        _ => {}
    }

    let image_file = match new_class.image {
        Some(img) => match routes::process_image(img, UploadType::ClassPicture,&new_class.file_name.unwrap_or("filename.jpg".to_string())).await {
            Ok(v) => v,
            Err(_) => return Err(Status::BadRequest)
        }
        None => {
            let url = env::var("SITE_URL").unwrap();
            format!("{}/api/media/img/placeholder.png", url)
        }
    };

    let update_ = Classroom {
        class_image: Some(image_file),
        ..class.clone()
    };

    update(class.clone(), update_, &connection).unwrap();

    Ok(Json(json!({"class_id":class.class_id})))
}

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

#[post("/topic", data = "<new_topic>")]
fn topic(key: ApiKey, new_topic: Json<NewTopic>, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let topic = new_topic.into_inner();

    match User::get_role(&key.0, &*connection).unwrap() {
        Role::Student => {
            return Err(Status::Forbidden)
        }
        _ => {
            match Topic::create(topic, &*connection) {
                Ok(_) => {}
                Err(_) => {
                    return Err(Status::Conflict)
                }
            };
        }
    }

    Ok(Json(json!({"status":200})))
}

#[get("/<class_id>", rank = 2)]
fn class(key: ApiKey, class_id: String, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let students_ = students
        .filter(class_id_students.eq(&class_id))
        .load::<Student>(&*connection)
        .unwrap().into_iter().collect::<Vec<Student>>();

    let teachers_ = teachers
        .filter(class_id_teachers.eq(&class_id))
        .load::<Teacher>(&*connection)
        .unwrap().into_iter().collect::<Vec<Teacher>>();

    todo!()
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount(
        "/api/classroom",
        routes![create_classroom, join, classrooms, topic, class],
    )
}
