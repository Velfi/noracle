#![feature(nll)]
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate json;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod models;
pub mod db;
pub mod debug;
pub mod operations;
pub mod routes;
pub mod schema;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, State};
use std::env::{self, VarError};
use std::ops::Deref;

// struct AppState {
//     db: Addr<DbExecutor>,
// }

#[derive(Clone)]
struct EnvVars {
    database_url: String,
    server_ip: String,
    server_port: String,
    debug_mode: bool,
}

fn main() {
    dotenv().ok();
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let env_vars = load_env_vars().expect("Failed to load all environment variables");

    let server = rocket::ignite()
        .manage(init_pool(&env_vars.database_url))
        .mount(
            "/api",
            routes![
                index,
                routes::outcomes::delete,
                routes::outcomes::get_all,
                routes::outcomes::get_by_id,
                routes::outcomes::post,
            ],
        );

    if env_vars.debug_mode {
        server.mount(
            "/debug",
            routes![
                routes::debug::initialize_db_with_test_data,
                routes::debug::print_all_data,
            ],
        );
    } else {
        server.launch();
    }
}

fn load_env_vars() -> Result<EnvVars, VarError> {
    Ok(EnvVars {
        database_url: env::var("DATABASE_URL")?,
        server_ip: env::var("SERVER_IP")?,
        server_port: env::var("SERVER_PORT")?,
        debug_mode: match env::var("DEBUG_MODE")?.as_ref() {
            "True" | "true" => true,
            _ => false,
        },
    })
}

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

fn init_pool(db_url: &str) -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    Pool::new(manager).expect("Failed to create pool.")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[get("/")]
fn index() -> &'static str {
    "Eventually, this will be a website."
}
