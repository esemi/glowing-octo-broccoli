use yahoo_finance_api::YahooConnector;
use crate::moex_api::MoexConnector;

pub struct AppState {
    pub yahoo_client: YahooConnector,
    pub moex_client: MoexConnector,
}