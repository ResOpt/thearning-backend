use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_dyn_templates::handlebars::JsonValue;
use crate::announcements::models::Announcement;
use crate::announcements::models::FillableAnnouncement;
use crate::auth::ClassGuard;
use crate::{db, Status};
use crate::attachments::models::Attachment;
use crate::comments::models::Comment;
use crate::traits::{ClassUser, Manipulable};
use crate::users::models::{Student, User};
use crate::utils::{get_attachments, get_comments, send_mail};

#[get("/<class_id>/announcements")]
pub fn get_announcements(key: ClassGuard, class_id: &str, conn: db::DbConn) -> Json<Vec<Announcement>> {
    let announcements = Announcement::load_in_class(&conn, class_id).unwrap();
    Json(announcements)
}

#[get("/<class_id>/announcements/<announcement_id>")]
pub fn get_announcement(key: ClassGuard, class_id: &str, announcement_id: &str, conn: db::DbConn) -> Json<JsonValue> {
    let announcement = Announcement::find_announcement(&conn, announcement_id).unwrap();

    let comments = Comment::load_by_announcement(announcement_id, &conn).unwrap();

    let comment_response = get_comments(&comments, &conn);

    let attachments = Attachment::load_by_announcement_id(&announcement_id.to_string(), &conn).unwrap();

    let attachment_response = get_attachments(&attachments, &conn);

    Json(json!({
        "announcement": announcement,
        "comments": comment_response,
        "attachments": attachment_response
    }))
}

#[post("/<class_id>/announcements")]
pub fn draft_announcement(key: ClassGuard, class_id: &str, conn: db::DbConn) -> Json<JsonValue> {
    let default = Announcement::default();

    default.draft(&conn);

    Json(json!({"announcement_id": default.announcement_id}))
}

#[patch("/<class_id>/announcements", data = "<announcement>")]
pub async fn update_announcement(key: ClassGuard, class_id: &str, announcement: Json<FillableAnnouncement>, conn: db::DbConn) -> Result<Json<Announcement>, Status> {
    let data = announcement.into_inner();

    let creator = match User::find_user(&key.0, &conn)
    {
        Ok(user) => user,
        Err(_) => return Err(Status::NotFound)
    };

    let announcement = Announcement::find_announcement(&conn, &data.announcement_id).unwrap();

    let update = announcement.update(data, &conn).unwrap();

    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>New Announcement!</title>
</head>
<body>
    <div style="display: block; align-items: center;">
        <h2 style="font-family: Arial, Helvetica, sans-serif;">New Announcement from {}: {}</h2>
        <br>
        <h4 style="font-family: Arial, Helvetica, sans-serif;">{}</h4>
    </div>
</body>
</html>"#, &creator.fullname, &update.announcement_name.as_ref().unwrap_or(&" ".to_string()), &update.body.as_ref().unwrap_or(&" ".to_string()));

    let students = Student::load_in_class(&class_id.to_string(), &conn).unwrap();

    let mut emails = Vec::new();

    for i in &students {
        emails.push(User::find_user(&i.user_id, &conn).unwrap().email)
    }

    send_mail(creator, emails, html, "New Announcement").await;

    Ok(Json(update))
}

#[delete("/<class_id>/announcements/<announcement_id>")]
pub fn delete_announcement(key: ClassGuard, class_id: &str, announcement_id: &str, conn: db::DbConn) -> Result<Json<JsonValue>, Status> {
    let announcement = Announcement::find_announcement(&conn, &announcement_id).unwrap();

    announcement.delete(&conn).unwrap();

    Ok(Json(json!({"announcement_id": announcement_id})))
}