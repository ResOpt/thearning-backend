use std::ops::Deref;
use std::env;

use crate::errors::ThearningResult;
use chrono::NaiveDate;
use diesel::PgConnection;
use rand::Rng;
use rocket::form;
use rocket::form::{DataField, FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use ayaya::Mailer;
use crate::attachments::models::Attachment;
use crate::comments::models::Commenter;
use crate::files::models::UploadedFile;
use crate::links::models::Link;

use crate::traits::{ClassUser, Manipulable};
use crate::users::models::{ResponseUser, User};

pub fn mailer() -> (Mailer, String) {
    let email = env::var("EMAIL").unwrap();
    let password = env::var("EMAIL_PASSWORD").unwrap();
    let server = env::var("SMTP_SERVER").unwrap();

   (Mailer::build(email, password), server)
}

pub fn generate_random_id() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>().abs()
}

pub fn update<T, U>(table: T, new_data: U, conn: &PgConnection) -> ThearningResult<T>
where
    T: Manipulable<U>,
{
    table.update(new_data, conn)
}

pub fn load_classuser<T>(class_id: &String, conn: &PgConnection) -> Vec<T>
where
    T: ClassUser,
{
    T::load_in_class(class_id, conn).unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct NaiveDateForm(NaiveDate);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for NaiveDateForm {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match NaiveDate::parse_from_str(field.value, "%Y-%m-%d") {
            Ok(res) => Ok(NaiveDateForm(res)),
            Err(_) => Err(form::Error::validation(""))?,
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        unimplemented!()
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;
    fn deref(&self) -> &NaiveDate {
        &self.0
    }
}

pub async fn send_mail(user: User, emails: Vec<String>, message: String, subject: &str) {

    let mail = mailer().0;

    let server = mailer().1;

    let mail = mail.clone().server(server)
        .subject(subject);

    for email in emails {
        let send = mail.clone().to(email.as_str()).message(message.as_str(), "-").clone().send();
        let job = tokio::task::spawn(async move {
            send.await.unwrap()
        });
    }
}

pub fn get_comments<'a, T>(vec: &'a Vec<T>, conn: &PgConnection) -> Vec<UserComment<'a, T>>
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
pub struct UserComment<'a, T: Serialize + Commenter> {
    commenter: ResponseUser,
    comment: &'a T,
}

#[derive(Serialize)]
pub struct AttachmentResponse<'a> {
    attachment: &'a Attachment,
    file: Option<UploadedFile>,
    link: Option<Link>,
}

pub fn get_attachments<'a>(vec: &'a Vec<Attachment>, conn: &PgConnection) -> Vec<AttachmentResponse<'a>> {
    let mut res = Vec::<AttachmentResponse>::new();

    for thing in vec {
        let resp = AttachmentResponse {
            attachment: thing,
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
