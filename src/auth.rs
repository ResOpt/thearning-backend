use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

use crate::utils::read_file;
use chrono::Utc;
use crate::users::models::Role;
use crate::errors::Errors;

pub(crate) const SECRET: &[u8] = include_bytes!("../secrets");
const ONE_WEEK: usize = 60 * 60 * 24 * 7;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    pub(crate) iat: usize,
    pub(crate) role: String,
    pub(crate) exp: usize,
}

pub struct ApiKey(pub String);

pub fn generate_token(key: &String, role: &Role) -> Result<String, Errors> {

    let now = (Utc::now().timestamp_nanos() / 1_000_000_00) as usize;

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

pub fn read_token(key: &str) -> Result<String, String> {
    match decode::<Claims>
        (key,
         &DecodingKey::from_secret(SECRET.as_ref()),
         &Validation::new(Algorithm::HS512)) {
        Ok(v) => Ok(v.claims.sub),
        Err(_) => Err("Token invalid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}