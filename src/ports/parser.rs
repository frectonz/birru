use crate::domain::{CurrencyParseError, DailyForexRate};

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

impl From<CurrencyParseError> for ParserError {
    fn from(_: CurrencyParseError) -> Self {
        ParserError::CurrencyParseFailed
    }
}

pub trait Parser {
    fn parse_daily_forex_rate(data: String) -> DailyForexRate;
}
