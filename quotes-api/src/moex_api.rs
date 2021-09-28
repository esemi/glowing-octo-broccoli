use std::fmt;
use async_compat::CompatExt;
use serde::Deserialize;

use actix_web::http::StatusCode;

const SECURITIES_URL: &str = "https://iss.moex.com/iss/securities/";

macro_rules! TICKER_SEARCH_QUERY {
    () => {
        "{url}{name}.json"
    };
}


#[derive(Debug, Deserialize)]
pub enum MoexError {
    FetchFailed(String),
    DeserializeFailed(String),
    ConnectionFailed,
    InvalidJson,
    EmptyDataSet,
    DataInconsistency,
}

impl std::error::Error for MoexError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl fmt::Display for MoexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchFailed(status) => write!(
                f,
                "fetching the data from iss.moex failed: with status code {}",
                status
            ),
            Self::DeserializeFailed(s) => write!(
                f,
                "deserializing response from iss.moex failed: {}",
                &s
            ),
            Self::ConnectionFailed => write!(f, "connection to iss.moex server failed"),
            Self::InvalidJson => write!(f, "iss.moex return invalid JSON format"),
            Self::EmptyDataSet => write!(f, "iss.moex returned an empty data set"),
            Self::DataInconsistency => write!(f, "iss.moex returned inconsistent data"),
        }
    }
}


#[derive(Default)]
pub struct MoexConnector {
    search_url: &'static str,
}

impl MoexConnector {
    pub fn new() -> MoexConnector {
        MoexConnector {
            search_url: SECURITIES_URL,
        }
    }
}

impl MoexConnector {
    /// Retrieve specification for the given ticker
    /// @see https://iss.moex.com/iss/reference/13
    pub async fn get_ticker_info(&self, ticker: &str) -> Result<MoexTickerSpec, MoexError> {
        let url: String = format!(
            TICKER_SEARCH_QUERY!(),
            url = self.search_url,
            name = ticker
        );
        let response = send_request(&url).await?;
        if len(response)
        println!("{:?}", response);
        Err(MoexError::EmptyDataSet)
    }
}

/// Send request to yahoo! finance server and transform response to JSON value
async fn send_request(url: &str) -> Result<serde_json::Value, MoexError> {
    let resp = match reqwest::get(url).compat().await {
        Ok(s) => s,
        Err(_) => return Err(MoexError::ConnectionFailed)
    };

    match resp.status() {
        StatusCode::OK => resp.json().await.map_err(|_| MoexError::InvalidJson),
        status => Err(MoexError::FetchFailed(format!("Status Code: {}", status))),
    }
}


#[derive(Deserialize, Debug)]
pub struct MoexTickerSpec {
}

impl MoexTickerSpec {
    pub fn from_json(json: serde_json::Value) -> Result<MoexTickerSpec, MoexError> {
        serde_json::from_value(json).map_err(|e| MoexError::DeserializeFailed(e.to_string()))
    }
}


#[cfg(test)]
mod tests {
    use crate::moex_api::MoexConnector;

    #[test]
    fn test_get_ticker_info() {
        let provider = MoexConnector::new();
        let response = tokio_test::block_on(provider.get_ticker_info("SU26209RMFS5")).unwrap();
        assert_eq!(&response.meta.secid, "SU26209RMFS5");
        assert_eq!(&response.meta.boardid, "TQOB");
        assert_eq!(&response.meta.market, "bonds");
        assert_eq!(&response.meta.engine, "stock");
        assert_eq!(&response.meta.currencyid, "RUB");
    }
}