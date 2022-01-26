#![feature(decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::response::content::Json;

mod users;
mod classes;

pub mod auth;
pub mod schema;
pub mod db;

fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::init_pool());
    rocket.launch();
}
