use gloo_net::http::Request;
use shared::datatypes::{Board, CreateBoardRequest, HealthResponse};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct APIError {
    pub message: String,
}
impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.message, f)
    }
}
impl Error for APIError {}

impl From<gloo_net::Error> for APIError {
    fn from(err: gloo_net::Error) -> APIError {
        APIError {
            message: format!("gloo::Error : {:?}", err.to_string()),
        }
    }
}

// https://github.com/yewstack/yew/blob/master/examples/futures/src/main.rs
pub async fn fetch_health() -> Result<HealthResponse, APIError> {
    let resp = Request::get("/api/health")
        .send()
        .await
        .unwrap()
        .json::<HealthResponse>()
        .await
        .unwrap();

    Ok(resp)
}

pub async fn fetch_boards() -> Result<Vec<Board>, APIError> {
    let resp = Request::get("/api/boards")
        .send()
        .await?
        .json::<Vec<Board>>()
        .await?;
    Ok(resp)
}

pub async fn create_board(name: String) -> Result<Board, APIError> {
    let req = CreateBoardRequest { name: name };
    let resp = Request::post("/api/boards")
        .json(&req)?
        .send()
        .await?
        .json::<Board>()
        .await?;
    Ok(resp)
}