use crate::auth;
use crate::users::models::Role;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::auth::SECRET;


#[test]
fn test_1() {
    let a = auth::generate_token("rahmanhakim2435@gmail.com".to_string(), &Role::Student).unwrap();
    let token = decode::<auth::Claims>(&a.as_str(), &DecodingKey::from_secret(SECRET.as_ref()), &Validation::new(Algorithm::HS512)).unwrap();
    println!("{}", token.claims.role)
}