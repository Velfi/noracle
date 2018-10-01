use super::{models, operations};
use actix::{Actor, Handler, Message, SyncContext};
use actix_web::Error;
use chrono::NaiveDateTime;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct InitializeDbWithTestData;

impl Message for InitializeDbWithTestData {
    type Result = Result<u8, Error>;
}

impl Handler<InitializeDbWithTestData> for DbExecutor {
    type Result = Result<u8, Error>;

    fn handle(&mut self, _msg: InitializeDbWithTestData, _ctx: &mut Self::Context) -> Self::Result {
        let conn: &SqliteConnection = &self.0.get().unwrap();
        crate::debug::initialize_db_with_test_data(conn)?;

        Ok(0)
    }
}

pub struct GetOutcomes;

impl Message for GetOutcomes {
    type Result = Result<Vec<models::Outcome>, Error>;
}

impl Handler<GetOutcomes> for DbExecutor {
    type Result = Result<Vec<models::Outcome>, Error>;

    fn handle(&mut self, _msg: GetOutcomes, _ctx: &mut Self::Context) -> Self::Result {
        let conn: &SqliteConnection = &self.0.get().unwrap();

        let outcomes = operations::outcomes::get_outcomes(conn).unwrap();

        Ok(outcomes)
    }
}

pub struct GetOutcome {
    pub id: i32,
}

impl Message for GetOutcome {
    type Result = Result<models::Outcome, Error>;
}

impl Handler<GetOutcome> for DbExecutor {
    type Result = Result<models::Outcome, Error>;

    fn handle(&mut self, msg: GetOutcome, _ctx: &mut Self::Context) -> Self::Result {
        let conn: &SqliteConnection = &self.0.get().unwrap();

        let outcome = operations::outcomes::get_outcome(conn, &msg.id).unwrap();

        // Technically, there could be more than 1 outcome with the same ID. This should never happen.
        Ok(outcome[0].clone())
    }
}

pub struct CreateOutcome {
    pub title: String,
    pub description: Option<String>,
    pub creation_date: NaiveDateTime,
    pub resolution_date: NaiveDateTime,
}

impl Message for CreateOutcome {
    type Result = Result<i32, Error>;
}

impl Handler<CreateOutcome> for DbExecutor {
    type Result = Result<i32, Error>;

    fn handle(&mut self, msg: CreateOutcome, _ctx: &mut Self::Context) -> Self::Result {
        let conn: &SqliteConnection = &self.0.get().unwrap();

        operations::outcomes::create_outcome(
            conn,
            &msg.title,
            msg.description.as_ref().map(String::as_ref),
            &msg.creation_date,
            &msg.resolution_date,
        ).unwrap();

        Ok(0)
    }
}

pub struct DeleteOutcome {
    pub id: i32,
}

impl Message for DeleteOutcome {
    type Result = Result<(), Error>;
}

impl Handler<DeleteOutcome> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteOutcome, _ctx: &mut Self::Context) -> Self::Result {
        let conn: &SqliteConnection = &self.0.get().unwrap();

        operations::outcomes::delete_outcome(conn, &msg.id).unwrap();

        Ok(())
    }
}
