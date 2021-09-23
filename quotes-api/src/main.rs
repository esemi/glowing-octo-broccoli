use actix_web::{App, HttpServer, middleware, web};

mod handlers;
mod models;
mod exchange;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .wrap(middleware::NormalizePath::default())
            .route("/", web::get().to(handlers::index))
            .route("/healthz", web::get().to(handlers::healthz))
            .route("/quotes/{exchange}/{ticker}", web::get().to(handlers::quotes))
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}