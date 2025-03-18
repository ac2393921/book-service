// use crate::domain::value_objects::book_id::BookId;
// use domain::value_objects::book_id::BookId;

use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize_server().await
}

async fn initialize_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .route("/book", web::get().to(book))
        // .app_data(web::Data::new(order_service.clone()))
        // .service(api::place_order)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn book() -> impl Responder {
    HttpResponse::Ok().body("book service")
}
