extern crate strum;

use std::str::FromStr;

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use yahoo_finance_api::{YahooConnector, YahooError};

use super::models::{Exchange, Price, Ticker, Currency};

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
            .body(format!("Exchange {} not found", request.exchange))
    };
    let ticker_info = Ticker {
        exchange,
        ticker: request.ticker.to_uppercase(),
    };
    println!("Parsed ticker {:?}", &ticker_info);

    let price = match search_ticker_price(&ticker_info).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest()
            .body(format!("Ticker {} was not found", ticker_info.ticker))
    };
    println!("Ticker price is {:?}", price);

    HttpResponse::Ok().json(QuoteOutput {
        status: String::from("im teapot"),
    })
}

async fn search_ticker_price(ticker: &Ticker) -> Result<Price, String> {
    // todo add short-time cache here

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
    let provider = YahooConnector::new();
    let yahoo_response = provider
        .get_latest_quotes(&ticker, "1d")
        .await?;
    let meta = &yahoo_response.chart.result[0].meta;
    println!("Ticker quote meta is {:?}", meta);

    let quote = yahoo_response.last_quote()?;
    println!("Ticker last quote is {:?}", quote);

    Ok(Price {
        currency: Currency::from_str(&meta.currency).unwrap(),
        price: quote.close,
        timestamp: quote.timestamp,
    })
}