use chrono::{NaiveDate, Utc};
use crate::operations::{self, outcomes, prediction_events, transactions, users};
use crate::schema;
use crate::DbConn;
use diesel::{QueryResult, RunQueryDsl, SqliteConnection};

#[post("/reset")]
pub fn initialize_db_with_test_data(conn: DbConn) -> QueryResult<()> {
    clear_all_data_in_db(&conn)?;

    // Id Some(1)
    users::create_user(&conn, "zhessler".to_string(), "Zelda Hessler".to_string())?;
    // Id Some(2)
    users::create_user(&conn, "jgardner".to_string(), "Scooter".to_string())?;
    // Id Some(3)
    users::create_user(&conn, "jbinion".to_string(), "Sarah Binion".to_string())?;

    let title = "The Nerdery Will Raise $100,000 for Extra Life.".to_string();
    let description = "The Nerdery raises lots of money for EL every year. Will it raise at least $100,000 this year?".to_string();
    let date_time_now = Utc::now().naive_utc();
    let extra_life_end_date = NaiveDate::from_ymd(2018, 11, 4).and_hms(0, 0, 0);

    // Id: Some(1)
    outcomes::create_outcome(
        &conn,
        title,
        Some(description),
        date_time_now,
        extra_life_end_date,
    )?;

    prediction_events::create_prediction_event(&conn, 1, 1, false, date_time_now)?;

    prediction_events::create_prediction_event(&conn, 2, 1, true, date_time_now)?;

    prediction_events::create_prediction_event(&conn, 3, 1, true, date_time_now)?;

    transactions::create_transaction(&conn, 1, -18, date_time_now)?;

    transactions::create_transaction(&conn, 1, 35, date_time_now)?;

    transactions::create_transaction(&conn, 3, 100, date_time_now)?;

    Ok(())
}

#[post("/print")]
pub fn print_all_data(conn: DbConn) -> QueryResult<()> {
    let outcome_results = operations::outcomes::get_outcomes(&conn)?;
    let prediction_event_results = operations::prediction_events::get_prediction_events(&conn)?;
    let transaction_results = operations::transactions::get_transactions(&conn)?;
    let user_results = operations::users::get_users(&conn)?;

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

    Ok(())
}

fn clear_all_data_in_db(conn: &SqliteConnection) -> QueryResult<()> {
    diesel::delete(schema::outcomes::table).execute(conn)?;
    diesel::delete(schema::prediction_events::table).execute(conn)?;
    diesel::delete(schema::transactions::table).execute(conn)?;
    diesel::delete(schema::users::table).execute(conn)?;

    Ok(())
}
