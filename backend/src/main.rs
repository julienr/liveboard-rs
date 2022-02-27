use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
mod rest_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").service(rest_handlers::health))
            .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
