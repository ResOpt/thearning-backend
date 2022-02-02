#![feature(decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::response::content::Json;

use crate::users::routes;

mod users;
mod classes;

pub mod auth;
pub mod schema;
pub mod db;
mod utils;
mod errors;
mod test;

fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::init_pool());
    rocket = routes::mount(rocket);
    rocket.launch();
}
