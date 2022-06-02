use std::env;

use crate::assignments::models::Assignment;
use chrono::Local;
use diesel::dsl::any;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::{self, routes};
use rocket_dyn_templates::handlebars::JsonValue;
use crate::announcements::models::Announcement;

use crate::assignments::routes::*;
use crate::auth::ApiKey;
use crate::auth::ClassGuard;
use crate::classes::models::{Classroom, NewClassroom, NewTopic, Topic};
use crate::classes::utils::{generate_class_code, get_class_codes};
use crate::db;
use crate::db::DbConn;
use crate::errors::ThearningResult;
use crate::files::models::UploadType;
use crate::files::routes;
use crate::schema::classes;
use crate::schema::users;
use crate::submissions::models::{FillableSubmissions, Submissions};
use crate::submissions::routes::*;
use crate::traits::{ClassUser, Manipulable};
use crate::users::models::{Admin, ResponseUser, Role, Student, Teacher, User};
use crate::utils::{load_classuser, update};
use crate::comments::routes::*;
use crate::announcements::routes::*;

#[post("/", data = "<new_class>", rank = 1)]
async fn create_classroom<'a>(
    key: ApiKey,
    new_class: Form<NewClassroom<'a>>,
    connection: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    let user = User::find_user(&key.0, &*connection).unwrap();

    let cloned_key = key.clone();

    match Role::from(user.status.as_str()) {
        Role::Student => return Err(Status::Forbidden),
        _ => {}
    }

    let new_class = new_class.into_inner();

    let codes = get_class_codes(&*connection).unwrap();
    let generate_code = generate_class_code(&codes);

    let class = Classroom {
        class_id: generate_code,
        class_name: new_class.class_name,
        class_creator: Some(cloned_key.0),
        class_description: new_class.class_description,
        class_image: None,
        section: new_class.section,
        created_at: Local::now().naive_local(),
    };

    match Classroom::create(class.clone(), &connection) {
        Ok(_) => {}
        Err(_) => return Err(Status::BadRequest),
    }

    match Role::from(user.status.as_str()) {
        Role::Teacher => {
            create_classuser::<Teacher>(&key.0, &class.class_id, &connection);
        }
        Role::Admin => {
            create_classuser::<Admin>(&key.0, &class.class_id, &connection);
        }
        _ => {}
    }

    let image_file = match new_class.image {
        Some(img) => match routes::process_image(
            img,
            UploadType::ClassPicture,
            &new_class.file_name.unwrap_or("filename.jpg".to_string()),
        )
        .await
        {
            Ok(v) => v,
            Err(_) => return Err(Status::BadRequest),
        },
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
) -> ThearningResult<T> {
    T::create(key, class_id, conn)
}

#[post("/<class_id>", rank = 1)]
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
            Role::Student => {
                let student = match create_classuser::<Student>(&key.0, &class_id, &connection) {
                    Ok(u) => u,
                    Err(_) => return Err(Status::NotFound),
                };
                let assignments = Assignment::load(&class_id, &connection).unwrap();

                if !assignments.is_empty() {
                    for i in assignments {
                        let new_submission = FillableSubmissions {
                            assignment_id: i.assignment_id,
                            user_id: student.user_id.clone(),
                        };
                        Submissions::create(new_submission, &connection);
                    }
                }

                Ok(Json(json!({"status":200})))
            }
            Role::Teacher => match create_classuser::<Teacher>(&key.0, &class_id, &connection) {
                Ok(c) => Ok(Json(json!({"status":200}))),
                Err(_) => Err(Status::Conflict),
            },
            Role::Admin => match create_classuser::<Admin>(&key.0, &class_id, &connection) {
                Ok(c) => Ok(Json(json!({"status":200}))),
                Err(_) => Err(Status::Conflict),
            },
        }
    } else {
        Err(Status::BadRequest)
    }
}

#[get("/", rank = 1)]
fn classrooms(key: ApiKey, connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let user = users::table.find(&key.0).get_result::<User>(&*connection);
    match Role::from(user.as_ref().unwrap().status.as_str()) {
        Role::Student => {
            let student = Student::find(&key.0, &connection).unwrap();

            let students_class_ids = student.into_iter().map(|i| i.class_id).collect::<Vec<_>>();

            let students_classes = classes::table
                .filter(classes::class_id.eq(any(students_class_ids)))
                .load::<Classroom>(&*connection)
                .unwrap();

            Ok(Json(json!({ "class_ids": students_classes })))
        }
        Role::Teacher => {
            let teacher = Teacher::find(&key.0, &connection).unwrap();

            let teachers_class_ids = teacher.into_iter().map(|i| i.class_id).collect::<Vec<_>>();

            let teachers_classes = classes::table
                .filter(classes::class_id.eq(any(teachers_class_ids)))
                .load::<Classroom>(&*connection)
                .unwrap();

            Ok(Json(json!({ "class_ids": teachers_classes })))
        }
        Role::Admin => {
            let admin = Admin::find(&key.0, &connection).unwrap();

            let admins_class_ids = admin.into_iter().map(|i| i.class_id).collect::<Vec<_>>();

            let admins_classes = classes::table
                .filter(classes::class_id.eq(any(admins_class_ids)))
                .load::<Classroom>(&*connection)
                .unwrap();

            Ok(Json(json!({ "class_ids": admins_classes })))
        }
    }
}

#[post("/topic", data = "<new_topic>")]
fn topic(
    key: ClassGuard,
    new_topic: Json<NewTopic>,
    connection: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    let topic = new_topic.into_inner();

    match User::get_role(&key.0, &*connection).unwrap() {
        Role::Student => {
            return Err(Status::Forbidden);
        }
        _ => {
            match Topic::create(topic, &*connection) {
                Ok(_) => {}
                Err(_) => {
                    return Err(Status::Conflict);
                }
            };
        }
    }

    Ok(Json(json!({"status":200})))
}

#[get("/<class_id>", rank = 1)]
fn class(key: ClassGuard, class_id: String, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let class = match Classroom::find(&class_id, &conn) {
        Ok(c) => c,
        Err(_) => return Err(Status::NotFound),
    };

    let students = load_classuser::<Student>(&class_id, &conn)
        .iter()
        .map(|x| User::find_user(&x.user_id, &conn).unwrap())
        .collect::<Vec<ResponseUser>>();
    let admins = load_classuser::<Admin>(&class_id, &conn)
        .iter()
        .map(|x| User::find_user(&x.user_id, &conn).unwrap())
        .collect::<Vec<ResponseUser>>();
    let teachers = load_classuser::<Teacher>(&class_id, &conn)
        .iter()
        .map(|x| User::find_user(&x.user_id, &conn).unwrap())
        .collect::<Vec<ResponseUser>>();

    let assignments = Assignment::load(&class.class_id, &conn).unwrap();

    let announcements = Announcement::load_in_class(&conn, class_id).unwrap();

    Ok(Json(
        json!({"class": class, "students": students, "admins": admins, "teachers": teachers, "assignments":assignments, "announcements":announcements}),
    ))
}

#[patch("/<class_id>", data = "<new_class>")]
async fn update_class<'a>(
    key: ApiKey,
    class_id: String,
    new_class: Form<NewClassroom<'a>>,
    conn: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    let new_class = new_class.into_inner();

    let user = User::find_user(&key.0, &conn).unwrap();

    let class = match Classroom::find(&class_id, &conn) {
        Ok(c) => c,
        Err(_) => return Err(Status::NotFound),
    };

    let cloned_class = class.clone();

    let image_file = match new_class.image {
        Some(img) => match routes::process_image(
            img,
            UploadType::ClassPicture,
            &new_class.file_name.unwrap_or("filename.jpg".to_string()),
        )
            .await
        {
            Ok(v) => Some(v),
            Err(_) => return Err(Status::BadRequest),
        },
        None => {
            None
        }
    };

    let update_ = Classroom {
        class_id: class_id,
        class_name: new_class.class_name,
        class_creator: Option::from(user.user_id),
        class_description: new_class.class_description,
        class_image: match image_file {
            Some(v) => Some(v),
            None => class.class_image,
        },
        section: new_class.section,
        created_at: class.created_at,
    };

    match User::get_role(&key.0, &conn).unwrap() {
        Role::Admin => {
            update(cloned_class, update_, &conn);
        }
        _ => {
            return Err(Status::Forbidden);
        }
    }

    Ok(Json(json!({"status":200})))
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount(
        "/api/classroom",
        routes![
            create_classroom,
            join,
            classrooms,
            topic,
            class,
            draft,
            update_assignment,
            delete_assignment,
            students_assignment,
            teachers_assignment,
            submit_submission,
            unsubmit_submission,
            teachers_submissions,
            all_teachers_assignments,
            post_comment,
            post_private_comment,
            delete_comment,
            delete_private_comment,
            draft_announcement,
            update_announcement,
            delete_announcement,
            get_announcements,
            get_announcement,
            mark_submission,
            update_mark,
            update_class
        ],
    )
}
