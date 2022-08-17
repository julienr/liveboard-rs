use actix_files as fs;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{middleware::Logger, web, App, HttpServer};
mod db;
mod rest_handlers;
mod ws_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let ws_state = web::Data::new(ws_handlers::make_state());
    let db_state = web::Data::new(db::make_state());

    HttpServer::new(move || {
        App::new()
            .app_data(ws_state.clone())
            .app_data(db_state.clone())
            .service(
                web::scope("/api")
                    .service(rest_handlers::create_board)
                    .service(rest_handlers::list_boards)
                    .service(rest_handlers::get_board)
                    .service(ws_handlers::ws_for_board),
            )
            .service(
                fs::Files::new("/", "../frontend/dist")
                    .index_file("index.html")
                    .default_handler(|req: ServiceRequest| {
                        let (http_req, _payload) = req.into_parts();
                        async {
                            let response = fs::NamedFile::open("../frontend/dist/index.html")?
                                .into_response(&http_req);
                            Ok(ServiceResponse::new(http_req, response))
                        }
                    }),
            )
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
