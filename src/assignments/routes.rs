use chrono::Local;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use rocket::http::{RawStr, Status};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::handlebars::JsonValue;
use tokio;

use crate::assignments::models::AssignmentData;
use crate::assignments::models::{Assignment, FillableAssignments};
use crate::attachments::models::Attachment;
use crate::auth::{ApiKey, ClassGuard};
use crate::comments::models::{Comment, Commenter, PrivateComment};
use crate::db;
use crate::db::DbConn;
use crate::files::models::UploadedFile;
use crate::links::models::Link;
use crate::schema::attachments;
use crate::submissions::models::{FillableSubmissions, Submissions};
use crate::traits::Embedable;
use crate::traits::{ClassUser, Manipulable};
use crate::users::models::{Student, User, ResponseUser};
use crate::users::routes::get_user;
use crate::utils::{generate_random_id, update, mailer};

#[post("/<class_id>/assignments")]
pub fn draft(key: ClassGuard, class_id: &str, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let default = Assignment::default();

    default.draft(&conn);

    Ok(Json(json!({"assignment_id": default.assignment_id})))
}

#[patch("/<class_id>/assignments", data = "<data>")]
pub async fn update_assignment(
    key: ClassGuard,
    class_id: &str,
    data: Json<AssignmentData>,
    conn: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    let data = data.into_inner();

    let assignment = match Assignment::get_by_id(&data.id, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::NotFound),
    };

    let students = Student::load_in_class(&class_id.to_string(), &conn).unwrap();

    for i in &students {
        let new_submission = FillableSubmissions {
            assignment_id: assignment.assignment_id.clone(),
            user_id: i.user_id.clone(),
        };
        match Submissions::create(new_submission, &conn) {
            Ok(s) => (),
            Err(_) => return Err(Status::InternalServerError),
        }
    }

    let mut assignment_data = data.assignment;

    assignment_data.creator = Some(get_user(&key.0, &conn).unwrap().user_id);

    let new = update(assignment, assignment_data, &conn).unwrap();

    let creator = User::find_user(&new.creator.as_ref().unwrap(), &conn).unwrap();

    let mut emails = Vec::new();

    for i in &students {
        emails.push(User::find_user(&i.user_id, &conn).unwrap().email)
    }
    
    send_mail(creator, emails, new.clone()).await;

    Ok(Json(json!({ "new_assignment": new })))
}

async fn send_mail(user: User, emails: Vec<String>, assignment: Assignment) {

    let mail = mailer().0;

    let server = mailer().1;

    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello from Lettre!</title>
</head>
<body>
    <div style="display: block; align-items: center;">
        <h2 style="font-family: Arial, Helvetica, sans-serif;">New Assignment from {}: {}</h2>
        <br>
        <h4 style="font-family: Arial, Helvetica, sans-serif;">{}</h4>
    </div>
</body>
</html>"#, &user.fullname, &assignment.assignment_name.unwrap(), &assignment.instructions.unwrap());

    let mail = mail.clone().server(server)
                            .subject("New Assignment");

    for email in emails {
        let send = mail.clone().to(email.as_str()).message(html.as_str(), "H").clone().send();
        let job = tokio::task::spawn(async move {
            send.await.unwrap()
        });
    }
}

#[delete("/<class_id>/assignments/<assignment_id>")]
pub fn delete_assignment(
    key: ClassGuard,
    class_id: &str,
    assignment_id: String,
    conn: db::DbConn,
) -> Result<Status, Status> {
    let user = get_user(&key.0, &conn).unwrap();

    let assignment = match Assignment::get_by_id(&assignment_id, &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound),
    };

    if user.is_student() {
        return Err(Status::Forbidden);
    }

    if assignment.draft == false {
        if user.is_teacher() && assignment.creator.as_ref().unwrap() == &user.user_id {
            return Err(Status::Forbidden);
        }
    }

    assignment.delete(&conn).unwrap();

    let att = match Attachment::load_by_assignment_id(&assignment.assignment_id, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::NotFound),
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
                Some(id) => Some(UploadedFile::receive(id, conn).unwrap()),
                None => None,
            },
            link: match &thing.link_id {
                Some(id) => Some(Link::receive(id, conn).unwrap()),
                None => None,
            },
        };
        res.push(resp)
    }

    res
}

fn get_comments<'a, T>(vec: &'a Vec<T>, conn: &PgConnection) -> Vec<UserComment<'a, T>>
where T: Commenter<Output=String> + Serialize {
    let mut res = Vec::<UserComment<T>>::new();

    for thing in vec {
        let resp = UserComment {
            commenter: {
                let user = User::find_user(thing.get_user_id(), &conn).unwrap();
                ResponseUser::from(user)
            },
            comment: thing,
        };
        res.push(resp)
    }

    res
}

#[derive(Serialize)]
struct UserComment<'a, T: Serialize + Commenter> {
    commenter: ResponseUser,
    comment: &'a T,
}

#[get("/<class_id>/assignments/students/<assignment_id>")]
pub fn students_assignment(
    key: ClassGuard,
    class_id: &str,
    assignment_id: &str,
    conn: DbConn,
) -> Result<Json<JsonValue>, Status> {
    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    if !user.is_student() {
        return Err(Status::Forbidden);
    }

    let assignment = match Assignment::get_by_id(&assignment_id.to_string(), &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound),
    };

    let comments = Comment::load_by_assignment(&assignment.assignment_id, &conn).unwrap();

    let comment_response = get_comments(&comments, &conn);

    let assignment_attachments = attachments::table
        .filter(attachments::assignment_id.eq(&assignment.assignment_id))
        .load::<Attachment>(&*conn)
        .unwrap();

    let submission =
        Submissions::get_by_id(&assignment_id.to_string(), &user.user_id, &conn).unwrap();

    let private_comments =
        PrivateComment::load_by_submission(&submission.submission_id, &conn).unwrap();

    let private_comment_response = get_comments(&private_comments, &conn);

    let submission_attachments = attachments::table
        .filter(attachments::submission_id.eq(&submission.submission_id))
        .load::<Attachment>(&*conn)
        .unwrap();

    let assignment_resp = get_attachments(assignment_attachments, &conn);

    let submission_resp = get_attachments(submission_attachments, &conn);

    Ok(Json(
        json!({"assignment_attachments": assignment_resp, "assignment": assignment, "submission": submission, "submission_attachments": submission_resp, "comments": comment_response, "private_comments": private_comment_response}),
    ))
}

#[derive(Serialize)]
struct SubmissionResponse<'a> {
    submission: &'a Submissions,
    attachment_amount: i32,
    user: ResponseUser,
}

#[get("/<class_id>/assignments/teachers/<assignment_id>")]
pub fn teachers_assignment(
    key: ClassGuard,
    class_id: &str,
    assignment_id: &str,
    conn: DbConn,
) -> Result<Json<JsonValue>, Status> {
    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    if user.is_student() {
        return Err(Status::Forbidden);
    }

    let assignment = match Assignment::get_by_id(&assignment_id.to_string(), &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound),
    };

    let unsubmitted = Submissions::load_unsubmitted(&assignment.assignment_id, &conn).unwrap();

    if unsubmitted.len() != 0 {
        for thing in unsubmitted {
            thing.update_on_time(&assignment, &conn).unwrap();
        }
    }

    let submission = match Submissions::load_by_assignment(&assignment.assignment_id, &conn) {
        Ok(s) => s,
        Err(_) => Vec::<Submissions>::new(),
    };

    let mut submissions = Vec::new();

    for sm in &submission {
        let attachment = attachments::table.filter(attachments::submission_id.eq(&sm.submission_id))
        .load::<Attachment>(&*conn)
        .unwrap();

        submissions.push(SubmissionResponse {
            submission: sm,
            attachment_amount: attachment.len() as i32,
            user: ResponseUser::from(User::find_user(&sm.user_id, &conn).unwrap()),
        });
    }

    let assignment_attachments = attachments::table
        .filter(attachments::assignment_id.eq(&assignment.assignment_id))
        .load::<Attachment>(&*conn)
        .unwrap();

    let assignment_resp = get_attachments(assignment_attachments, &conn);

    Ok(Json(
        json!({"assignment_attachments": assignment_resp, "assignment": assignment, "submissions": submissions}),
    ))
}

#[get("/<class_id>/assignments/teachers/<assignment_id>/submissions/<submission_id>")]
pub fn teachers_submission(
    key: ClassGuard,
    class_id: &str,
    assignment_id: &str,
    submission_id: &str,
    conn: DbConn,
) -> Result<Json<JsonValue>, Status> {
    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::NotFound),
    };

    if user.is_student() {
        return Err(Status::Forbidden);
    }

    let assignment = match Assignment::get_by_id(&assignment_id.to_string(), &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound),
    };

    let submission = match Submissions::find_submission(&submission_id.to_string(), &conn) {
        Ok(s) => s,
        Err(_) => return Err(Status::NotFound),
    };

    let submission_attachments = attachments::table
        .filter(attachments::submission_id.eq(&submission.submission_id))
        .load::<Attachment>(&*conn)
        .unwrap();

    let submission_resp = get_attachments(submission_attachments, &conn);

    let private_comments = PrivateComment::load_by_submission(&submission.submission_id, &conn).unwrap();

    let student = User::find_user(&submission.user_id, &conn).unwrap();

    Ok(Json(
        json!({"submission_attachments": submission_resp, "submission": submission, "student":student, "private_comments": private_comments}),
    ))
}