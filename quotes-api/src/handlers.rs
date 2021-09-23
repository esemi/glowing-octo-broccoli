extern crate strum;

use std::str::FromStr;

use actix_web::{Error, HttpResponse, web};
use serde::{Deserialize, Serialize};
use yahoo_finance_api as yahoo;

use super::models::{Exchange, Price, Ticker};

#[derive(Serialize, Deserialize)]
struct HealthzOutput {
    status: String,
}

#[derive(Serialize, Deserialize)]
struct QuoteOutput {
    status: String,
}

#[derive(Deserialize, Debug)]
pub struct QuoteInput {
    pub exchange: String,
    pub ticker: String,
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .body("Usage: GET /quotes/LSE/EIMI")
}

pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok()
        .json(HealthzOutput {
            status: String::from("Ok"),
            // todo redis check
        })
}

pub async fn quotes(request: web::Path<QuoteInput>) -> HttpResponse {
    println!("Get quotes {:?}", request);

    // parse exchange
    let exchange: Exchange = match Exchange::from_str(&request.exchange) {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest()
            .body(String::from("Exchange not found"))
    };
    let ticker_info = Ticker {
        exchange,
        ticker: request.ticker.to_uppercase(),
    };
    println!("Parsed ticker {:?}", &ticker_info);

    let price = match fetch_ticker_price(&ticker_info).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest()
            .body(String::from("Ticker was not found"))
    };
    println!("Ticker price is {}", price);

    HttpResponse::Ok().json(QuoteOutput {
        status: String::from("im teapot"),
    })
}

async fn fetch_ticker_price(ticker: &Ticker) -> Result<f64, Error> {
    let provider = yahoo::YahooConnector::new();
    let yahoo_response = provider.get_latest_quotes(&ticker.ticker, "1d").await.unwrap();
    let quote = yahoo_response.last_quote().unwrap();
    println!("At quote price of {:?} was {}", ticker, quote.close);
    Ok(quote.close)

    // // todo fetch from exchange-client and save to cache(x2)
    // // todo fetch from long-cache
}