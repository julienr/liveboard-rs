use actix_web::{get, web, Responder, Result};
use shared::datatypes::HealthResponse;

#[get("/health")]
async fn health() -> Result<impl Responder> {
    let obj = HealthResponse {
        value1: String::from("this is a value"),
    };
    Ok(web::Json(obj))
}
