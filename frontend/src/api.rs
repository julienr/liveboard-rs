use gloo_net::http::Request;
use shared::datatypes::{Board, CreateBoardRequest};
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

pub async fn fetch_boards() -> Result<Vec<Board>, APIError> {
    let resp = Request::get("/api/boards")
        .send()
        .await?
        .json::<Vec<Board>>()
        .await?;
    Ok(resp)
}

pub async fn create_board(name: String) -> Result<Board, APIError> {
    let req = CreateBoardRequest { name };
    let resp = Request::post("/api/boards")
        .json(&req)?
        .send()
        .await?
        .json::<Board>()
        .await?;
    Ok(resp)
}
