use std::{env, io};

use diesel::{Connection, PgConnection};
use rocket::fs::{FileServer, TempFile};
use rocket::fs::relative;

use crate::db::database_url;
use crate::files::models::{FileType, UploadedFile, UploadType};
use crate::MEDIA_URL;
use crate::utils::generate_random_id;

pub async fn process_image<'a>(mut image: TempFile<'a>, upload_type: UploadType, filename: &String) -> io::Result<String> {
    let url = env::var("SITE_URL").unwrap();
    let file_id = generate_random_id().to_string();
    let current_dir = std::env::current_dir()?;
    let file = match &upload_type {
        UploadType::ProfilePhoto => {
            format!("{}/{}/profiles/{}-{}", current_dir.display(), MEDIA_URL, &file_id, filename)
        }
        UploadType::ClassPicture => {
            format!("{}/{}/classes/{}-{}", current_dir.display(), MEDIA_URL, &file_id, filename)
        }
        UploadType::AssignmentFile => {
            todo!()
        }
    };
    let url = match &upload_type {
        UploadType::ProfilePhoto => {
            format!("{}/{}-{}", format!("http://{}/api/media/img/profiles", url), &file_id, filename)
        }
        UploadType::ClassPicture => {
            format!("{}/{}-{}", format!("http://{}/api/media/img/classes", url), &file_id, filename)
        }
        UploadType::AssignmentFile => {
            todo!()
        }
    };
    let db_conn = PgConnection::establish(&database_url()).unwrap();
    UploadedFile::new(&file_id, &filename, &file,&url, &"image".to_string(), &db_conn);
    image.move_copy_to(&file).await?;
    Ok(url)
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
        .mount("/api/media/img/profiles", FileServer::from(relative!("media/profiles")))
        .mount("/api/media/img/classes", FileServer::from(relative!("media/classes")))
}
