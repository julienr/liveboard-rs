use actix_web::{get, web, App, HttpServer, Result, Responder, middleware::Logger};
use actix_files as fs;
use shared::datatypes::{HealthResponse};


#[get("/health")]
async fn health() -> Result<impl Responder> {
    //HttpResponse::Ok().body("Hello")
    let obj = HealthResponse{
        value1: String::from("this is a value")
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .service(health))
            .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}