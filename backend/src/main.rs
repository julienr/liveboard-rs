use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
mod rest_handlers;
mod ws_handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let ws_state = web::Data::new(ws_handlers::make_state());
    let db_state = web::Data::new(db::make_state());

    HttpServer::new(move || {
        App::new()
            .app_data(ws_state.clone())
            .app_data(db_state.clone())
            .service(web::scope("/api").service(rest_handlers::health))
            .route("/ws/", web::get().to(ws_handlers::index))
            .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
