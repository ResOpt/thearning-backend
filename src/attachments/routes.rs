use rocket;
use rocket::http::Status;

use crate::auth::ApiKey;
use crate::db;
use crate::attachments::models::Attachment;

#[delete("/<attachment_id>")]
pub fn delete_attachment(key: ApiKey, attachment_id: String, conn: db::DbConn) -> Result<Status, Status> {
    let attachment = match Attachment::find(&attachment_id, &conn) {
        Ok(a) => a,
        Err(_) => return Err(Status::NotFound)
    };
    
    attachment.delete(&conn);

    Ok(Status::Ok)
}

pub fn mount(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    rocket.mount("/api/attachments", routes![delete_attachment])
}