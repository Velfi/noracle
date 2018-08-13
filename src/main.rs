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

use self::operations::outcomes;
use self::operations::prediction_events;
use self::operations::transactions;
use self::operations::users;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db_connection = establish_connection();

    initialize_db_with_test_data(&db_connection);
    print_all_data(&db_connection);
}

fn initialize_db_with_test_data(conn: &SqliteConnection) {
    clear_all_data_in_db(conn);

    // Id Some(1)
    users::create_user(conn, "zhessler", "Zelda Hessler");
    // Id Some(2)
    users::create_user(conn, "jgardner", "Scooter");
    // Id Some(3)
    users::create_user(conn, "jbinion", "Sarah Binion");

    let title = "The Nerdery Will Raise $100,000 for Extra Life.";
    let description = "The Nerdery raises lots of money for EL every year. Will it raise at least $100,000 this year?";
    let date_time_now = Utc::now().naive_utc();
    let extra_life_end_date = NaiveDate::from_ymd(2018, 11, 4).and_hms(0, 0, 0);

    // Id: Some(1)
    outcomes::create_outcome(
        conn,
        &title,
        Some(&description),
        &date_time_now,
        &extra_life_end_date,
    );

    prediction_events::create_prediction_event(&conn, &1, &1, &false, &date_time_now);

    prediction_events::create_prediction_event(&conn, &2, &1, &true, &date_time_now);

    prediction_events::create_prediction_event(&conn, &3, &1, &true, &date_time_now);

    transactions::create_transaction(&conn, &1, &(-18), &date_time_now);

    transactions::create_transaction(&conn, &1, &35, &date_time_now);

    transactions::create_transaction(&conn, &3, &100, &date_time_now);
}

fn print_all_data(conn: &SqliteConnection) {
    let outcome_results = operations::outcomes::get_outcomes(conn).unwrap();
    let prediction_event_results =
        operations::prediction_events::get_prediction_events(conn).unwrap();
    let transaction_results = operations::transactions::get_transactions(conn).unwrap();
    let user_results = operations::users::get_users(conn).unwrap();

    println!("Total Outcomes: {}", outcome_results.len());
    for outcome in outcome_results {
        println!("{}", outcome);
    }

    println!("Total Predictions: {}", prediction_event_results.len());
    for prediction_event in prediction_event_results {
        println!("{}", prediction_event);
    }

    println!("Total Transactions: {}", transaction_results.len());
    for transaction in transaction_results {
        println!("{}", transaction);
    }

    println!("Total Users: {}", user_results.len());
    for user in user_results {
        println!("{}", user);
    }
}

fn clear_all_data_in_db(conn: &SqliteConnection) {
    diesel::delete(schema::outcomes::table).execute(conn).unwrap();
    diesel::delete(schema::prediction_events::table).execute(conn).unwrap();
    diesel::delete(schema::transactions::table).execute(conn).unwrap();
    diesel::delete(schema::users::table).execute(conn).unwrap();
}
