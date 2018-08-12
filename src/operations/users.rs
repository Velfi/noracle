use crate::diesel::{self, RunQueryDsl, SqliteConnection};
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

pub fn delete_user() {
    unimplemented!()
}

pub fn get_user() {
    unimplemented!()
}

pub fn get_users(conn: &SqliteConnection) -> Result<Vec<User>, diesel::result::Error> {
    schema::users::table.load::<User>(conn)
}
