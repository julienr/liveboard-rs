use super::db;
use super::db::{get_boards, get_shapes, State};
use actix_web::{get, post, web, Responder, Result};
use shared::datatypes as data;
use shared::datatypes::CreateBoardRequest;

#[post("/boards")]
async fn create_board(
    db_state: web::Data<State>,
    data: web::Json<CreateBoardRequest>,
) -> Result<impl Responder> {
    let client = db_state.as_ref().pool.clone().get().await.unwrap();
    let board: data::Board = db::create_board(&client, data.name.clone())
        .await
        .unwrap()
        .into();
    println!("New board {:?}", board);
    Ok(web::Json(board))
}

#[get("/boards")]
async fn list_boards(db_state: web::Data<State>) -> Result<impl Responder> {
    let client = db_state.as_ref().pool.clone().get().await.unwrap();
    let boards: Vec<data::Board> = get_boards(&client)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.into())
        .collect();
    Ok(web::Json(boards))
}

#[get("/board/{id}")]
async fn get_board(db_state: web::Data<State>, path: web::Path<(u32,)>) -> Result<impl Responder> {
    let client = db_state.as_ref().pool.clone().get().await.unwrap();
    let shapes: Vec<data::Shape> = get_shapes(&client, path.0)
        .await
        .unwrap()
        .into_iter()
        .map(|s| s.into())
        .collect();
    Ok(web::Json(shapes))
}
