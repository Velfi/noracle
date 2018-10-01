#![feature(nll)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate json;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate serde_json;

pub mod db;
pub mod models;
pub mod operations;
pub mod schema;

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, State,
};
use crate::db::{DbExecutor, GetOutcomes};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use dotenv::dotenv;
use futures::Future;
use std::env::{self, VarError};

struct AppState {
    db: Addr<DbExecutor>,
}

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

    let sys = actix::System::new("noracle");
    let db_pool = init_pool(&env_vars.database_url);

    let addr = SyncArbiter::start(3, move || DbExecutor(db_pool.clone()));

    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            .middleware(middleware::Logger::default())
            .resource("/outcomes", |r| r.method(http::Method::GET).with(index))
    })
    .bind(format!("{}:{}", env_vars.server_ip, env_vars.server_port))
    .unwrap()
    .start();

    let _ = sys.run();
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

type SqlitePool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;

fn init_pool(db_url: &str) -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    Pool::new(manager).expect("Failed to create pool.")
}

fn index((state): (State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetOutcomes)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
