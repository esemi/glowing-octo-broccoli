extern crate strum;

use std::str::FromStr;

use actix_web::{HttpResponse, web, web::Data};
use serde::{Deserialize, Serialize};
use yahoo_finance_api::{YahooConnector, YahooError};

use super::models::{Currency, Exchange, Price, Ticker};
use super::moex_api::{MoexConnector, MoexError};
use super::state::AppState;

const STATUS_OK: &'static str = "OK";
const STATUS_FAIL: &'static str = "NOOK";

#[derive(Serialize, Deserialize)]
struct HealthzOutput {
    status: String,
    yahoo: String,
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

pub async fn healthz(state: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .json(HealthzOutput {
            status: STATUS_OK.to_string(),
            yahoo: if state.yahoo_client.get_latest_quotes("AAPL", "1m").await.is_ok() {
                STATUS_OK.to_string()
            } else {
                STATUS_FAIL.to_string()
            },
            // todo moex check
        })
}

pub async fn quotes(request: web::Path<QuoteInput>, state: Data<AppState>) -> HttpResponse {
    println!("Get quotes {:?}", request);

    // parse exchange
    let exchange = Exchange::from_str(&request.exchange);
    let exchange: Exchange = match exchange {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest()
            .body(format!("Exchange '{}' not found", request.exchange))
    };
    let ticker_info = Ticker {
        exchange,
        ticker: request.ticker.to_uppercase(),
    };
    println!("Parsed ticker {:?}", &ticker_info);

    let price = match search_ticker_price(&ticker_info, state).await {
        Ok(s) => s,
        Err(e) => return HttpResponse::BadRequest()
            .body(format!("Ticker '{}' was not found {}", ticker_info.ticker, e))
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

async fn search_ticker_price(ticker: &Ticker, state: Data<AppState>) -> Result<Price, String> {
    let price = match ticker.exchange {
        // Exchange::MOEX => {_moex_fetch(&ticker.ticker, &state.moex_client).await},
        _ => _yahoo_fetch(&ticker.ticker, &state.yahoo_client).await
    };

    match price {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}


async fn _moex_fetch(ticker: &String, client: &MoexConnector) -> Result<Price, MoexError> {
    let moex_response = client.get_ticker_info(&ticker).await?;

    let meta = &moex_response.meta;
    println!("Ticker quote meta is {:?}", meta);

    let quote = &moex_response.quote?;
    println!("Ticker last quote is {:?}", quote);

    // get securities specification
    // https://iss.moex.com/iss/securities?q=SU26209RMFS5 and select primary board
    // select market by specification.group
    // get quotes
    // https://iss.moex.com/iss/engines/stock/markets/bonds/boards/TQOB/securities/SU26209RMFS5.xml
    // adjust price by nominal for bonds

    Ok(Price {
        exchange_name: Exchange::MOEX,
        currency: Currency::from_str(&meta.currency)
            .expect("Unexpected currency"),
        price: quote.close,
        timestamp: quote.timestamp,
    })
}


async fn _yahoo_fetch(ticker: &String, client: &YahooConnector) -> Result<Price, YahooError> {
    let yahoo_response = client.get_latest_quotes(&ticker, "1d").await?;
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