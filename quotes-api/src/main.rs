use actix_web::{App, HttpServer, web};
use yahoo_finance_api::YahooConnector;

use crate::moex_api::MoexConnector;

mod handlers;
mod models;
mod state;
mod moex_api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(state::AppState {
                yahoo_client: YahooConnector::new(),
                moex_client: MoexConnector::new(),
            })
            .route("/", web::get().to(handlers::index))
            .route("/healthz", web::get().to(handlers::healthz))
            .route("/quotes/{exchange}/{ticker}", web::get().to(handlers::quotes))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}