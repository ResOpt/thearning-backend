use std::{env, io};

use diesel::{Connection, PgConnection};
use rocket::form::Form;
use rocket::fs::relative;
use rocket::fs::{FileServer, TempFile};
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket_dyn_templates::handlebars::JsonValue;

use crate::attachments::models::{Attachment, FillableAttachment};
use crate::auth::ApiKey;
use crate::db::database_url;
use crate::errors::ThearningResult;
use crate::files::models::{FileType, UploadType, UploadedFile};
use crate::users::models::User;
use crate::utils::generate_random_id;
use crate::{db, MEDIA_URL};

pub async fn process_image<'a>(
    mut image: TempFile<'a>,
    upload_type: UploadType,
    filename: &String,
) -> ThearningResult<String> {
    let url = env::var("SITE_URL")?;
    let file_id = generate_random_id().to_string();
    let current_dir = std::env::current_dir()?;
    let file = match &upload_type {
        UploadType::ProfilePhoto => {
            format!(
                "{}/{}/profiles/{}-{}",
                current_dir.display(),
                MEDIA_URL,
                &file_id,
                filename
            )
        }
        UploadType::ClassPicture => {
            format!(
                "{}/{}/classes/{}-{}",
                current_dir.display(),
                MEDIA_URL,
                &file_id,
                filename
            )
        }
        UploadType::AssignmentFile => {
            todo!()
        }
    };
    let url = match &upload_type {
        UploadType::ProfilePhoto => {
            format!(
                "{}/{}-{}",
                format!("http://{}/api/media/img/profiles", url),
                &file_id,
                filename
            )
        }
        UploadType::ClassPicture => {
            format!(
                "{}/{}-{}",
                format!("http://{}/api/media/img/classes", url),
                &file_id,
                filename
            )
        }
        UploadType::AssignmentFile => {
            format!(
                "{}/{}-{}",
                format!("http://{}/api/media/attachments", url),
                &file_id,
                filename
            )
        }
    };
    let db_conn = PgConnection::establish(&database_url())?;
    UploadedFile::new(
        &file_id,
        &filename,
        &file,
        &url,
        &"image".to_string(),
        &db_conn,
    )?;
    image.move_copy_to(&file).await?;
    Ok(url)
}

pub async fn process_attachment<'a>(
    mut f: TempFile<'a>,
    name: &str,
    ext: &str,
    ft: FileType,
) -> ThearningResult<UploadedFile> {
    let url = env::var("SITE_URL").unwrap();
    let file_id = format!(
        "{}{}",
        generate_random_id().to_string(),
        generate_random_id().to_string()
    );
    let current_dir = std::env::current_dir()?;
    let file = format!(
        "{}/{}/attachments/{}-{}.{}",
        current_dir.display(),
        MEDIA_URL,
        &file_id,
        name,
        ext
    );

    let url = format!(
        "{}/{}-{}.{}",
        format!("http://{}/api/media/attachments", url),
        &file_id,
        name,
        ext
    );

    let db_conn = PgConnection::establish(&database_url()).unwrap();

    let up = UploadedFile::new(
        &file_id,
        &format!("{}.{}", name, ext),
        &file,
        &url,
        &ft.to_string(),
        &db_conn,
    )
    .unwrap();

    f.move_copy_to(&file).await?;

    Ok(up)
}

#[derive(FromForm)]
struct AttachmentData<'a> {
    file: TempFile<'a>,
    filename: Option<&'a str>,
    assignment_id: Option<&'a str>,
    announcement_id: Option<&'a str>,
    submission_id: Option<&'a str>,
}

#[post("/", data = "<data>")]
async fn upload_file<'a>(
    key: ApiKey,
    data: Form<AttachmentData<'a>>,
    conn: db::DbConn,
) -> Result<Json<JsonValue>, Status> {
    let user = match User::find_user(&key.0, &conn) {
        Ok(u) => u,
        Err(_) => return Err(Status::Unauthorized),
    };

    let data = data.into_inner();

    let file = data.file;

    let file_content_type = &file.content_type();

    let ft = match file_content_type {
        Some(t) => t.to_string(),
        None => return Err(Status::BadRequest),
    };

    let name = match data.filename {
        Some(n) => n,
        None => "",
    };

    let filetype = match FileType::from_str(&ft) {
        Ok(v) => v,
        Err(_) => return Err(Status::BadRequest),
    };

    let uploaded_file = process_attachment(file, name, filetype.ext(), filetype).await;

    let new_file = match uploaded_file {
        Ok(v) => v,
        Err(_) => return Err(Status::BadRequest),
    };

    let cloned_file = new_file.clone();

    let new_attachment = FillableAttachment {
        file_id: Some(cloned_file.file_id),
        link_id: None,
        assignment_id: data.assignment_id,
        announcement_id: data.announcement_id,
        submission_id: data.submission_id,
        uploader: user.user_id.as_str(),
    };

    let attachment = Attachment::create(new_attachment, &conn).unwrap();

    let file = UploadedFile::receive(&attachment.file_id.as_ref().unwrap(), &conn).unwrap();

    Ok(Json(json!({"attachment": &attachment, "file": file})))
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
        .mount("/api/upload", routes![upload_file])
        .mount(
            "/api/media/img/profiles",
            FileServer::from(relative!("media/profiles")),
        )
        .mount(
            "/api/media/img/classes",
            FileServer::from(relative!("media/classes")),
        )
        .mount(
            "/api/media/attachments",
            FileServer::from(relative!("media/attachments")),
        )
}
