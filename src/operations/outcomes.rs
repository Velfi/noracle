use chrono::naive::NaiveDateTime;
use crate::models::{NewOutcome, Outcome};
use crate::schema;
use diesel::{self, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};

pub fn create_outcome(
    conn: &SqliteConnection,
    title: String,
    description: Option<String>,
    creation_date: NaiveDateTime,
    resolution_date: NaiveDateTime,
) -> QueryResult<usize> {
    let new_outcome = NewOutcome {
        title,
        description,
        creation_date,
        resolution_date,
    };

    diesel::insert_into(schema::outcomes::table)
        .values(&new_outcome)
        .execute(conn)
}

pub fn delete_outcome(conn: &SqliteConnection, outcome_id: i32) -> QueryResult<usize> {
    diesel::delete(schema::outcomes::table.find(outcome_id)).execute(conn)
}

pub fn get_outcome(conn: &SqliteConnection, outcome_id: i32) -> QueryResult<Vec<Outcome>> {
    schema::outcomes::table
        .find(outcome_id)
        .load::<Outcome>(conn)
}

pub fn get_outcomes(conn: &SqliteConnection) -> QueryResult<Vec<Outcome>> {
    schema::outcomes::table.load::<Outcome>(conn)
}
