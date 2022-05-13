use chrono::Local;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket_dyn_templates::handlebars::JsonValue;
use emotional_scraper::data::{OtherData, Scrapable, WikipediaData, YoutubeData};
use emotional_scraper::{get_raw_data, Url, UrlData};
use serde::{Deserialize, Serialize};

use crate::attachments::models::{Attachment, FillableAttachment};
use crate::auth::ApiKey;
use crate::db;
use crate::links::models::Link;
use crate::traits::Manipulable;
use crate::users::models::User;
use crate::utils::generate_random_id;

#[derive(Deserialize, Serialize)]
struct AttachmentData<'a> {
    url: &'a str,
    assignment_id: Option<&'a str>,
    announcement_id: Option<&'a str>,
    submission_id: Option<&'a str>,
}

#[post("/", data = "<data>")]
async fn handle_link<'a>(
    key: ApiKey,
    data: Json<AttachmentData<'a>>,
    conn: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    let data = data.into_inner();

    let user = match User::find_user(&key.0, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::NotFound),
    };

    let link_id = format!("{}{}", generate_random_id(), generate_random_id());

    let raw_data = get_raw_data(&data.url).await.unwrap();

    let url_data = match Url::from(data.url) {
        Url::Youtube => UrlData::from(YoutubeData { raw_data }),
        Url::Wikipedia => UrlData::from(WikipediaData { raw_data }),
        Url::Other => UrlData::from(OtherData { raw_data }),
    };

    let link = Link {
        id: link_id,
        title: url_data.title,
        description: url_data.content,
        thumbnail: url_data.thumbnail,
        url: Some(data.url.to_string()),
        created_at: Local::now().naive_local(),
    };

    let cloned_link = link.clone();

    let create_link = match Link::create(cloned_link, &conn) {
        Ok(l) => l,
        Err(_) => return Err(Status::BadRequest),
    };

    let new_attachment = FillableAttachment {
        file_id: None,
        link_id: Some(link.id),
        assignment_id: data.assignment_id,
        announcement_id: data.announcement_id,
        submission_id: data.submission_id,
        uploader: user.user_id.as_str(),
    };

    let attachment = match Attachment::create(new_attachment, &conn) {
        Ok(v) => v,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    Ok(Json(json!({"link":create_link, "attachment": attachment})))
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount("/api/links", routes![handle_link])
}
