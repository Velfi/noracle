#![feature(rust_2018_preview)]
#![feature(nll)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
pub mod models;
pub mod operations;
pub mod schema;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use self::operations::outcomes;
use self::operations::prediction_events;
use self::operations::transactions;
use self::operations::users;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use self::models::User;
    use self::schema::users::dsl::*;

    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db_connection = establish_connection();

    let results = users
        .load::<User>(&db_connection)
        .expect("Couldn't load users table");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("Id: {:?}", user.id);
        println!("Username: {}", user.username);
        println!("Display Name: {}", user.display_name);
    }
}
