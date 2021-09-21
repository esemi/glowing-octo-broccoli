#[macro_use]
extern crate rocket;
extern crate strum;

use std::str::FromStr;

use rocket::response::status::NotFound;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use strum_macros::{EnumString, ToString};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Quote {
    status: String
}

struct Ticker {
    exchange: String,
    ticker: String,
}

struct Price {
    currency: String,
    price: f64,
}

#[derive(ToString, EnumString, Debug)]
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

#[get("/")]
fn index() -> &'static str {
    "todo api manual"
}

#[get("/healthz")]
fn healthz() -> &'static str {
    "todo impl"
}

#[get("/<exchange>/<ticker>")]
fn quote_get(exchange: String, ticker: String) -> Result<Json<Quote>, NotFound<String>> {
    // parse exchange
    let exchange_parsed = match Exchange::from_str(&exchange) {
        Ok(s) => s,
        Err(_) => NotFound(String::from("Exchange not found")),
    };

    println!("{:?}", exchange_parsed);


    // todo fetch from short-cache
    // todo fetch from exchange-client and save to cache(x2)
    // todo fetch from long-cache
    // todo fetch from logging
    //
    // let ticker_parsed = Ticker {
    //     exchange,
    //     ticker,
    // }
    //
    Json(Quote {
        status: String::from("im teapot"),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, healthz])
        .mount("/quotes", routes![quote_get])
}