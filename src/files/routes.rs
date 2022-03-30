use std::{env, io};

use diesel::{Connection, PgConnection};
use rocket::fs::{FileServer, TempFile};
use rocket::fs::relative;

use crate::db::database_url;
use crate::files::models::UploadedFile;
use crate::MEDIA_URL;
use crate::utils::generate_random_id;

pub async fn process_image<'a>(mut image: TempFile<'a>, filename: &String) -> io::Result<String> {
    let url = env::var("SITE_URL").unwrap();
    let file_id = generate_random_id().to_string();
    let current_dir = std::env::current_dir()?;
    let file = format!("{}/{}/{}-{}", current_dir.display(), MEDIA_URL, &file_id, filename);
    let url = format!("{}/{}-{}", format!("http://{}/api/media/img", url), &file_id, filename);
    let db_conn = PgConnection::establish(&database_url()).unwrap();
    UploadedFile::new(&file_id, &filename, &file,&url, &"image".to_string(), &db_conn);
    image.move_copy_to(&file).await?;
    Ok(url)
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket
        .mount("/api/media/img/", FileServer::from(relative!("media")))
}
