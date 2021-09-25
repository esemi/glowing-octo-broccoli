extern crate strum;

use std::str::FromStr;

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use yahoo_finance_api::{YahooConnector, YahooError};

use super::models::{Currency, Exchange, Price, Ticker};

#[derive(Serialize, Deserialize)]
struct HealthzOutput {
    status: String,
}

#[derive(Serialize, Deserialize)]
struct QuoteOutput {
    exchange: String,
    ticker: String,
    price: f64,
    currency: String,
    timestamp: u64,
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
            // todo yahoo check
            // todo moex check
        })
}

pub async fn quotes(request: web::Path<QuoteInput>) -> HttpResponse {
    println!("Get quotes {:?}", request);

    // parse exchange
    let exchange: Exchange = match Exchange::from_str(&request.exchange) {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest()
            .body(format!("Exchange '{}' not found", request.exchange))
    };
    let ticker_info = Ticker {
        exchange,
        ticker: request.ticker.to_uppercase(),
    };
    println!("Parsed ticker {:?}", &ticker_info);

    let price = match search_ticker_price(&ticker_info).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest()
            .body(format!("Ticker '{}' was not found", ticker_info.ticker))
    };
    println!("Ticker price is {:?}", price);

    HttpResponse::Ok().json(QuoteOutput {
        exchange: ticker_info.exchange.to_string(),
        ticker: ticker_info.ticker,

        price: price.price,
        currency: price.currency.to_string(),
        timestamp: price.timestamp,
    })
}

async fn search_ticker_price(ticker: &Ticker) -> Result<Price, String> {
    let price = match ticker.exchange {
        // Exchange::MOEX => {
        //
        // },
        _ => _yahoo_fetch(&ticker.ticker).await
    };

    match price {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

async fn _yahoo_fetch(ticker: &String) -> Result<Price, YahooError> {
    // todo move to appstate
    let provider = YahooConnector::new();
    let yahoo_response = provider
        .get_latest_quotes(&ticker, "1d").await?;
    let meta = &yahoo_response.chart.result[0].meta;
    println!("Ticker quote meta is {:?}", meta);

    let quote = yahoo_response.last_quote()?;
    println!("Ticker last quote is {:?}", quote);

    Ok(Price {
        exchange_name: Exchange::from_str(&meta.exchange_name)
            .expect("Unexpected exchange name"),
        currency: Currency::from_str(&meta.currency)
            .expect("Unexpected currency"),
        price: quote.close,
        timestamp: quote.timestamp,
    })
}