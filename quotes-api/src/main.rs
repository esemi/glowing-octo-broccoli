use actix_web::{App, HttpServer, web};

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/healthz", web::get().to(handlers::healthz))
            .route("/quotes/{exchange}/{ticker}", web::get().to(handlers::quotes))
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}