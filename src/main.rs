#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rocket::http::Method;
use rocket::response::content::Json;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Cors,
    CorsOptions, Error,
};

use assignments::routes as assignment_routes;
use classes::routes as class_routes;
use errors::mount as error_routes;
use users::routes as user_routes;

mod users;
mod classes;

pub mod auth;
pub mod schema;
pub mod db;
mod utils;
mod errors;
mod tests;
mod assignments;
mod submissions;
mod files;

#[cfg(debug_assertions)]
fn allowed_origins() -> AllowedOrigins {
    AllowedOrigins::all()
}

#[cfg(not(debug_assertions))]
fn allowed_origins() -> AllowedOrigins {
    AllowedOrigins::some_exact(&[
        "http://www.domain.tld",
        "http://127.0.0.1:5000"
    ])
}


fn make_cors() -> Cors {
    let allowed_origins = allowed_origins();

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("error while building CORS")
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let mut rocket = rocket::build()
        .manage(db::init_pool());
    rocket = user_routes::mount(rocket);
    rocket = class_routes::mount(rocket);
    rocket = assignment_routes::mount(rocket);
    rocket = error_routes(rocket).attach(make_cors());
    rocket
}
