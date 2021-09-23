extern crate strum;

use std::str::FromStr;

use actix_web::{Error, HttpResponse, Result};
use strum_macros::{EnumString, ToString};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Healthz {
    status: String,
}

#[derive(Serialize, Deserialize)]
struct Quote {
    status: String,
}

struct Ticker {
    exchange: String,
    ticker: String,
}

struct Price {
    currency: String,
    price: f64,
}

#[derive(ToString, EnumString)]
enum Exchange {
    #[strum(serialize = "LON", serialize = "LSE")]
    LSE,

    #[strum(serialize = "NYSEARCA")]
    NYSEARCA,

    NASDAQ,

    #[strum(serialize = "MCX")]
    MOEX,
}

#[derive(ToString)]
enum Currency {
    USD,
    EUR,
    RUB,
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .body("Usage: GET /quotes/LSE/EIMI")
}

pub async fn healthz() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Healthz {
        status: String::from("Ok"),
        // todo redis check
    }))
}

pub async fn quotes(exchange: String, ticker: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Quote {
        status: String::from("im teapot"),
    }))
    // // parse exchange
    // let exchange_parsed = match Exchange::from_str(&exchange) {
    //     Ok(s) => s,
    //     Err(_) => NotFound(String::from("Exchange not found")),
    // };
    //
    // println!("{:?}", exchange_parsed);
    //
    //
    // // todo fetch from short-cache
    // // todo fetch from exchange-client and save to cache(x2)
    // // todo fetch from long-cache
    // // todo fetch from logging
    // //
    // // let ticker_parsed = Ticker {
    // //     exchange,
    // //     ticker,
    // // }
    // //
    // let quote = Quote {
    //     status: String::from("im teapot"),
    // };
    // HttpResponse::Ok().json(quote);
}