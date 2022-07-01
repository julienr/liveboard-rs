use actix_web::{error, get, web, Responder, Result};
use shared::datatypes::HealthResponse;
use super::db::{State};

#[get("/health")]
async fn health(db_state: web::Data<State>,) -> Result<impl Responder> {
    // Test db connection
    let conn = db_state.as_ref().pool.clone().get().await;
    if conn.is_err() {
        return Err(error::ErrorInternalServerError(conn.err().unwrap()));
    }

    let obj = HealthResponse {
        value1: String::from("this is a value"),
    };
    Ok(web::Json(obj))
}
