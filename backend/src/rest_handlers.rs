use super::db;
use super::db::{get_boards, State};
use actix_web::{error, get, post, web, Responder, Result};
use shared::datatypes::CreateBoardRequest;

#[post("/boards")]
async fn create_board(
    db_state: web::Data<State>,
    data: web::Json<CreateBoardRequest>,
) -> Result<impl Responder> {
    let client = db_state.as_ref().pool.clone().get().await.unwrap();
    let board = db::create_board(&client, data.name.clone()).await.unwrap();
    println!("New board {:?}", board);
    Ok(web::Json(board))
}

#[get("/boards")]
async fn list_boards(db_state: web::Data<State>) -> Result<impl Responder> {
    let client = db_state.as_ref().pool.clone().get().await.unwrap();
    let boards = get_boards(&client).await.unwrap();
    Ok(web::Json(boards))
}
