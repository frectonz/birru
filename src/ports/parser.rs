use crate::domain::DailyForexRate;
use async_trait::async_trait;

#[derive(Debug)]
pub enum ParserError {
    EmptyDocument,
    NoBuyingValue,
    NoSellingValue,
    NoCurrencyValue,
    CurrencyParseFailed,
    BuyingPriceParseFailed,
    SellingPriceParseFailed,
}

#[async_trait]
pub trait Parser {
    async fn parse_daily_forex_rate(data: String) -> DailyForexRate;
}
