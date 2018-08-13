use crate::chrono::naive::NaiveDateTime;
use crate::schema::{outcomes, prediction_events, transactions, users};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Outcome {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub creation_date: NaiveDateTime,
    pub resolution_date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "outcomes"]
pub struct NewOutcome<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub creation_date: &'a NaiveDateTime,
    pub resolution_date: &'a NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct PredictionEvent {
    pub id: Option<i32>,
    pub by_user: i32,
    pub for_outcome: i32,
    pub prediction: bool,
    pub creation_date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "prediction_events"]
pub struct NewPredictionEvent<'a> {
    pub by_user: &'a i32,
    pub for_outcome: &'a i32,
    pub prediction: &'a bool,
    pub creation_date: &'a NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Transaction {
    pub id: Option<i32>,
    pub date: NaiveDateTime,
    pub amount: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub user_id: &'a i32,
    pub amount: &'a i32,
    pub date: &'a NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub display_name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
}
