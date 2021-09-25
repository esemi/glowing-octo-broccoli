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
    pub currency: Currency,
    pub price: f64,
    pub timestamp: u64,
}

#[derive(ToString, EnumString, Debug)]
pub enum Currency {
    USD,
    EUR,
    RUB,
}