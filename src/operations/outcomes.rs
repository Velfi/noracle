use crate::chrono::naive::NaiveDateTime;
use crate::diesel::{self, RunQueryDsl, SqliteConnection};
use crate::models::{NewOutcome, Outcome};
use crate::schema;

pub fn create_outcome(
    conn: &SqliteConnection,
    title: &str,
    description: Option<&str>,
    creation_date: &NaiveDateTime,
    resolution_date: &NaiveDateTime,
) -> Result<usize, diesel::result::Error> {
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

pub fn delete_outcome() {
    unimplemented!()
}

pub fn get_outcome() {
    unimplemented!()
}

pub fn get_outcomes(conn: &SqliteConnection) -> Result<Vec<Outcome>, diesel::result::Error> {
    schema::outcomes::table.load::<Outcome>(conn)
}