use crate::operations::outcomes::*;
use crate::models;
use crate::DbConn;
use diesel::QueryResult;
use rocket_contrib::Json;

#[get("/outcomes")]
pub fn get_all(conn: DbConn) -> QueryResult<Json<Vec<models::Outcome>>> {
    get_outcomes(&conn).map(Json)
}

#[get("/outcomes/<id>")]
pub fn get_by_id(id: i32, conn: DbConn) -> QueryResult<Json<Vec<models::Outcome>>> {
    get_outcome(&conn, id).map(Json)
}

#[post(
    "/outcomes",
    format = "application/json",
    data = "<req_body>"
)]
pub fn post(req_body: Json<models::NewOutcome>, conn: DbConn) -> QueryResult<Json> {
    create_outcome(
        &conn,
        req_body.title.clone(),
        req_body.description.clone(),
        req_body.creation_date,
        req_body.resolution_date,
    ).map(|outcome_id| Json(json!({"status": "ok", "id": outcome_id})))
}

#[delete("/outcomes/<id>")]
pub fn delete(id: i32, conn: DbConn) -> QueryResult<Json> {
    delete_outcome(&conn, id).map(|_| Json(json!({"status": "ok"})))
}
