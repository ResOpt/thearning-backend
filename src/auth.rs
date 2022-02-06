use chrono::{Utc, Local};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

use crate::errors::Errors;
use crate::users::models::Role;
use crate::utils::read_file;
use rocket::http::Status;

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

    let claims = Claims {
        sub: key.to_string(),
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

    match decode::<Claims>
        (key,
         &DecodingKey::from_secret(SECRET.as_ref()),
         &Validation::new(Algorithm::HS512)) {
        Ok(v) => {
            if v.claims.exp < now {
                return Err(Errors::TokenExpired)
            }
            Ok(v.claims.sub)
        },
        Err(_) => Err(Errors::TokenInvalid)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = Errors;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, Errors> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, Errors::TokenInvalid));
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(e) => Outcome::Failure((Status::Unauthorized, e))
        }
    }
}