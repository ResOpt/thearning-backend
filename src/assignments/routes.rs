use chrono::Local;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, PgConnection};
use rocket::http::{RawStr, Status};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;

use crate::assignments::models::{Assignment, FillableAssignments};
use crate::attachments::models::Attachment;
use crate::auth::ApiKey;
use crate::db;
use crate::db::DbConn;
use crate::files::models::UploadedFile;
use crate::links::models::Link;
use crate::schema::attachments;
use crate::traits::{Manipulable, ClassUser};
use crate::users::models::{User, Student};
use crate::utils::{update, generate_random_id};
use crate::assignments::models::AssignmentData;
use crate::traits::Embedable;
use crate::submissions::models::{Submissions, FillableSubmissions};


#[post("/<class_id>/assignments")]
pub fn draft(key: ApiKey, class_id: &str, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let default = Assignment::default();

    default.draft(&conn);

    Ok(Json(json!({"assignment_id": default.assignment_id})))
}

#[patch("/<class_id>/assignments", data = "<data>")]
pub async fn update_assignment(key: ApiKey, class_id: &str, data: Json<AssignmentData>, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let data = data.into_inner();

    let assignment = match Assignment::get_by_id(&data.id, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::NotFound),
    };

    let students = Student::load_in_class(&class_id.to_string(), &conn).unwrap();
 
    for i in students {
        let new_submission = FillableSubmissions {
            assignment_id: assignment.assignment_id.clone(),
            user_id: i.user_id,
        };
        match Submissions::create(new_submission, &conn) {
            Ok(s) => (),
            Err(_) => return Err(Status::InternalServerError),
        }
    };

    let new = update(assignment, data.assignment, &conn).unwrap();

    Ok(Json(json!({"new_assignment": new})))
}

#[delete("/<class_id>/assignments/<assignment_id>")]
pub fn delete_assignment(key: ApiKey, class_id: &str, assignment_id: String, conn: db::DbConn) -> Result<Status, Status> {
    let assignment = match Assignment::get_by_id(&assignment_id, &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound)
    };

    assignment.delete(&conn).unwrap();

    let att = match Attachment::load_by_assignment_id(&assignment.assignment_id, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::NotFound)
    };

    att.into_iter().for_each(|i| {
        i.delete(&conn).unwrap();
    });

    Ok(Status::Ok)
}

#[derive(Serialize)]
struct AssignmentResponse {
    attachment: Attachment,
    file: Option<UploadedFile>,
    link: Option<Link>,
}

fn get_attachments(vec: Vec<Attachment>, conn: &PgConnection) -> Vec<AssignmentResponse> {

    let mut res = Vec::<AssignmentResponse>::new();

    for thing in vec {
        let resp = AssignmentResponse {
            attachment: thing.clone(),
            file: match &thing.file_id {
                Some(id) => {
                    Some(UploadedFile::receive(id, conn).unwrap())
                }
                None => None,
            },
            link: match &thing.link_id {
                Some(id) => {
                    Some(Link::receive(id, conn).unwrap())
                }
                None => None
            },
        };
        res.push(resp)
    }

    res
}

#[get("/<class_id>/assignments/students/<assignment_id>")]
pub fn students_assignment(key: ApiKey, class_id: &str, assignment_id: &str, conn: DbConn) -> Result<Json<JsonValue>, Status> {

    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    if !user.is_student() {
        return Err(Status::Forbidden)
    }

    let assignment = match Assignment::get_by_id(&assignment_id.to_string(), &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound)
    };

    let assignment_attachments = attachments::table.filter(attachments::assignment_id.eq(&assignment.assignment_id)).load::<Attachment>(&*conn).unwrap();

    let submission = Submissions::get_by_id(&assignment_id.to_string(), &user.user_id, &conn).unwrap();

    let submission_attachments = attachments::table.filter(attachments::submission_id.eq(&submission.submission_id)).load::<Attachment>(&*conn).unwrap();

    let assignment_resp = get_attachments(assignment_attachments, &conn);

    let submission_resp = get_attachments(submission_attachments, &conn);

    Ok(Json(json!({"assignment_attachments": assignment_resp, "assignment": assignment, "submission": submission, "submission_attachments": submission_resp})))
}

#[get("/<class_id>/assignments/teachers/<assignment_id>")]
pub fn teachers_assignment(key: ApiKey, class_id: &str, assignment_id: &str, conn: DbConn) -> Result<Json<JsonValue>, Status> {

    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    if user.is_student() {
        return Err(Status::Forbidden)
    }

    let assignment = match Assignment::get_by_id(&assignment_id.to_string(), &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound)
    };

    let submission = match Submissions::get_by_assignment(&assignment.assignment_id, &conn) {
        Ok(s) => Some(s),
        Err(_) => None,
    };

    let assignment_attachments = attachments::table.filter(attachments::assignment_id.eq(&assignment.assignment_id)).load::<Attachment>(&*conn).unwrap();

    let assignment_resp = get_attachments(assignment_attachments, &conn);

    Ok(Json(json!({"assignment_attachments": assignment_resp, "assignment": assignment, "submissions": submission})))
}

// pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
//     rocket.mount("/api/assignments", routes![draft, update_assignment, delete_assignment, students_assignment, teachers_assignment])
// }
