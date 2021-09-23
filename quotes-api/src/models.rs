use strum_macros::{EnumString, ToString};
use serde::{Deserialize};

#[derive(Debug)]
pub struct Ticker {
    pub exchange: Exchange,
    pub ticker: String,
}

#[derive(ToString, EnumString, Deserialize, Debug)]
pub enum Exchange {
    #[strum(serialize = "LON", serialize = "LSE")]
    LSE,

    #[strum(serialize = "NYSEARCA")]
    NYSEARCA,

    NASDAQ,

    #[strum(serialize = "MCX")]
    MOEX,
}

#[derive(Debug)]
pub struct Price {
    currency: String,
    price: f64,
    timestamp: i64,
}

#[derive(ToString)]
enum Currency {
    USD,
    EUR,
    RUB,
}