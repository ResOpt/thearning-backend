#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::content::Json;

use classes::routes as class_routes;
use users::routes as user_routes;
use assignments::routes as assignment_routes;

mod users;
mod classes;

pub mod auth;
pub mod schema;
pub mod db;
mod utils;
mod errors;
mod test;
mod assignments;

fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::init_pool());
    rocket = user_routes::mount(rocket);
    rocket = class_routes::mount(rocket);
    rocket = assignment_routes::mount(rocket);
    rocket.launch();
}
