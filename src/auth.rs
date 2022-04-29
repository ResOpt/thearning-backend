use std::env;
use chrono::{Duration, Local};
use diesel::{Connection, PgConnection};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorKind, JWTCError, ThearningResult};
use crate::db::database_url;
use crate::users::models::{Role, User};
use crate::users::utils::is_email;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub role: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct ApiKey(pub String);

pub fn generate_token(key: &String, role: &Role) -> ThearningResult<String> {

    let dt = Local::now();

    let now = dt.timestamp_nanos() as usize;

    let exp = now + (Duration::days(7).num_nanoseconds().unwrap() as usize);

    let mut sub = key.clone();
    let db_conn = PgConnection::establish(&database_url())?;

    if is_email(key) {
        sub = User::get_id_from_email(key, &db_conn)?;
    }

    let claims = Claims {
        sub,
        iat: now,
        role: role.to_string(),
        exp,
    };

    let header = Header::new(Algorithm::HS512);
    Ok(encode(&header, &claims, &EncodingKey::from_secret(env::var("SECRETS").unwrap().as_bytes()))?)
}

pub fn read_token(key: &str) -> ThearningResult<String> {

    let dt = Local::now();

    let now = dt.timestamp_nanos() as usize;

    match decode::<Claims>(
        key,
        &DecodingKey::from_secret(env::var("SECRETS").unwrap().as_bytes().as_ref()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(v) => {

            if now > v.claims.exp {
                return Err(ErrorKind::JWTCreationError(JWTCError::TokenExpired))
            }
            Ok(v.claims.sub)
        }
        Err(e) => Err(ErrorKind::from(e)),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ErrorKind;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiKey, ErrorKind> {
        let keys = match request.headers().get("Authorization").collect::<Vec<_>>().first() {
            Some(k) => {
                k.split("Bearer").map(|i| i.trim()).collect::<String>()
            }
            None => return request::Outcome::Failure((Status::BadRequest, ErrorKind::InvalidValue)),
        };

        match read_token(keys.as_str()) {
            Ok(claim) => request::Outcome::Success(ApiKey(claim)),
            Err(e) => request::Outcome::Failure((Status::Unauthorized, e)),
        }
    }
}
