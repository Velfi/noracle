use crate::diesel::{self, RunQueryDsl, QueryDsl, SqliteConnection};
use crate::models::{NewUser, User};
use crate::schema;

pub fn create_user(
    conn: &SqliteConnection,
    username: &str,
    display_name: &str,
) -> Result<usize, diesel::result::Error> {
    let new_user = NewUser {
        username,
        display_name,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)
}

pub fn delete_user(conn: &SqliteConnection, user_id: &i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(schema::users::table.find(user_id))
        .execute(conn)
}

pub fn get_user(conn: &SqliteConnection, user_id: &i32) -> Result<Vec<User>, diesel::result::Error> {
    schema::users::table.find(user_id).load::<User>(conn)
}

pub fn get_users(conn: &SqliteConnection) -> Result<Vec<User>, diesel::result::Error> {
    schema::users::table.load::<User>(conn)
}
