use chrono::Local;
use diesel::{Connection, PgConnection};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

use crate::db::database_url;
use crate::errors::Errors;
use crate::users::models::{Role, User};
use crate::users::utils::is_email;

pub(crate) const SECRET: &[u8] = include_bytes!("../secrets");
const ONE_WEEK: usize = 60 * 60 * 24 * 7;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    pub(crate) iat: usize,
    pub(crate) role: String,
    pub(crate) exp: usize,
}

#[derive(Clone)]
pub struct ApiKey(pub String);

pub fn generate_token(key: &String, role: &Role) -> Result<String, Errors> {
    let now = (Local::now().timestamp_nanos() / 1_000_000_00) as usize;

    let mut sub = key.clone();
    let db_conn = match PgConnection::establish(&database_url()) {
        Ok(c) => c,
        Err(_) => return Err(Errors::FailedToCreateJWT),
    };

    if is_email(key) {
        sub = match User::get_id_from_email(key, &db_conn) {
            Ok(ok) => ok,
            Err(_) => return Err(Errors::FailedToCreateJWT),
        };
    }

    let claims = Claims {
        sub,
        iat: now,
        role: role.to_string(),
        exp: now + ONE_WEEK,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(SECRET))
        .map_err(|_| Errors::FailedToCreateJWT)
}

pub fn read_token(key: &str) -> Result<String, Errors> {
    let now = (Local::now().timestamp_nanos() / 1_000_000_00) as usize;

    match decode::<Claims>(
        key,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(v) => {
            Ok(v.claims.sub)
        }
        Err(_) => Err(Errors::TokenInvalid),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = Errors;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiKey, Errors> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return request::Outcome::Failure((Status::BadRequest, Errors::TokenInvalid));
        }
        match read_token(keys[0]) {
            Ok(claim) => request::Outcome::Success(ApiKey(claim)),
            Err(e) => request::Outcome::Failure((Status::Unauthorized, e)),
        }
    }
}
