use crate::chrono::naive::NaiveDateTime;
use crate::diesel::{self, RunQueryDsl, QueryDsl, SqliteConnection};
use crate::models::{NewTransaction, Transaction};
use crate::schema;

pub fn create_transaction(
    conn: &SqliteConnection,
    user_id: &i32,
    amount: &i32,
    date: &NaiveDateTime,
) -> Result<usize, diesel::result::Error> {
    let new_transaction = NewTransaction {
        user_id,
        amount,
        date,
    };

    diesel::insert_into(schema::transactions::table)
        .values(&new_transaction)
        .execute(conn)
}

pub fn delete_transaction(
    conn: &SqliteConnection,
    transaction_id: &i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(schema::transactions::table.find(transaction_id))
        .execute(conn)
}

pub fn get_transaction(conn: &SqliteConnection, transaction_id: &i32) -> Result<Vec<Transaction>, diesel::result::Error> {
    schema::transactions::table.find(transaction_id).load::<Transaction>(conn)
}

pub fn get_transactions(
    conn: &SqliteConnection,
) -> Result<Vec<Transaction>, diesel::result::Error> {
    schema::transactions::table.load::<Transaction>(conn)
}
