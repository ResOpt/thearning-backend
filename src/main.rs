#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
#[macro_use]
extern crate rocket_contrib;

use rocket::http::Method;

use rocket::response::content::Json;

use classes::routes as class_routes;
use users::routes as user_routes;
use assignments::routes as assignment_routes;
use errors::mount as error_routes;

mod users;
mod classes;

pub mod auth;
pub mod schema;
pub mod db;
mod utils;
mod errors;
mod test;
mod assignments;
mod submissions;

use rocket_cors::{
AllowedHeaders, AllowedOrigins, Error,
Cors, CorsOptions
};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://127.0.0.1:5000",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authentication",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type",
            "Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("error while building CORS")
}

fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::init_pool()).attach(make_cors());
    rocket = user_routes::mount(rocket);
    rocket = class_routes::mount(rocket);
    rocket = assignment_routes::mount(rocket);
    rocket = error_routes(rocket);
    rocket.launch();
}
