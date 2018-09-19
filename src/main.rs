#![feature(nll)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate json;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate warp;

pub mod models;
pub mod operations;
pub mod schema;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use dotenv::dotenv;
use std::env::{self, VarError};
use std::net::{Ipv4Addr, SocketAddrV4};
use warp::Filter;

// struct AppState {
//     db: Addr<DbExecutor>,
// }

#[derive(Clone)]
struct EnvVars {
    database_url: String,
    socket_addr: SocketAddrV4,
    debug_mode: bool,
}

fn main() {
    dotenv().ok();
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let env_vars = load_env_vars().expect("Failed to load all environment variables");

    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(env_vars.socket_addr);
}

fn load_env_vars() -> Result<EnvVars, VarError> {
    let ip_address = parse_string_ip(env::var("SERVER_IP")?);
    let port = env::var("SERVER_PORT")?.parse::<u16>().unwrap();
    let server_address = SocketAddrV4::new(ip_address, port);

    Ok(EnvVars {
        database_url: env::var("DATABASE_URL")?,
        debug_mode: match env::var("DEBUG_MODE")?.as_ref() {
            "True" | "true" => true,
            _ => false,
        },
        socket_addr: server_address,
    })
}

fn parse_string_ip(string_ip: String) -> Ipv4Addr {
    let ip_digits: Vec<u8> = string_ip
        .split(".")
        .map(|str_u8| str_u8.parse::<u8>().unwrap())
        .collect();
    let a = ip_digits[0];
    let b = ip_digits[1];
    let c = ip_digits[2];
    let d = ip_digits[3];
    Ipv4Addr::new(a, b, c, d)
}

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

fn init_pool(db_url: &str) -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    Pool::new(manager).expect("Failed to create pool.")
}
