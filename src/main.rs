#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use diesel::{Connection, PgConnection};
use std::env;

use dotenv::dotenv;
use rocket::http::{Method, Status};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

use crate::db::database_url;
use assignments::routes as assignment_routes;
use attachments::routes as att_routes;
use classes::routes as class_routes;
use errors::mount as error_routes;
use files::routes as file_routes;
use links::routes as link_routes;
use users::routes as user_routes;

mod classes;
mod users;

mod assignments;
mod attachments;
pub mod auth;
mod comments;
pub mod db;
mod errors;
mod files;
mod links;
mod pagination;
pub mod schema;
mod submissions;
mod tests;
mod traits;
mod utils;

const MEDIA_URL: &str = "media";

#[cfg(debug_assertions)]
fn allowed_origins() -> AllowedOrigins {
    AllowedOrigins::all()
}

#[cfg(not(debug_assertions))]
fn allowed_origins() -> AllowedOrigins {
    let domain = env::var("DOMAIN").unwrap();
    AllowedOrigins::some_exact(&[domain.as_str()])
}

fn make_cors() -> Cors {
    let allowed_origins = allowed_origins();

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Options,
            Method::Delete,
            Method::Patch,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[options("/<_..>")]
fn handle_cors() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    dotenv().ok();

    let mut rocket = rocket::build()
        .manage(db::init_pool())
        .mount("/", routes![handle_cors]);
    rocket = user_routes::mount(rocket);
    rocket = class_routes::mount(rocket);
    // rocket = assignment_routes::mount(rocket);
    rocket = file_routes::mount(rocket);
    rocket = link_routes::mount(rocket);
    rocket = error_routes(rocket).attach(make_cors());
    rocket = att_routes::mount(rocket);
    rocket
}
