use crate::chrono::naive::NaiveDateTime;
use crate::diesel::{self, RunQueryDsl, QueryDsl, SqliteConnection};
use crate::models::{NewPredictionEvent, PredictionEvent};
use crate::schema;

pub fn create_prediction_event(
    conn: &SqliteConnection,
    by_user: &i32,
    for_outcome: &i32,
    prediction: &bool,
    creation_date: &NaiveDateTime,
) -> Result<usize, diesel::result::Error> {
    let new_prediction_event = NewPredictionEvent {
        by_user,
        for_outcome,
        prediction,
        creation_date,
    };

    diesel::insert_into(schema::prediction_events::table)
        .values(&new_prediction_event)
        .execute(conn)
}

pub fn delete_prediction_event(conn: &SqliteConnection, prediction_event_id: &i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(schema::prediction_events::table.find(prediction_event_id))
        .execute(conn)
}

pub fn get_prediction_event(conn: &SqliteConnection, prediction_event_id: &i32) -> Result<Vec<PredictionEvent>, diesel::result::Error> {
    schema::prediction_events::table.find(prediction_event_id).load::<PredictionEvent>(conn)
}

pub fn get_prediction_events(
    conn: &SqliteConnection,
) -> Result<Vec<PredictionEvent>, diesel::result::Error> {
    schema::prediction_events::table.load::<PredictionEvent>(conn)
}
